use std::fmt::{Display, Formatter};

use log::info;
use serde::{Deserialize, Serialize};

use crate::error::ExecutableError;
use crate::result::ExecutableResult;
use crate::shell::{Shell, ShellExecutor};
use crate::tasks::{Executable, ExecutableSender};
use crate::ExecutableMessage;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandTask {
    #[serde(deserialize_with = "crate::render_str")]
    pub script: String,
    pub args: Option<Vec<String>>,
    pub envs: Option<Vec<(String, String)>>,
    pub shell: Shell,
}

impl CommandTask {
    pub fn new(
        script: String,
        args: Option<Vec<String>>,
        envs: Option<Vec<(String, String)>>,
        shell: Shell,
    ) -> Self {
        Self {
            script,
            args,
            envs,
            shell,
        }
    }
}

impl Display for CommandTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(envs) = self.envs.as_ref() {
            for (k, v) in envs {
                write!(f, "{}={} ", k, v)?;
            }
        }
        match self.shell {
            Shell::Bash(_) => write!(f, "bash"),
            Shell::LinuxSudo(_) => write!(f, "sudo"),
            Shell::MacOSSudo(_) => write!(f, "sudo"),
            Shell::Sh(_) => write!(f, "sh"),
            Shell::Zsh(_) => write!(f, "zsh"),
        }?;
        write!(f, " {}", self.script)?;
        if let Some(args) = self.args.as_ref() {
            for arg in args {
                write!(f, " {}", arg)?;
            }
        }
        Ok(())
    }
}

impl<'a> Executable<'a> for CommandTask {
    fn _run(&self, mut send: Box<ExecutableSender<'a>>) -> ExecutableResult {
        info!("{}", self);
        let args = self
            .args
            .as_ref()
            .map(|args| args.iter().map(AsRef::as_ref).collect::<Vec<_>>());
        let envs = self.envs.as_ref().map(|envs| {
            envs.iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect::<Vec<_>>()
        });

        let (tx1, rx1) = crossbeam::channel::unbounded::<ExecutableMessage>();
        let (tx2, rx2) = crossbeam::channel::bounded(1);
        rayon::scope(|s| {
            s.spawn(|_| {
                let result = self
                    .shell
                    .run(&self.script, args.as_deref(), envs.as_deref(), Some(tx1))
                    .map_err(ExecutableError::ShellError);
                tx2.send(result).unwrap();
            });
        });
        while let Ok(message) = rx1.recv() {
            send(message);
        }

        rx2.recv().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::shell::{Sh, Shell};
    use crate::tasks::{spawn_task, CommandTask, Executable};

    #[test]
    fn test_serialize() -> CoolResult<()> {
        init_backtrace();

        let expect = CommandTask::new("echo hello".to_string(), None, None, Shell::Sh(Sh));
        let toml = toml::to_string(&expect)?;
        let command: CommandTask = toml::from_str(&toml)?;
        pretty_assertions::assert_eq!(expect, command);

        let mut outputs = String::new();
        command.execute(Box::new(|msg| {
            outputs.push_str(&msg.message);
        }))?;
        pretty_assertions::assert_eq!("hello\n".to_string(), outputs);
        Ok(())
    }

    #[test]
    fn ping() -> CoolResult<()> {
        init_backtrace();

        let command = CommandTask::new(
            "ping -c 1 www.baidu.com".to_string(),
            None,
            None,
            Shell::Sh(Sh),
        );
        command.execute(Box::new(|_| {}))?;
        // spawn_task(command, |_| {})?;
        Ok(())
    }

    #[test]
    fn run_script_file() -> CoolResult<()> {
        init_backtrace();
        let base_dir = tempfile::Builder::new()
            .prefix("cool")
            .suffix("command")
            .tempdir()?;
        let script_file = base_dir.path().join("script.sh");
        let script = r#"
        #!/bin/env zsh
        echo first:$1
        echo second:$2
        "#;
        std::fs::write(&script_file, script)?;
        let command = CommandTask::new(
            script_file.to_string_lossy().to_string(),
            Some(vec!["hello".to_string(), "world".to_string()]),
            None,
            Shell::Sh(Sh),
        );
        let mut outputs = String::new();
        spawn_task(command, |msg| {
            outputs.push_str(&msg.message);
        })?;
        pretty_assertions::assert_eq!("first:hello\nsecond:world\n".to_string(), outputs);

        let command = CommandTask::new(
            script.to_string(),
            Some(vec!["hello".to_string(), "world".to_string()]),
            None,
            Shell::Sh(Sh),
        );
        outputs.clear();
        spawn_task(command, |msg| {
            outputs.push_str(&msg.message);
        })?;
        pretty_assertions::assert_eq!("first:hello\nsecond:world\n".to_string(), outputs,);

        Ok(())
    }
}
