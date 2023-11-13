use std::fmt::{Display, Formatter};

use color_eyre::eyre::eyre;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use which::which;

use crate::error::ExecutableError;
use crate::result::ExecutableResult;
use crate::tasks::{Executable, MessageSender};
use crate::IntoInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
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

impl<'a> Executable<'a> for WhichTask {
    fn execute(&self, mut send: Box<MessageSender<'a>>) -> ExecutableResult {
        match which(&self.command) {
            Ok(result) => {
                send(result.to_string_lossy().into_info());
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
    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::tasks::spawn_task;

    use super::*;

    #[test]
    fn test_which() -> CoolResult<()> {
        init_backtrace();
        let which = WhichTask::new("ls".to_string());
        let mut outputs = String::new();
        spawn_task(which, |msg| {
            outputs.push_str(&msg.message);
        })?;
        pretty_assertions::assert_eq!(outputs, "/bin/ls");
        Ok(())
    }
}
