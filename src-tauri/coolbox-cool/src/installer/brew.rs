use std::process::Command;

use tracing::info;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::{ShellExecutor, ShellResult};

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

    fn install(&self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult> {
        info!("installing {} with brew", name);

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

    fn uninstall(&self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult> {
        info!("uninstalling {} with brew", name);

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

    fn check_available(&self, name: &str, _args: Option<&[&str]>) -> CoolResult<bool> {
        info!("checking {} with brew", name);

        Ok(self.run("list", Some(&[name]), None).is_ok())
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
        if Brew.check_available("wget", None)? {
            Brew.uninstall("wget", None)?;
        }
        Brew.install("wget", None)?;
        Brew.uninstall("wget", None)?;
        Ok(())
    }
}
