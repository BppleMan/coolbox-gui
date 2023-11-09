use std::process::Command;

use log::info;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::{ShellExecutor, ShellResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rpm;

impl ShellExecutor for Rpm {
    fn interpreter(&self) -> Command {
        let mut command = Command::new("sudo");
        command.arg("rpm");
        command
    }
}

impl Installable for Rpm {
    fn name(&self) -> &'static str {
        "rpm"
    }

    fn install(&self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult> {
        info!("installing {} with rpm", name);

        let mut arguments = vec![];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        self.run("-i", Some(&arguments), None)
    }

    fn uninstall(&self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult> {
        info!("uninstalling {} with rpm", name);

        let mut arguments = vec![];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        self.run("-e", Some(&arguments), None)
    }

    fn check_available(&self, name: &str, _args: Option<&[&str]>) -> CoolResult<bool> {
        info!("checking {}", name);

        self.run("-q", Some(vec![name].as_slice()), None)
            .map(|_| true)
            .or_else(|_| Ok(false))
    }
}
