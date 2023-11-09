use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::error::ExecutableError;
use crate::installer::{Installable, Installer};
use crate::result::ExecutableResult;
use crate::shell::ShellResult;
use crate::tasks::{redirect_output, Executable, ExecutableSender};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InstallTask {
    pub name: String,
    pub args: Option<Vec<String>>,
    pub installer: Installer,
}

impl InstallTask {
    pub fn new(name: String, args: Option<Vec<String>>, installer: Installer) -> Self {
        Self {
            name,
            args,
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

impl Executable for InstallTask {
    fn _run(&self, sender: &ExecutableSender) -> ExecutableResult {
        let ShellResult {
            input: _input,
            output,
            error,
        } = self
            .installer
            .install(
                &self.name,
                self.args
                    .as_ref()
                    .map(|args| args.iter().map(AsRef::as_ref).collect::<Vec<_>>())
                    .as_deref(),
            )
            .map_err(ExecutableError::ShellError)?;
        redirect_output(sender, &output, &error);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::init_backtrace;
    use crate::installer::{Brew, Installable, Installer};
    use crate::result::CoolResult;
    use crate::tasks::{spawn_task, InstallTask};
    use tracing::info;

    #[test]
    fn install_bat() -> CoolResult<()> {
        init_backtrace();

        #[cfg(target_os = "macos")]
        let installer = Installer::Brew(Brew);
        #[cfg(target_os = "linux")]
        let installer = Installer::Apt(Apt);

        if installer.check_available("bat", None)? {
            installer.uninstall("bat", None)?;
        }

        #[cfg(target_os = "macos")]
        let install = InstallTask::new("bat".to_string(), None, Installer::Brew(Brew));
        #[cfg(target_os = "linux")]
        let mut install = InstallTask::new("bat".to_string(), None, Installer::Apt(Apt));

        spawn_task(install, |msg| {
            info!("{}", msg);
        })?;

        installer.uninstall("bat", None)?;

        Ok(())
    }
}
