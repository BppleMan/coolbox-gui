use std::fmt::{Display, Formatter};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ExecutableError;
use crate::installer::{Installable, Installer};
use crate::result::ExecutableResult;
use crate::tasks::{Executable, MessageSender};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct UninstallTask {
    pub name: String,
    #[serde(deserialize_with = "crate::template_args", default)]
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

impl<'a> Executable<'a> for UninstallTask {
    fn execute(&self, mut send: Box<MessageSender<'a>>) -> ExecutableResult {
        let this = self.clone();

        let (tx1, rx1) = crossbeam::channel::unbounded();
        let (tx2, rx2) = crossbeam::channel::bounded(1);
        rayon::spawn(move || {
            let result = this
                .installer
                .uninstall(
                    &this.name,
                    this.args
                        .as_ref()
                        .map(|args| args.iter().map(AsRef::as_ref).collect::<Vec<_>>())
                        .as_deref(),
                    None,
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
