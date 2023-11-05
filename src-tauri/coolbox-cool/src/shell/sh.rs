use std::process::Command;

use crate::shell::ShellExecutor;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sh;

impl ShellExecutor for Sh {
    fn interpreter(&self) -> Command {
        Command::new("sh")
    }
}

#[cfg(test)]
mod test {
    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::shell::{Bash, Sh, ShellExecutor, Zsh};

    #[test]
    fn test() -> CoolResult<()> {
        init_backtrace();
        let script = reqwest::blocking::get("https://sh.rustup.rs")?.text()?;
        Sh.run(&script, Some(&["-h"]), None)?;
        Bash.run(&script, Some(&["-h"]), None)?;
        Zsh.run(&script, Some(&["-h"]), None)?;
        Ok(())
    }
}
