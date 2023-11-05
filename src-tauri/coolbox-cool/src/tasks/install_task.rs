use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use coolbox_macros::State;

use crate::installer::{Installable, Installer};
use crate::result::CoolResult;
use crate::shell::ShellResult;
use crate::tasks::{Executable, ExecutableState};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, State)]
pub struct InstallTask {
    pub name: String,
    pub args: Option<Vec<String>>,
    pub installer: Installer,

    #[serde(skip)]
    state: ExecutableState,
    #[serde(skip)]
    outputs: Vec<String>,
    #[serde(skip)]
    errors: Vec<String>,
}

impl InstallTask {
    pub fn new(name: String, args: Option<Vec<String>>, installer: Installer) -> Self {
        Self {
            name,
            args,
            installer,
            state: ExecutableState::NotStarted,
            outputs: vec![],
            errors: vec![],
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
    fn _run(&mut self) -> CoolResult<()> {
        let initial_result: CoolResult<()> = Err(eyre!("No attempts made"));

        (0..5).fold(initial_result, |acc, _| {
            if let Err(_) = acc {
                let ShellResult {
                    input: _input,
                    output,
                    error,
                } = self.installer.install(
                    &self.name,
                    self.args
                        .as_ref()
                        .map(|args| args.iter().map(AsRef::as_ref).collect::<Vec<_>>())
                        .as_deref(),
                )?;

                rayon::scope(|s| {
                    s.spawn(|_| {
                        while let Ok(r) = output.recv() {
                            self.outputs.push(r);
                        }
                    });
                    s.spawn(|_| {
                        while let Ok(r) = error.recv() {
                            self.errors.push(r);
                        }
                    });
                });
            }
            Ok(())
        })
    }
}

#[cfg(test)]
mod test {
    use crate::init_backtrace;
    use crate::installer::{Brew, Installable, Installer};
    use crate::result::CoolResult;
    use crate::tasks::{Executable, InstallTask};

    #[test]
    fn install_bat() -> CoolResult<()> {
        init_backtrace();

        #[cfg(target_os = "macos")]
        let mut installer = Installer::Brew(Brew);
        #[cfg(target_os = "linux")]
        let installer = Installer::Apt(Apt);

        if installer.check_available("bat", None)? {
            installer.uninstall("bat", None)?;
        }

        #[cfg(target_os = "macos")]
        let mut install = InstallTask::new("bat".to_string(), None, Installer::Brew(Brew));
        #[cfg(target_os = "linux")]
        let mut install = InstallTask::new("bat".to_string(), None, Installer::Apt(Apt));

        install.execute()?;

        installer.uninstall("bat", None)?;

        Ok(())
    }
}
