use std::fmt::{Display, Formatter};

use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};

use crate::error::ExecutableError;
use crate::installer::{Installable, Installer};
use crate::result::ExecutableResult;
use crate::shell::ShellResult;
use crate::tasks::{Executable, ExecutableSender};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UninstallTask {
    pub name: String,
    pub args: Option<Vec<String>>,
    pub installer: Installer,
}

impl UninstallTask {
    pub fn new(name: String, args: Option<Vec<String>>, installer: Installer) -> Self {
        Self {
            name,
            args,
            installer,
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
    fn _run(&mut self, sender: &ExecutableSender) -> ExecutableResult {
        let initial_result = Err(ExecutableError::ShellError(eyre!("No attempts made")));

        (0..5).fold(initial_result, |acc, _| {
            if let Err(_) = acc {
                let ShellResult {
                    input: _input,
                    output,
                    error,
                } = self
                    .installer
                    .uninstall(
                        &self.name,
                        self.args
                            .as_ref()
                            .map(|args| args.iter().map(AsRef::as_ref).collect::<Vec<_>>())
                            .as_deref(),
                    )
                    .map_err(|e| ExecutableError::ShellError(e))?;

                rayon::scope(|s| {
                    s.spawn(|_| {
                        while let Ok(r) = output.recv() {
                            sender.outputs.send(r).unwrap();
                        }
                    });
                    s.spawn(|_| {
                        while let Ok(r) = error.recv() {
                            sender.errors.send(r).unwrap();
                        }
                    });
                });
            }
            Ok(())
        })
    }
}
