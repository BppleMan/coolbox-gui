use schemars::JsonSchema;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::error::ExecutableError;
use crate::installer::{Installable, Installer};
use crate::result::ExecutableResult;
use crate::tasks::{Executable, MessageSender};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct InstallTask {
    pub name: String,
    #[serde(deserialize_with = "crate::template_args", default)]
    pub args: Option<Vec<String>>,
    #[serde(deserialize_with = "crate::template_envs", default)]
    pub envs: Option<Vec<(String, String)>>,
    pub installer: Installer,
}

impl InstallTask {
    pub fn new(
        name: String,
        args: Option<Vec<String>>,
        envs: Option<Vec<(String, String)>>,
        installer: Installer,
    ) -> Self {
        Self {
            name,
            args,
            envs,
            installer,
        }
    }
}

impl Display for InstallTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.installer {
            Installer::Apt(_) => write!(f, "sudo apt-get install -y "),
            Installer::Brew(_) => write!(f, "brew install "),
            Installer::Cargo(_) => write!(f, "cargo install "),
            Installer::Yun(_) => write!(f, "sudo yum install -y "),
            Installer::Dnf(_) => write!(f, "sudo dnf install -y "),
            Installer::Rpm(_) => write!(f, "sudo rpm -i "),
        }?;
        if let Some(args) = self.args.as_ref() {
            for arg in args {
                write!(f, "{} ", arg)?;
            }
        }
        write!(f, " {}", self.name)
    }
}

impl<'a> Executable<'a> for InstallTask {
    fn execute(&self, mut send: Box<MessageSender<'a>>) -> ExecutableResult {
        let (tx1, rx1) = crossbeam::channel::unbounded();
        let (tx2, rx2) = crossbeam::channel::bounded(1);
        let installer = self.installer.clone();
        let name = self.name.clone();
        let args = self.args.clone();
        let envs = self.envs.clone();
        rayon::spawn(move || {
            let result = installer
                .install(
                    &name,
                    args.as_ref()
                        .map(|args| args.iter().map(AsRef::as_ref).collect::<Vec<_>>())
                        .as_deref(),
                    envs.as_ref()
                        .map(|envs| {
                            envs.iter()
                                .map(|(k, v)| (k.as_str(), v.as_str()))
                                .collect::<Vec<_>>()
                        })
                        .as_deref(),
                    tx1,
                )
                .map_err(ExecutableError::ShellError);
            tx2.send(result).unwrap();
        });
        while let Ok(msg) = rx1.recv() {
            send(msg);
        }
        rx2.recv().unwrap()
    }
}

#[cfg(test)]
mod test {
    use tracing::info;

    use crate::init_backtrace;
    use crate::installer::{Brew, Installable, Installer};
    use crate::result::CoolResult;
    use crate::tasks::{spawn_task, InstallTask};

    #[test]
    fn install_bat() -> CoolResult<()> {
        init_backtrace();

        #[cfg(target_os = "macos")]
        let installer = Installer::Brew(Brew);
        #[cfg(target_os = "linux")]
        let installer = Installer::Apt(Apt);

        let (sender, _receiver) = crossbeam::channel::unbounded();
        if installer.check_available("bat", None, None)? {
            installer.uninstall("bat", None, None, sender.clone())?;
        }

        #[cfg(target_os = "macos")]
        let install = InstallTask::new("bat".to_string(), None, None, Installer::Brew(Brew));
        #[cfg(target_os = "linux")]
        let mut install = InstallTask::new("bat".to_string(), None, Installer::Apt(Apt));

        spawn_task(install, |msg| {
            info!("{}", msg);
        })?;

        installer.uninstall("bat", None, None, sender)?;

        Ok(())
    }
}
