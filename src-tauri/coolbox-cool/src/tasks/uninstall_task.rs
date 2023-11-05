use crate::installer::{Installable, Installer};
use crate::result::CoolResult;
use crate::shell::ShellResult;
use crate::tasks::{Executable, ExecutableState};
use color_eyre::eyre::eyre;
use coolbox_macros::State;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, State)]
pub struct UninstallTask {
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

impl UninstallTask {
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

impl Display for UninstallTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.installer {
            Installer::Apt(_) => write!(f, "sudo apt-get remove -y --purge"),
            Installer::Brew(_) => write!(f, "brew uninstall "),
            Installer::Cargo(_) => write!(f, "cargo uninstall "),
            Installer::Yun(_) => write!(f, "sudo yum remove "),
            Installer::Dnf(_) => write!(f, "sudo dnf remove "),
            Installer::Rpm(_) => write!(f, "sudo rpm -e "),
        }?;
        if let Some(args) = self.args.as_ref() {
            for arg in args {
                write!(f, "{} ", arg)?;
            }
        }
        write!(f, " {}", self.name)?;

        if let Installer::Apt(_) = &self.installer {
            write!(f, " && sudo apt-get autoremove")
        } else {
            Ok(())
        }
    }
}

impl Executable for UninstallTask {
    fn _run(&mut self) -> CoolResult<()> {
        let initial_result: CoolResult<()> = Err(eyre!("No attempts made"));

        (0..5).fold(initial_result, |acc, _| {
            if let Err(_) = acc {
                let ShellResult {
                    input: _input,
                    output,
                    error,
                } = self.installer.uninstall(
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
