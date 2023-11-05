use std::process::Command;

use crate::shell::ShellExecutor;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bash;

impl ShellExecutor for Bash {
    fn interpreter(&self) -> Command {
        Command::new("bash")
    }
}
