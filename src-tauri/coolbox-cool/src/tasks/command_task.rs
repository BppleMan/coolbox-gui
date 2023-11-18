use std::fmt::{Display, Formatter};

use log::info;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{CommandTaskError, TaskError};
use crate::result::CoolResult;
use crate::shell::{Shell, ShellExecutor};
use crate::tasks::{Executable, MessageSender};
use crate::Message;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct CommandTask {
    #[serde(deserialize_with = "crate::template_string")]
    pub script: String,
    #[serde(deserialize_with = "crate::template_envs", default)]
    pub envs: Option<Vec<(String, String)>>,
    pub shell: Shell,
}

impl CommandTask {
    pub fn new(script: String, envs: Option<Vec<(String, String)>>, shell: Shell) -> Self {
        Self {
            script,
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
        write!(f, r#"{} -c "{}""#, self.shell.name(), self.script)?;
        Ok(())
    }
}

impl<'a> Executable<'a> for CommandTask {
    fn execute(&self, mut send: Box<MessageSender<'a>>) -> CoolResult<(), TaskError> {
        info!("{}", self);
        let envs = self.envs.clone();
        let script = self.script.clone();
        let shell = self.shell.clone();

        let (tx1, rx1) = crossbeam::channel::unbounded::<Message>();
        let (tx2, rx2) = crossbeam::channel::bounded(1);
        let task = self.clone();
        rayon::spawn(move || {
            let envs = envs.as_ref().map(|envs| {
                envs.iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect::<Vec<_>>()
            });
            let result = shell.run(&script, envs.as_deref(), Some(tx1)).map_err(|e| {
                TaskError::CommandTaskError {
                    task,
                    source: CommandTaskError::ShellError(e),
                }
            });
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

        let expect = CommandTask::new("echo hello".to_string(), None, Shell::Sh(Sh));
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

        let command = CommandTask::new("ping -c 1 www.baidu.com".to_string(), None, Shell::Sh(Sh));
        command.execute(Box::new(|_| {}))?;
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
        #!/usr/bin/env zsh
        echo first:$1
        echo second:$2
        "#;
        std::fs::write(&script_file, script)?;
        let command = CommandTask::new(
            format!("bash {} hello world", script_file.display()),
            None,
            Shell::Sh(Sh),
        );
        let mut outputs = Vec::new();
        spawn_task(command, |msg| {
            outputs.push(msg.message);
        })?;
        // println!("{}", outputs.as_ref().join("\n"));
        pretty_assertions::assert_eq!("first:hello\nsecond:world".to_string(), outputs.join("\n"));

        let command = CommandTask::new(
            format!("bash -c '{}' -- hello world", script.trim()),
            None,
            Shell::Sh(Sh),
        );
        outputs.clear();
        spawn_task(command, |msg| {
            outputs.push(msg.message);
        })?;
        pretty_assertions::assert_eq!("first:hello\nsecond:world".to_string(), outputs.join("\n"));

        Ok(())
    }
}
