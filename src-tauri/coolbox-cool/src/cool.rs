use std::cmp::Ordering;

use color_eyre::eyre::eyre;
use crossbeam::channel::{Receiver, Sender};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::error::CoolError;
use crate::result::CoolResult;
use crate::state::CoolState;
use crate::tasks::Tasks;
use crate::{TaskEvent, COOL_LIST};

static INSTALLING: Lazy<DashMap<String, Receiver<()>>> = Lazy::new(DashMap::new);
static UNINSTALLING: Lazy<DashMap<String, Receiver<()>>> = Lazy::new(DashMap::new);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cool {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub install_tasks: Tasks,
    pub uninstall_tasks: Tasks,
    pub check_tasks: Tasks,
}

impl Cool {
    pub fn new(
        name: String,
        description: String,
        dependencies: Vec<String>,
        install_tasks: Tasks,
        uninstall_tasks: Tasks,
        check_tasks: Tasks,
    ) -> Self {
        Self {
            name,
            description,
            dependencies,
            install_tasks,
            uninstall_tasks,
            check_tasks,
        }
    }

    pub fn install(&self, event_sender: &Option<Sender<TaskEvent>>) -> CoolResult<(), CoolError> {
        let name = self.name.clone();
        match self.check() {
            CoolState::Ready => {
                info!("{} is ready, to be install", name)
            }
            CoolState::Installing => {
                info!("waiting for {} to be installed", name);
                return Ok(());
            }
            CoolState::Installed => {
                info!("{} is installed", name);
                return Ok(());
            }
            CoolState::Uninstalling => {
                info!("waiting for {} to be uninstalled, then to be install", name);
                UNINSTALLING
                    .get(&name)
                    .unwrap()
                    .recv()
                    .map_err(|e| CoolError::UnknownError {
                        cool_name: name.clone(),
                        error: format!("{:?}", e),
                    })?;
            }
        }

        self.install_dependencies(event_sender)?;

        let (lock_sender, lock_receiver) = crossbeam::channel::bounded(1);
        INSTALLING.insert(name.clone(), lock_receiver);
        let (tx, rx) = crossbeam::channel::bounded(1);
        rayon::scope(|s| {
            s.spawn(|_| {
                let result = self.install_tasks.execute(Box::new(|i, task, message| {
                    if let Some(sender) = event_sender.as_ref() {
                        sender
                            .send(TaskEvent {
                                cool_name: name.clone(),
                                task_name: task.name().to_string(),
                                task_index: i,
                                message,
                            })
                            .unwrap();
                    }
                }));
                lock_sender.send(()).unwrap();
                INSTALLING.remove(&name);
                tx.send(result).unwrap();
            })
        });
        rx.recv()
            .unwrap()
            .map_err(|(tn, ti, error)| CoolError::from(name.clone(), tn, ti, error))
    }

    pub fn uninstall(&self, event_sender: &Option<Sender<TaskEvent>>) -> CoolResult<(), CoolError> {
        let name = self.name.clone();
        match self.check() {
            CoolState::Ready => {
                info!("{} is ready, to be install", name);
                return Ok(());
            }
            CoolState::Installing => {
                info!("waiting for {} to be installed, then to be uninstall", name);
                INSTALLING
                    .get(&name)
                    .unwrap()
                    .recv()
                    .map_err(|e| CoolError::UnknownError {
                        cool_name: name.clone(),
                        error: format!("{:?}", e),
                    })?;
            }
            CoolState::Installed => {
                info!("{} is installed, to be uninstall", name);
            }
            CoolState::Uninstalling => {
                info!("{} is uninstalling", name);
                return Ok(());
            }
        }

        let (lock_sender, lock_receiver) = crossbeam::channel::bounded(1);
        UNINSTALLING.insert(name.clone(), lock_receiver);
        let (tx, rx) = crossbeam::channel::bounded(1);
        rayon::scope(|s| {
            s.spawn(|_| {
                let result = self.uninstall_tasks.execute(Box::new(|i, task, message| {
                    if let Some(sender) = event_sender.as_ref() {
                        sender
                            .send(TaskEvent {
                                cool_name: name.clone(),
                                task_name: task.name().to_string(),
                                task_index: i,
                                message,
                            })
                            .unwrap();
                    }
                }));
                lock_sender.send(()).unwrap();
                UNINSTALLING.remove(&name);
                tx.send(result).unwrap();
            });
        });
        rx.recv()
            .unwrap()
            .map_err(|(tn, ti, error)| CoolError::from(name.clone(), tn, ti, error))
    }

    fn install_dependencies(
        &self,
        sender: &Option<Sender<TaskEvent>>,
    ) -> CoolResult<(), CoolError> {
        self.dependencies.par_iter().try_for_each(|d| {
            info!("installing dependency [{}] for [{}]", d, self.name);
            let cool = COOL_LIST.get(d).ok_or(CoolError::NotFoundCool {
                cool_name: d.clone(),
            })?;
            cool.lock().unwrap().install(sender)
        })?;
        Ok(())
    }

    pub fn check(&self) -> CoolState {
        let check = self.check_tasks.execute(Box::new(|i, task, message| {
            info!("[{}]{}: {}", i, task, message);
        }));
        if check.is_ok() {
            CoolState::Installed
        } else {
            let mut state = CoolState::Ready;
            if INSTALLING.contains_key(&self.name) {
                state = CoolState::Installing;
            }
            if UNINSTALLING.contains_key(&self.name) {
                state = CoolState::Uninstalling;
            }
            state
        }
    }
}

impl Ord for Cool {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Cool {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use crate::result::CoolResult;
    use crate::shell::{MacOSSudo, Shell};
    use crate::tasks::{Task, Tasks, WhichTask};
    use crate::{init_backtrace, Cool, COOL_LIST};

    #[test]
    fn test_cool() -> CoolResult<()> {
        init_backtrace();
        let mut brew_cool = Cool {
            name: "homebrew".to_string(),
            description: "适用于macOS的包管理器。它使您能够从命令行安装和更新软件包，从而使您的Mac保持最新状态，而无需使用App Store。".to_string(),
            dependencies: vec![],
            install_tasks: Tasks(vec![
                Task::download("https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh", "{{TEMP_DIR}}/homebrew/install.sh"),
                Task::command("{{TEMP_DIR}}/homebrew/install.sh", None::<Vec<&str>>, Some(vec![("NONINTERACTIVE", "1")]), Shell::MacOSSudo(MacOSSudo)),
            ]),
            uninstall_tasks: Tasks(vec![
                Task::download("https://raw.githubusercontent.com/Homebrew/install/HEAD/uninstall.sh", "{{TEMP_DIR}}/homebrew/uninstall.sh"),
                Task::command("{{TEMP_DIR}}/homebrew/uninstall.sh", None::<Vec<&str>>, Some(vec![("NONINTERACTIVE", "1")]), Shell::MacOSSudo(MacOSSudo)),
            ]),
            check_tasks: Tasks(vec![
                Task::WhichTask(WhichTask::new("brew".to_string()))
            ]),
        };
        let string = toml::to_string(&brew_cool)?;
        println!("{}", string);
        let cool = toml::from_str::<Cool>(&string)?;
        println!("{:#?}", cool);

        brew_cool.check_tasks.0.clear();
        let string = toml::to_string(&brew_cool)?;
        println!("==========\n{}", string);
        toml::from_str::<Cool>(&string)?;
        Ok(())
    }

    #[test]
    fn test_install_curl() -> CoolResult<()> {
        init_backtrace();
        let curl = COOL_LIST.get("curl").unwrap();
        curl.lock().unwrap().install(&None)?;
        Ok(())
    }
}
