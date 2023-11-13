use std::process::Command;

use crossbeam::channel::Sender;
use schemars::JsonSchema;
use tracing::info;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::ShellExecutor;
use crate::Message;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
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
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
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

        self.run("install", Some(&args), envs, Some(sender))
    }

    fn uninstall(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
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

        self.run("uninstall", Some(&args), envs, Some(sender))
    }

    fn check_available(
        &self,
        name: &str,
        _args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
    ) -> CoolResult<bool> {
        info!("checking {} with brew", name);

        Ok(self.run("list", Some(&[name]), envs, None).is_ok())
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
        if Brew.check_available("wget", None, None)? {
            Brew.uninstall("wget", None, None, sender.clone())?;
        }
        Brew.install("wget", None, None, sender.clone())?;
        Brew.uninstall("wget", None, None, sender.clone())?;
        Ok(())
    }
}
