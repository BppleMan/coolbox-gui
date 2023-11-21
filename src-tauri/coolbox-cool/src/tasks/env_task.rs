use crate::env_manager::EnvVar;
use crate::error::{EnvTaskError, TaskError};
use crate::result::CoolResult;
use crate::shell::{Bash, ShellExecutor};
use crate::tasks::Executable;
use crate::MessageSender;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct EnvTask {
    pub command: EnvCommand,
}

impl EnvTask {
    pub fn new(command: EnvCommand) -> Self {
        Self { command }
    }
}

impl Display for EnvTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

impl<'a> Executable<'a> for EnvTask {
    fn execute(&self, send: Box<MessageSender<'a>>) -> CoolResult<(), TaskError> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum EnvCommand {
    Export(EnvVar),
    Unset(String),
    AppendPath(String),
    RemovePath(String),
    #[cfg(unix)]
    AddSource(String),
    #[cfg(unix)]
    RemoveSource(String),
}

impl Display for EnvCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!();
        // match self {
        //     EnvCommand::Export(env_var) => {
        //         if cfg!(unix) {
        //             write!(f, "export {}", env_var)
        //         } else {
        //             write!(f, "set {}", env_var)
        //         }
        //     }
        //     EnvCommand::Unset(value) => {
        //         if cfg!(unix) {
        //             write!(f, "unset {}", value)
        //         } else {
        //             write!(f, "set {}=", value)
        //         }
        //     }
        //     EnvCommand::AppendPath(value) => {}
        //     EnvCommand::RemovePath(value) => {
        //         if cfg!(unix) {
        //             write!(f, "unset {}", value)
        //         } else {
        //             write!(f, "set PATH=%PATH:{};=%", value)
        //         }
        //     }
        //     #[cfg(unix)]
        //     EnvCommand::AddSource(value) => {}
        //     #[cfg(unix)]
        //     EnvCommand::RemoveSource(value) => {}
        // }
    }
}

#[cfg(test)]
mod test {
    use std::env;
    use std::process::Command;

    #[test]
    fn test() {
        println!("{:?}", env::current_dir());
        Command::new("zsh")
            .arg("-c")
            .arg("source ../coolrc && env")
            .env_clear()
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
