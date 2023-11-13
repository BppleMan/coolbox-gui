use std::process::Command;

use crossbeam::channel::Sender;
use log::info;
use schemars::JsonSchema;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::ShellExecutor;
use crate::Message;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Yum;

impl ShellExecutor for Yum {
    fn interpreter(&self) -> Command {
        let mut command = Command::new("pkexec");
        command.arg("yum");
        command
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

impl Installable for Yum {
    fn name(&self) -> &'static str {
        "yum"
    }

    fn install(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<()> {
        info!("installing {} with yum", name);

        let mut arguments = vec!["-y"];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        self.run("install", Some(&arguments), envs, Some(sender))
    }

    fn uninstall(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<()> {
        info!("uninstalling {} with yum", name);

        let mut arguments = vec![];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        self.run("remove", Some(&arguments), envs, Some(sender))
    }

    fn check_available(
        &self,
        name: &str,
        _args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
    ) -> CoolResult<bool> {
        info!("checking {}", name);

        self.run("list", Some(vec!["installed", name].as_slice()), envs, None)
            .map(|_| true)
            .or_else(|_| Ok(false))
    }
}
