use crate::shell::ShellExecutor;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Zsh;

impl ShellExecutor for Zsh {
    fn interpreter(&self) -> Command {
        Command::new("zsh")
    }
}
