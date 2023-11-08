use std::fmt::{Display, Formatter};

use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use which::which;

use crate::error::ExecutableError;
use crate::result::ExecutableResult;
use crate::tasks::{Executable, ExecutableSender};
use crate::IntoMessage;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WhichTask {
    pub command: String,
}

impl WhichTask {
    pub fn new(command: String) -> Self {
        Self { command }
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
    fn _run(&mut self, sender: &ExecutableSender) -> ExecutableResult {
        match which(&self.command) {
            Ok(result) => {
                sender
                    .message
                    .send(result.to_string_lossy().into_info())
                    .unwrap();
                Ok(())
            }
            Err(_) => {
                let report = eyre!("{} not found", &self.command);
                Err(ExecutableError::CommandNotFound(report))
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
