use std::process::Command;

use log::info;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::{ShellExecutor, ShellResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dnf;

impl ShellExecutor for Dnf {
    fn interpreter(&self) -> Command {
        let mut command = Command::new("sudo");
        command.arg("dnf");
        command
    }
}

impl Installable for Dnf {
    fn name(&self) -> &'static str {
        "dnf"
    }

    fn install(&self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult> {
        info!("installing {} with dnf", name);

        let mut arguments = vec!["-y"];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        self.run("install", Some(&arguments), None)
    }

    fn uninstall(&self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult> {
        info!("uninstalling {} with rpm", name);

        let mut arguments = vec![];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        self.run("remove", Some(&arguments), None)
    }

    fn check_available(&self, name: &str, _args: Option<&[&str]>) -> CoolResult<bool> {
        info!("checking {}", name);

        self.run("list", Some(vec!["installed", name].as_slice()), None)
            .map(|_| true)
            .or_else(|_| Ok(false))
    }
}
