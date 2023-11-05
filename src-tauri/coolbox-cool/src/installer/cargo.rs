use std::process::Command;

use tracing::info;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::{ShellExecutor, ShellResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    fn install(&mut self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult> {
        info!("installing {} with cargo", name);

        let args = match args {
            None => vec![name],
            Some(args) => {
                let mut args = args.to_vec();
                args.insert(0, name);
                args
            }
        };

        self.run("install", Some(&args), None)
    }

    fn uninstall(&mut self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult> {
        info!("uninstalling {} with cargo", name);

        let args = match args {
            None => vec![name],
            Some(args) => {
                let mut args = args.to_vec();
                args.insert(0, name);
                args
            }
        };

        self.run("uninstall", Some(&args), None)
    }

    fn check_available(&mut self, name: &str, _args: Option<&[&str]>) -> CoolResult<bool> {
        info!("checking {} with cargo", name);
        let ShellResult { output, .. } = self.run("install", Some(&["--list"]), None)?;
        let result = output.iter().collect::<Vec<_>>().join("\n");
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
        if Cargo.check_available("zoxide", None)? {
            Cargo.uninstall("zoxide", None)?;
        }
        Cargo.install("zoxide", None)?;
        Cargo.uninstall("zoxide", None)?;
        Ok(())
    }
}
