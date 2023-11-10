use std::process::Command;

use crossbeam::channel::Sender;
use tracing::info;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::{Sh, ShellExecutor};
use crate::Message;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Apt;

impl ShellExecutor for Apt {
    fn interpreter(&self) -> Command {
        let mut command = Command::new("sudo");
        command.arg("apt-get");
        command
    }
}

impl Installable for Apt {
    fn name(&self) -> &'static str {
        "apt"
    }

    fn install(
        &self,
        name: &str,
        args: Option<&[&str]>,
        sender: Sender<Message>,
    ) -> CoolResult<()> {
        info!("installing {} with apt-get", name);

        let mut arguments = vec!["-y"];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        self.run("install", Some(&arguments), None, Some(sender))
    }

    fn uninstall(
        &self,
        name: &str,
        args: Option<&[&str]>,
        sender: Sender<Message>,
    ) -> CoolResult<()> {
        info!("uninstalling {} with apt-get", name);

        let mut arguments = vec!["-y", "--purge"];
        if let Some(args) = args {
            arguments.append(&mut args.to_vec());
        }
        arguments.push(name);

        self.run("remove", Some(&arguments), None, Some(sender.clone()))?;
        self.run("autoremove", None, None, Some(sender))
    }

    fn check_available(&self, name: &str, _args: Option<&[&str]>) -> CoolResult<bool> {
        info!("checking {}", name);

        Ok(Sh.run("dpkg", Some(&["-L", name]), None, None).is_ok())
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
