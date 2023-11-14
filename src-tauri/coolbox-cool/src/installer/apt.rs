use crossbeam::channel::Sender;
use schemars::JsonSchema;
use tracing::info;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::{Bash, ShellExecutor};
use crate::Message;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Apt;

impl Installable for Apt {
    fn name(&self) -> &'static str {
        "apt"
    }

    fn install(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<()> {
        info!("installing {} with apt-get", name);

        let mut arguments = vec!["-y"];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);
        let script = format!("pkexec apt-get install {}", arguments.join(" "));

        Bash.run(&script, envs, Some(sender))
    }

    fn uninstall(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<()> {
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
    ) -> CoolResult<bool> {
        info!("checking {} with dpkg", name);
        Ok(Bash.run("dpkg", envs, None).is_ok())
    }
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod test {
    use crate::cool_test::init_test;
    use crate::installer::apt::APT;
    use crate::installer::Installable;
    use crate::result::CoolResult;

    #[test]
    #[ignore]
    fn test() -> CoolResult<()> {
        init_test();
        if !APT.check_available("bat", None)? {
            APT.install("bat", None)?;
        }
        if APT.check_available("bat", None)? {
            APT.uninstall("bat", None)?;
        }
        Ok(())
    }
}
