use std::fmt::{Display, Formatter};

use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};

use crate::error::ExecutableError;
use crate::installer::{Installable, Installer};
use crate::result::ExecutableResult;
use crate::tasks::{Executable, ExecutableSender};
use crate::IntoMessage;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CheckTask {
    pub name: String,
    pub installer: Installer,
}

impl CheckTask {
    pub fn new(name: String, installer: Installer) -> Self {
        Self { name, installer }
    }
}

impl Display for CheckTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "check {}", self.name)
    }
}

impl Executable for CheckTask {
    fn _run(&mut self, sender: &ExecutableSender) -> ExecutableResult {
        self.installer
            .check_available(&self.name, None)
            .map_err(|r| ExecutableError::ShellError(r))
            .and_then(|result| {
                if result {
                    sender
                        .message
                        .send(format!("{} is available", &self.name).into_info())
                        .unwrap();
                    Ok(())
                } else {
                    Err(ExecutableError::NotAvailable(eyre!(
                        "{} is not available",
                        &self.name
                    )))
                }
            })
    }
}
