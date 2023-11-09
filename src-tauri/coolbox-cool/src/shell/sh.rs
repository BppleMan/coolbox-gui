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
        let (sender, receiver) = crossbeam::channel::unbounded();
        rayon::spawn(move || {
            receiver.iter().for_each(|message| {
                println!("{}", message);
            });
        });
        Sh.run(&script, Some(&["-h"]), None, Some(sender.clone()))?;
        Bash.run(&script, Some(&["-h"]), None, Some(sender.clone()))?;
        Zsh.run(&script, Some(&["-h"]), None, Some(sender))?;
        Ok(())
    }
}
