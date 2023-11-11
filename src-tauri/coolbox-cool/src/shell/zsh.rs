use schemars::JsonSchema;
use std::process::Command;

use crate::shell::ShellExecutor;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Zsh;

impl ShellExecutor for Zsh {
    fn interpreter(&self) -> Command {
        Command::new("zsh")
    }
}
