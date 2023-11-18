use crate::error::ShellError;
use crossbeam::channel::Sender;
use schemars::JsonSchema;
use tracing::info;

use crate::installer::Installable;
use crate::result::CoolResult;
use crate::shell::{Bash, ShellExecutor};
use crate::Message;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Brew;

impl Installable for Brew {
    fn name(&self) -> &'static str {
        "brew"
    }

    fn install(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<(), ShellError> {
        info!("installing {} with brew", name);

        let arguments = match args {
            None => vec![name],
            Some(args) => {
                let mut args = args.to_vec();
                args.insert(0, name);
                args
            }
        };

        Bash.run(
            &format!("brew install {}", arguments.join(" ")),
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
        info!("uninstalling {} with brew", name);

        let arguments = match args {
            None => vec![name],
            Some(args) => {
                let mut args = args.to_vec();
                args.insert(0, name);
                args
            }
        };

        Bash.run(
            &format!("brew uninstall {}", arguments.join(" ")),
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
        info!("checking {} with brew", name);

        Ok(Bash.run(&format!("brew list {}", name), envs, None).is_ok())
    }
}

#[cfg(test)]
mod test {
    use crate::init_backtrace;
    use crate::installer::{Brew, Installable};
    use crate::result::CoolResult;

    #[test]
    fn install_wget() -> CoolResult<()> {
        init_backtrace();
        let (sender, receiver) = crossbeam::channel::unbounded();
        rayon::spawn(|| {
            for msg in receiver {
                println!("{}", msg);
            }
        });
        if Brew.check_available("wget", None, None)? {
            Brew.uninstall("wget", None, None, sender.clone())?;
        }
        Brew.install("wget", None, None, sender.clone())?;
        Brew.uninstall("wget", None, None, sender.clone())?;
        Ok(())
    }
}
