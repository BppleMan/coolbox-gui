use std::process::Command;

use crate::shell::ShellExecutor;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Zsh;

impl ShellExecutor for Zsh {
    fn interpreter(&self) -> Command {
        Command::new("zsh")
    }
}
