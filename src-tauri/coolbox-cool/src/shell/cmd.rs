use std::process::Command;

use schemars::JsonSchema;

use crate::result::CoolResult;
use crate::shell::ShellExecutor;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Cmd;

impl ShellExecutor for Cmd {
    fn interpreter(&self) -> Command {
        Command::new("cmd")
    }

    fn command(&self, script: &str, envs: Option<&[(&str, &str)]>) -> CoolResult<Command> {
        let mut command = self.interpreter();

        command.arg("/C").arg(script);
        if let Some(envs) = envs {
            command.envs(envs.to_vec());
        }

        Ok(command)
    }
}
