use crate::error::ShellError;
use crossbeam::channel::Sender;
use schemars::JsonSchema;
use tracing::info;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::{Bash, ShellExecutor};
use crate::Message;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct WinGet;

impl Installable for WinGet {
    fn name(&self) -> &'static str {
        "winget"
    }

    fn install(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<(), ShellError> {
        info!("installing {} with winget", name);

        let mut scripts = args.map_or(vec![], |args| args.to_vec());
        scripts.push(name);

        let script = format!("winget install {}", scripts.join(" "));

        Bash.run(&script, envs, Some(sender))
    }

    fn uninstall(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<(), ShellError> {
        info!("uninstalling {} with apt-get", name);

        let mut arguments = vec!["-y", "--purge"];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        Bash.run(
            &format!("pkexec apt-get remove {}", arguments.join(" ")),
            envs,
            Some(sender.clone()),
        )?;
        Bash.run(
            &format!("pkexec apt-get autoremove {}", arguments.join(" ")),
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
        info!("checking {} with dpkg", name);
        Ok(Bash.run("dpkg", envs, None).is_ok())
    }
}
