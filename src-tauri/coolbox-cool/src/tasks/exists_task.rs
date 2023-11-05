use crate::result::CoolResult;
use crate::tasks::{Executable, ExecutableState};
use color_eyre::eyre::eyre;
use coolbox_macros::State;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, State)]
pub struct ExistsTask {
    pub path: String,

    #[serde(skip)]
    state: ExecutableState,
    #[serde(skip)]
    outputs: Vec<String>,
    #[serde(skip)]
    errors: Vec<String>,
}

impl ExistsTask {
    pub fn new(path: String) -> Self {
        Self {
            path,
            state: ExecutableState::NotStarted,
            outputs: vec![],
            errors: vec![],
        }
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
    fn _run(&mut self) -> CoolResult<()> {
        if Path::new(&self.path).exists() {
            Ok(())
        } else {
            let msg = format!("{} does not exist", self.path);
            self.errors.push(msg.clone());
            Err(eyre!(msg))
        }
    }
}
