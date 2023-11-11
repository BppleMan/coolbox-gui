use std::process::Command;

use crossbeam::channel::Sender;
use schemars::JsonSchema;
use tracing::info;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::ShellExecutor;
use crate::Message;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Cargo;

impl ShellExecutor for Cargo {
    fn interpreter(&self) -> Command {
        Command::new("cargo")
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

impl Installable for Cargo {
    fn name(&self) -> &'static str {
        "cargo"
    }

    fn install(
        &self,
        name: &str,
        args: Option<&[&str]>,
        sender: Sender<Message>,
    ) -> CoolResult<()> {
        info!("installing {} with cargo", name);

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
        sender: Sender<Message>,
    ) -> CoolResult<()> {
        info!("uninstalling {} with cargo", name);

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
        info!("checking {} with cargo", name);
        let (sender, receiver) = crossbeam::channel::unbounded::<Message>();
        self.run("install", Some(&["--list"]), None, Some(sender))?;
        let result = receiver
            .iter()
            .map(|m| m.message)
            .collect::<Vec<_>>()
            .join("\n");
        Ok(result.contains(name))
    }
}

#[cfg(test)]
mod test {
    use crate::init_backtrace;
    use crate::installer::{Cargo, Installable};
    use crate::result::CoolResult;

    #[test]
    fn install_bat() -> CoolResult<()> {
        init_backtrace();
        let (sender, _) = crossbeam::channel::unbounded();
        if Cargo.check_available("zoxide", None)? {
            Cargo.uninstall("zoxide", None, sender.clone())?;
        }
        Cargo.install("zoxide", None, sender.clone())?;
        Cargo.uninstall("zoxide", None, sender.clone())?;
        Ok(())
    }
}
