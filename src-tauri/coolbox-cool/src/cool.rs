use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::thread;

use crate::{SafeCool, COOL_LIST};
use color_eyre::eyre::eyre;
use color_eyre::Report;
use crossbeam::channel::Receiver;
use dashmap::mapref::one::Ref;
use lazy_static::lazy_static;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::error::{InstallError, TransformError};
use crate::result::CoolResult;
use crate::state::CoolState;
use crate::tasks::Tasks;

lazy_static! {
    static ref INSTALLING: Arc<RwLock<HashMap<String, Receiver<()>>>> =
        Arc::new(RwLock::new(HashMap::new()));
    static ref UNINSTALLING: Arc<RwLock<HashMap<String, Receiver<()>>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

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

    pub fn install(&mut self) -> CoolResult<Vec<Vec<String>>> {
        let name = self.name.clone();
        if INSTALLING.read().unwrap().contains_key(&name) {
            return Err(Report::new(InstallError::AlreadyInstalling(name)));
        }

        info!("installing {}", name);

        if UNINSTALLING.read().unwrap().contains_key(&name) {
            info!("waiting for {} to be uninstalled", name);
            UNINSTALLING.read().unwrap()[&name].recv()?;
        }

        self.install_dependencies()?;

        let mut tasks = self.install_tasks.clone();
        let handle = thread::spawn(move || tasks.execute());

        let (sender, receiver) = crossbeam::channel::bounded(1);

        INSTALLING
            .write()
            .unwrap()
            .insert(name.clone(), receiver.clone());

        let result = handle.join().unwrap()?;
        sender.send(())?;
        INSTALLING.write().unwrap().remove(&name);

        Ok(result)
    }

    pub fn uninstall(&mut self) -> CoolResult<Vec<Vec<String>>> {
        let name = self.name.clone();
        if UNINSTALLING.read().unwrap().contains_key(&name) {
            return Err(Report::new(InstallError::AlreadyUninstalling(name)));
        }

        info!("uninstalling {}", name);

        if INSTALLING.read().unwrap().contains_key(&name) {
            info!("waiting for {} to be installed", name);
            INSTALLING.read().unwrap()[&name].recv()?;
        }

        let mut tasks = self.uninstall_tasks.clone();
        let handle = thread::spawn(move || tasks.execute());

        let (sender, receiver) = crossbeam::channel::bounded(1);

        UNINSTALLING
            .write()
            .unwrap()
            .insert(name.clone(), receiver.clone());

        let result = handle.join().unwrap()?;
        sender.send(())?;
        UNINSTALLING.write().unwrap().remove(&name);

        Ok(result)
    }

    fn install_dependencies(&self) -> CoolResult<Vec<Vec<String>>> {
        let results = self
            .dependencies
            .par_iter()
            .map(|d| {
                if let Some(cool) = COOL_LIST.get(d) {
                    Ok(cool.write().unwrap().install()?)
                } else {
                    Err(eyre!("{} not found", d))
                }
            })
            .try_fold(Vec::new, |mut results, result| match result {
                Ok(result) => {
                    results.extend(result);
                    Ok(results)
                }
                Err(e) => Err(e),
            })
            .try_reduce(Vec::new, |mut results, result| {
                results.extend(result);
                Ok(results)
            })?;
        Ok(results)
    }

    pub fn check(&mut self) -> CoolResult<CoolState> {
        self.check_tasks.execute()
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
    use crate::{init_backtrace, Cool};

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
}
