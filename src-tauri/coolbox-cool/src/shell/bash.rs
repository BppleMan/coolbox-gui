use std::process::Command;

use schemars::JsonSchema;

use crate::shell::ShellExecutor;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Bash;

impl ShellExecutor for Bash {
    fn name(&self) -> &'static str {
        "bash"
    }

    fn interpreter(&self) -> Command {
        Command::new("bash")
    }
}
