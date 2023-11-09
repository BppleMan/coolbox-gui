use std::fmt::{Display, Formatter};
use std::path::Path;

use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};

use crate::error::ExecutableError;
use crate::result::ExecutableResult;
use crate::tasks::{Executable, ExecutableSender};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExistsTask {
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

impl Executable for ExistsTask {
    fn _run(&self, _sender: &ExecutableSender) -> ExecutableResult {
        if Path::new(&self.path).exists() {
            Ok(())
        } else {
            Err(ExecutableError::FileNotExists(eyre!(
                "{} does not exist",
                self.path
            )))
        }
    }
}
