use std::process::Command;

use crossbeam::channel::Sender;
use log::info;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::ShellExecutor;
use crate::ExecutableMessage;

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

    fn install(
        &self,
        name: &str,
        args: Option<&[&str]>,
        sender: Sender<ExecutableMessage>,
    ) -> CoolResult<()> {
        info!("installing {} with rpm", name);

        let mut arguments = vec![];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        self.run("-i", Some(&arguments), None, Some(sender))
    }

    fn uninstall(
        &self,
        name: &str,
        args: Option<&[&str]>,
        sender: Sender<ExecutableMessage>,
    ) -> CoolResult<()> {
        info!("uninstalling {} with rpm", name);

        let mut arguments = vec![];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        self.run("-e", Some(&arguments), None, Some(sender))
    }

    fn check_available(&self, name: &str, _args: Option<&[&str]>) -> CoolResult<bool> {
        info!("checking {}", name);

        self.run("-q", Some(vec![name].as_slice()), None, None)
            .map(|_| true)
            .or_else(|_| Ok(false))
    }
}
