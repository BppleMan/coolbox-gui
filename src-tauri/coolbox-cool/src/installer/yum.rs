use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::{ShellExecutor, ShellResult};
use log::info;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Yum;

impl ShellExecutor for Yum {
    fn interpreter(&self) -> Command {
        let mut command = Command::new("sudo");
        command.arg("yum");
        command
    }
}

impl Installable for Yum {
    fn name(&self) -> &'static str {
        "yum"
    }

    fn install(&mut self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult> {
        info!("installing {} with yum", name);

        let mut arguments = vec!["-y"];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        self.run("install", Some(&arguments), None)
    }

    fn uninstall(&mut self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult> {
        info!("uninstalling {} with yum", name);

        let mut arguments = vec![];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        self.run("remove", Some(&arguments), None)
    }

    fn check_available(&mut self, name: &str, _args: Option<&[&str]>) -> CoolResult<bool> {
        info!("checking {}", name);

        self.run("list", Some(vec!["installed", name].as_slice()), None)
            .map(|_| true)
            .or_else(|_| Ok(false))
    }
}
