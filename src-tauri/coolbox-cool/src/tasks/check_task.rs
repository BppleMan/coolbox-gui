use std::fmt::{Display, Formatter};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{CheckTaskError, TaskError};
use crate::installer::{Installable, Installer};
use crate::result::CoolResult;
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
    fn execute(&self, mut send: Box<MessageSender<'a>>) -> CoolResult<(), TaskError> {
        self.installer
            .check_available(&self.name, None, None)
            .map_err(|e| TaskError::CheckTaskError {
                task: self.clone(),
                source: CheckTaskError::ShellError(e),
            })
            .and_then(|result| {
                if result {
                    send(format!("{} is available", &self.name).into_info());
                    Ok(())
                } else {
                    Err(TaskError::CheckTaskError {
                        task: self.clone(),
                        source: CheckTaskError::NotAvailable {
                            name: self.name.clone(),
                            installer: self.installer.name().to_string(),
                        },
                    })
                }
            })
    }
}
