use std::process::Command;

use crossbeam::channel::Sender;
use tracing::info;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::ShellExecutor;
use crate::ExecutableMessage;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Brew;

impl ShellExecutor for Brew {
    fn interpreter(&self) -> Command {
        Command::new("brew")
    }

    fn command(&self, cmd: &str, args: Option<&[&str]>) -> CoolResult<Command> {
        let mut command = self.interpreter();
        command.arg(cmd);
        if let Some(args) = args {
            command.args(args);
        }
        Ok(command)
    }
}

impl Installable for Brew {
    fn name(&self) -> &'static str {
        "brew"
    }

    fn install(
        &self,
        name: &str,
        args: Option<&[&str]>,
        sender: Sender<ExecutableMessage>,
    ) -> CoolResult<()> {
        info!("installing {} with brew", name);

        let args = match args {
            None => vec![name],
            Some(args) => {
                let mut args = args.to_vec();
                args.insert(0, name);
                args
            }
        };

        self.run("install", Some(&args), None, Some(sender))
    }

    fn uninstall(
        &self,
        name: &str,
        args: Option<&[&str]>,
        sender: Sender<ExecutableMessage>,
    ) -> CoolResult<()> {
        info!("uninstalling {} with brew", name);

        let args = match args {
            None => vec![name],
            Some(args) => {
                let mut args = args.to_vec();
                args.insert(0, name);
                args
            }
        };

        self.run("uninstall", Some(&args), None, Some(sender))
    }

    fn check_available(&self, name: &str, _args: Option<&[&str]>) -> CoolResult<bool> {
        info!("checking {} with brew", name);

        Ok(self.run("list", Some(&[name]), None, None).is_ok())
    }
}

#[cfg(test)]
mod test {
    use crate::init_backtrace;
    use crate::installer::{Brew, Installable};
    use crate::result::CoolResult;

    #[test]
    fn install_wget() -> CoolResult<()> {
        init_backtrace();
        let (sender, receiver) = crossbeam::channel::unbounded();
        rayon::spawn(|| {
            for msg in receiver {
                println!("{}", msg);
            }
        });
        if Brew.check_available("wget", None)? {
            Brew.uninstall("wget", None, sender.clone())?;
        }
        Brew.install("wget", None, sender.clone())?;
        Brew.uninstall("wget", None, sender.clone())?;
        Ok(())
    }
}
