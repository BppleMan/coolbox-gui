use std::fmt::{Display, Formatter};

use color_eyre::eyre::eyre;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ExecutableError;
use crate::installer::{Installable, Installer};
use crate::result::ExecutableResult;
use crate::tasks::{Executable, MessageSender};
use crate::IntoInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
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

impl<'a> Executable<'a> for CheckTask {
    fn execute(&self, mut send: Box<MessageSender<'a>>) -> ExecutableResult {
        self.installer
            .check_available(&self.name, None)
            .map_err(ExecutableError::ShellError)
            .and_then(|result| {
                if result {
                    send(format!("{} is available", &self.name).into_info());
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
