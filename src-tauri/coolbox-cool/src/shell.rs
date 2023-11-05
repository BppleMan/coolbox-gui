use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::thread;

use crate::StringExt;
use color_eyre::eyre::eyre;
use crossbeam::channel::{Receiver, Sender};
use log::info;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tracing::debug;

pub use bash::*;
pub use linux_sudo::*;
pub use macos_sudo::*;
pub use sh::*;
pub use zsh::*;

use crate::result::CoolResult;

mod bash;
mod linux_sudo;
mod macos_sudo;
mod sh;
mod zsh;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Shell {
    Bash(Bash),
    LinuxSudo(LinuxSudo),
    MacOSSudo(MacOSSudo),
    Sh(Sh),
    Zsh(Zsh),
}

impl AsRef<dyn ShellExecutor> for Shell {
    fn as_ref(&self) -> &(dyn ShellExecutor + 'static) {
        match self {
            Shell::Bash(bash) => bash,
            Shell::LinuxSudo(linux_sudo) => linux_sudo,
            Shell::MacOSSudo(macos_sudo) => macos_sudo,
            Shell::Sh(sh) => sh,
            Shell::Zsh(zsh) => zsh,
        }
    }
}

impl ShellExecutor for Shell {
    fn interpreter(&self) -> Command {
        self.as_ref().interpreter()
    }

    fn command(&self, cmd: &str, args: Option<&[&str]>) -> CoolResult<Command> {
        self.as_ref().command(cmd, args)
    }

    fn run(
        &self,
        cmd: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
    ) -> CoolResult<ShellResult> {
        self.as_ref().run(cmd, args, envs)
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
            _ => Err(serde::de::Error::custom(format!("unknown shell: {}", name))),
        }
    }
}

pub trait ShellExecutor {
    fn interpreter(&self) -> Command;

    fn command(&self, cmd: &str, args: Option<&[&str]>) -> CoolResult<Command> {
        let mut command = self.interpreter();
        if PathBuf::from_str(cmd)?.exists() {
            command.arg(cmd);
            if let Some(args) = args {
                command.args(args);
            }
        } else {
            command.arg("-c");
            command.arg(cmd);
            if let Some(args) = args {
                command.arg("--");
                command.args(args);
            }
        }
        Ok(command)
    }

    fn prepare(
        &self,
        cmd: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
    ) -> CoolResult<Command> {
        let mut command = self.command(cmd, args)?;
        debug!("{}", format!("{:?}", command).truncate_string(100));
        if let Some(envs) = envs {
            command.envs(envs.to_vec());
        }
        Ok(command)
    }

    fn run(
        &self,
        cmd: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
    ) -> CoolResult<ShellResult> {
        let mut command = self.prepare(cmd, args, envs)?;
        let command_desc = format!("{:?}", command);
        info!("run: {}", command_desc.truncate_string(100));
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = command.spawn()?;

        let (in_sender, in_receiver) = crossbeam::channel::unbounded::<String>();
        let mut in_writer = child.stdin.take().unwrap();
        thread::spawn(move || {
            while let Ok(line) = in_receiver.recv() {
                in_writer.write_all(line.as_bytes()).unwrap();
                in_writer.write_all(b"\n").unwrap();
                in_writer.flush().unwrap();
            }
        });

        let (out_sender, out_receiver) = crossbeam::channel::unbounded::<String>();
        let mut out_reader = BufReader::new(child.stdout.take().unwrap());
        let output = thread::spawn(move || {
            let mut buf = String::new();
            while let Ok(size) = out_reader.read_line(&mut buf) {
                if size == 0 {
                    break;
                }
                out_sender
                    .send(std::mem::take(&mut buf))
                    .map_err(|e| eyre!(e))
                    .unwrap();
                buf.clear();
            }
        });

        let (err_sender, err_receiver) = crossbeam::channel::unbounded::<String>();
        let mut err_reader = BufReader::new(child.stderr.take().unwrap());
        let error = thread::spawn(move || {
            let mut buf = String::new();
            while let Ok(size) = err_reader.read_line(&mut buf) {
                if size == 0 {
                    break;
                }
                err_sender
                    .send(std::mem::take(&mut buf))
                    .map_err(|e| eyre!(e))
                    .unwrap();
                buf.clear();
            }
        });

        output.join().unwrap();
        error.join().unwrap();

        let result = child.wait_with_output()?;
        match result.status.success() {
            true => Ok(ShellResult::new(in_sender, out_receiver, err_receiver)),
            false => Err(eyre!("run command failed: {}", command_desc)),
        }
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
    use crate::init_backtrace;
    use serde::{Deserialize, Serialize};

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
