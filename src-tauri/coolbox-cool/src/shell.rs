use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use color_eyre::eyre::eyre;
use crossbeam::channel::{Receiver, Sender};
use log::info;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use bash::*;
pub use cmd::*;
pub use linux_sudo::*;
pub use macos_sudo::*;
pub use sh::*;
pub use zsh::*;

use crate::result::CoolResult;
use crate::{IntoInfo, Message};

mod bash;
mod cmd;
mod linux_sudo;
mod macos_sudo;
mod sh;
mod zsh;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub enum Shell {
    Bash(Bash),
    LinuxSudo(LinuxSudo),
    MacOSSudo(MacOSSudo),
    Sh(Sh),
    Zsh(Zsh),
    Cmd(Cmd),
}

impl Shell {
    pub fn name(&self) -> &'static str {
        match self {
            Shell::Bash(_) => "bash",
            Shell::LinuxSudo(_) => "linux_sudo",
            Shell::MacOSSudo(_) => "macos_sudo",
            Shell::Sh(_) => "sh",
            Shell::Zsh(_) => "zsh",
            Shell::Cmd(_) => "cmd",
        }
    }
}

impl AsRef<dyn ShellExecutor> for Shell {
    fn as_ref(&self) -> &(dyn ShellExecutor + 'static) {
        match self {
            Shell::Bash(bash) => bash,
            Shell::LinuxSudo(linux_sudo) => linux_sudo,
            Shell::MacOSSudo(macos_sudo) => macos_sudo,
            Shell::Sh(sh) => sh,
            Shell::Zsh(zsh) => zsh,
            Shell::Cmd(cmd) => cmd,
        }
    }
}

impl ShellExecutor for Shell {
    fn interpreter(&self) -> Command {
        self.as_ref().interpreter()
    }

    fn command(&self, script: &str, envs: Option<&[(&str, &str)]>) -> CoolResult<Command> {
        self.as_ref().command(script, envs)
    }

    fn run(
        &self,
        cmd: &str,
        envs: Option<&[(&str, &str)]>,
        sender: Option<Sender<Message>>,
    ) -> CoolResult<()> {
        self.as_ref().run(cmd, envs, sender)
    }
}

impl Serialize for Shell {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Shell::Bash(_) => serializer.serialize_str("bash"),
            Shell::LinuxSudo(_) => serializer.serialize_str("linux_sudo"),
            Shell::MacOSSudo(_) => serializer.serialize_str("macos_sudo"),
            Shell::Sh(_) => serializer.serialize_str("sh"),
            Shell::Zsh(_) => serializer.serialize_str("zsh"),
            Shell::Cmd(_) => serializer.serialize_str("cmd"),
        }
    }
}

impl<'de> Deserialize<'de> for Shell {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = <String>::deserialize(deserializer)?;
        match name {
            name if name == "bash" => Ok(Shell::Bash(Bash)),
            name if name == "linux_sudo" => Ok(Shell::LinuxSudo(LinuxSudo)),
            name if name == "macos_sudo" => Ok(Shell::MacOSSudo(MacOSSudo)),
            name if name == "sh" => Ok(Shell::Sh(Sh)),
            name if name == "zsh" => Ok(Shell::Zsh(Zsh)),
            name if name == "cmd" => Ok(Shell::Cmd(Cmd)),
            _ => Err(serde::de::Error::custom(format!("unknown shell: {}", name))),
        }
    }
}

pub trait ShellExecutor {
    fn interpreter(&self) -> Command;

    fn command(&self, script: &str, envs: Option<&[(&str, &str)]>) -> CoolResult<Command> {
        let mut command = self.interpreter();
        command.arg("-c").arg(script);
        if let Some(envs) = envs {
            command.envs(envs.to_vec());
        }
        Ok(command)
    }

    fn run(
        &self,
        script: &str,
        envs: Option<&[(&str, &str)]>,
        sender: Option<Sender<Message>>,
    ) -> CoolResult<()> {
        let mut command = self.command(script, envs)?;
        let command_desc = format!("{:?}", command);
        info!("run: {}", command_desc);
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = command.spawn()?;

        let out_reader = BufReader::new(child.stdout.take().unwrap());
        let err_reader = BufReader::new(child.stderr.take().unwrap());
        let (tx, rx) = crossbeam::channel::bounded(1);
        rayon::scope(|s| {
            s.spawn(|_| redirect(out_reader, &sender));
            s.spawn(|_| redirect(err_reader, &sender));
            s.spawn(|_| {
                let result = child.wait_with_output();
                tx.send(result).unwrap();
            })
        });

        let result = rx.recv().unwrap()?;
        match result.status.success() {
            true => Ok(()),
            false => Err(eyre!("run command failed: {}", command_desc)),
        }
    }
}

fn redirect(mut reader: impl BufRead, sender: &Option<Sender<Message>>) {
    let mut buf = String::new();
    while let Ok(size) = reader.read_line(&mut buf) {
        if size == 0 {
            break;
        }
        if let Some(sender) = sender.as_ref() {
            sender
                .send(std::mem::take(&mut buf.trim()).into_info())
                .unwrap();
        }
        buf.clear();
    }
}

#[derive(Debug, Clone)]
pub struct ShellResult {
    pub input: Sender<String>,
    pub output: Receiver<String>,
    pub error: Receiver<String>,
}

impl ShellResult {
    pub fn new(input: Sender<String>, output: Receiver<String>, error: Receiver<String>) -> Self {
        Self {
            input,
            output,
            error,
        }
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use crate::init_backtrace;
    use crate::result::CoolResult;

    #[test]
    fn test() -> CoolResult<()> {
        init_backtrace();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Test {
            pub name: Option<String>,
            pub age: Option<u8>,
        }

        let test = Test {
            name: Some("test".to_string()),
            age: None,
        };

        let toml = toml::to_string(&test)?;
        println!("{}", toml);
        let test2: Test = toml::from_str(&toml)?;
        println!("{:?}", test2);
        Ok(())
    }
}
