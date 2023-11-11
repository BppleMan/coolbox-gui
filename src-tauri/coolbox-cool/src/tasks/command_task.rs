use std::fmt::{Display, Formatter};

use log::info;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ExecutableError;
use crate::result::ExecutableResult;
use crate::shell::{Shell, ShellExecutor};
use crate::tasks::{Executable, MessageSender};
use crate::Message;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct CommandTask {
    #[serde(deserialize_with = "crate::template_string")]
    pub script: String,
    #[serde(deserialize_with = "crate::template_args", default)]
    pub args: Option<Vec<String>>,
    #[serde(deserialize_with = "crate::template_envs", default)]
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
    fn execute(&self, mut send: Box<MessageSender<'a>>) -> ExecutableResult {
        info!("{}", self);
        let args = self.args.clone();
        let envs = self.envs.clone();
        let script = self.script.clone();
        let shell = self.shell.clone();

        let (tx1, rx1) = crossbeam::channel::unbounded::<Message>();
        let (tx2, rx2) = crossbeam::channel::bounded(1);
        rayon::spawn(move || {
            let args = args
                .as_ref()
                .map(|args| args.iter().map(AsRef::as_ref).collect::<Vec<_>>());
            let envs = envs.as_ref().map(|envs| {
                envs.iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect::<Vec<_>>()
            });
            let result = shell
                .run(&script, args.as_deref(), envs.as_deref(), Some(tx1))
                .map_err(ExecutableError::ShellError);
            tx2.send(result).unwrap();
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
