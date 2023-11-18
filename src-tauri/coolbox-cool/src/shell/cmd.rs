use std::process::Command;

use schemars::JsonSchema;

use crate::shell::ShellExecutor;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Cmd;

impl ShellExecutor for Cmd {
    fn name(&self) -> &'static str {
        "cmd"
    }

    fn interpreter(&self) -> Command {
        Command::new("cmd")
    }

    fn command(&self, script: &str, envs: Option<&[(&str, &str)]>) -> Command {
        let mut command = self.interpreter();

        command.arg("/C").arg(script);
        if let Some(envs) = envs {
            command.envs(envs.to_vec());
        } else {
            command.env_clear();
        }

        command
    }
}
