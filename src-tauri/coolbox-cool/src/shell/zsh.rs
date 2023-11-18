use std::process::Command;

use schemars::JsonSchema;

use crate::shell::ShellExecutor;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Zsh;

impl ShellExecutor for Zsh {
    fn name(&self) -> &'static str {
        "zsh"
    }

    fn interpreter(&self) -> Command {
        Command::new("zsh")
    }
}
