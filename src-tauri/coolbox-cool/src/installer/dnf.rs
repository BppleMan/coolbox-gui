use crate::cool::Message;
use crate::error::ShellError;
use crossbeam::channel::Sender;
use log::info;
use schemars::JsonSchema;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::{Bash, ShellExecutor};

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Dnf;

impl Installable for Dnf {
    fn name(&self) -> &'static str {
        "dnf"
    }

    fn install(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<(), ShellError> {
        info!("installing {} with dnf", name);

        let mut arguments = vec!["-y"];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        Bash.run(
            &format!("dnf install {}", arguments.join(" ")),
            envs,
            Some(sender),
        )
    }

    fn uninstall(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<(), ShellError> {
        info!("uninstalling {} with rpm", name);

        let mut arguments = vec![];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        Bash.run(
            &format!("dnf remove {}", arguments.join(" ")),
            envs,
            Some(sender),
        )
    }

    fn check_available(
        &self,
        name: &str,
        _args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
    ) -> CoolResult<bool, ShellError> {
        info!("checking {}", name);

        Bash.run(&format!("dnf list installed {}", name), envs, None)
            .map(|_| true)
            .or_else(|_| Ok(false))
    }
}
