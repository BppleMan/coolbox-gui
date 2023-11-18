use std::fmt::{Display, Formatter};
use std::path::Path;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::TaskError;
use crate::result::CoolResult;
use crate::tasks::{Executable, MessageSender};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct ExistsTask {
    #[serde(deserialize_with = "crate::template_string")]
    pub path: String,
}

impl ExistsTask {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl Display for ExistsTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if cfg!(target_os = "unix") {
            write!(f, "test -e {}", self.path)
        } else if cfg!(target_os = "windows") {
            write!(f, "if exist {}", self.path)
        } else {
            write!(f, "test -e {}", self.path)
        }
    }
}

impl<'a> Executable<'a> for ExistsTask {
    fn execute(&self, _send: Box<MessageSender<'a>>) -> CoolResult<(), TaskError> {
        if Path::new(&self.path).exists() {
            Ok(())
        } else {
            Err(TaskError::ExistsTaskError(self.path.clone()))
        }
    }
}
