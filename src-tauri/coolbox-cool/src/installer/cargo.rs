use crate::cool::Message;
use crate::error::ShellError;
use crossbeam::channel::Sender;
use schemars::JsonSchema;
use tracing::info;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::{Bash, ShellExecutor};

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Cargo;

impl Installable for Cargo {
    fn name(&self) -> &'static str {
        "cargo"
    }

    fn install(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<(), ShellError> {
        info!("installing {} with cargo", name);

        let args = match args {
            None => vec![name],
            Some(args) => {
                let mut args = args.to_vec();
                args.insert(0, name);
                args
            }
        };

        Bash.run(
            &format!("cargo install {}", args.join(" ")),
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
        info!("uninstalling {} with cargo", name);

        let args = match args {
            None => vec![name],
            Some(args) => {
                let mut args = args.to_vec();
                args.insert(0, name);
                args
            }
        };

        Bash.run(
            &format!("cargo uninstall {}", args.join(" ")),
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
        info!("checking {} with cargo", name);
        let (sender, receiver) = crossbeam::channel::unbounded::<Message>();
        Bash.run(&format!("cargo install --list"), envs, Some(sender))?;
        let result = receiver
            .iter()
            .map(|m| m.message)
            .collect::<Vec<_>>()
            .join("\n");
        Ok(result.contains(name))
    }
}

#[cfg(test)]
mod test {
    use crate::init_backtrace;
    use crate::installer::{Cargo, Installable};
    use crate::result::CoolResult;

    #[test]
    fn install_bat() -> CoolResult<()> {
        init_backtrace();
        let (sender, _) = crossbeam::channel::unbounded();
        if Cargo.check_available("zoxide", None, None)? {
            Cargo.uninstall("zoxide", None, None, sender.clone())?;
        }
        Cargo.install("zoxide", None, None, sender.clone())?;
        Cargo.uninstall("zoxide", None, None, sender.clone())?;
        Ok(())
    }
}
