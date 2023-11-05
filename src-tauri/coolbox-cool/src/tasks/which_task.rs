use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use which::which;

use coolbox_macros::State;

use crate::result::CoolResult;
use crate::tasks::{Executable, ExecutableState};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, State)]
pub struct WhichTask {
    pub command: String,

    #[serde(skip)]
    state: ExecutableState,
    #[serde(skip)]
    outputs: Vec<String>,
    #[serde(skip)]
    errors: Vec<String>,
}

impl WhichTask {
    pub fn new(command: String) -> Self {
        Self {
            command,
            state: ExecutableState::NotStarted,
            outputs: vec![],
            errors: vec![],
        }
    }
}

impl Display for WhichTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if cfg!(target_os = "unix") {
            write!(f, "which {}", self.command)
        } else if cfg!(target_os = "windows") {
            write!(f, "where {}", self.command)
        } else {
            write!(f, "which {}", self.command)
        }
    }
}

impl Executable for WhichTask {
    fn _run(&mut self) -> CoolResult<()> {
        match which(&self.command) {
            Ok(result) => {
                self.outputs.push(result.to_string_lossy().to_string());
                Ok(())
            }
            Err(_) => {
                let msg = format!("{} not found", &self.command);
                self.errors.push(msg.clone());
                Err(eyre!(msg))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_which() -> CoolResult<()> {
        let mut which = WhichTask::new("ls".to_string());
        which.execute()?;
        assert_eq!(which.outputs.len(), 1);
        pretty_assertions::assert_eq!(which.outputs[0], "/bin/ls");
        Ok(())
    }
}
