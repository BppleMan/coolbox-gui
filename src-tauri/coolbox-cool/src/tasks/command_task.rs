use std::fmt::{Display, Formatter};

use color_eyre::eyre::eyre;
use log::info;
use serde::{Deserialize, Serialize};

use crate::error::ExecutableError;
use crate::result::ExecutableResult;
use crate::shell::{Shell, ShellExecutor, ShellResult};
use crate::tasks::{Executable, ExecutableSender};

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

impl Executable for CommandTask {
    fn _run(&mut self, sender: &ExecutableSender) -> ExecutableResult {
        info!("{}", self);
        let initial_result = Err(ExecutableError::ShellError(eyre!("No attempts made")));

        (0..5).fold(initial_result, |acc, _| {
            if acc.is_err() {
                let ShellResult {
                    input: _input,
                    output,
                    error,
                } = self
                    .shell
                    .run(
                        &self.script,
                        self.args
                            .as_ref()
                            .map(|args| args.iter().map(AsRef::as_ref).collect::<Vec<_>>())
                            .as_deref(),
                        self.envs
                            .as_ref()
                            .map(|envs| {
                                envs.iter()
                                    .map(|(k, v)| (k.as_str(), v.as_str()))
                                    .collect::<Vec<_>>()
                            })
                            .as_deref(),
                    )
                    .map_err(ExecutableError::ShellError)?;
                rayon::scope(|s| {
                    s.spawn(|_| {
                        while let Ok(r) = output.recv() {
                            sender.outputs.send(r).unwrap();
                        }
                    });
                    s.spawn(|_| {
                        while let Ok(r) = error.recv() {
                            sender.errors.send(r).unwrap();
                        }
                    });
                });
            }
            Ok(())
        })
    }
}

#[cfg(test)]
mod test {
    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::shell::{Sh, Shell};
    use crate::tasks::{CommandTask, Executable};

    #[test]
    fn test_serialize() -> CoolResult<()> {
        init_backtrace();

        let mut expect = CommandTask::new("echo hello".to_string(), None, None, Shell::Sh(Sh));
        let toml = toml::to_string(&expect)?;
        let command: CommandTask = toml::from_str(&toml)?;
        pretty_assertions::assert_eq!(expect, command);

        expect.execute()?;
        pretty_assertions::assert_eq!("hello\n".to_string(), expect.outputs.join("\n"));
        Ok(())
    }

    #[test]
    fn ping() -> CoolResult<()> {
        init_backtrace();

        let mut command = CommandTask::new(
            "ping -c 1 www.baidu.com".to_string(),
            None,
            None,
            Shell::Sh(Sh),
        );
        let result = command.execute();
        assert!(result.is_ok());
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
        let mut command = CommandTask::new(
            script_file.to_string_lossy().to_string(),
            Some(vec!["hello".to_string(), "world".to_string()]),
            None,
            Shell::Sh(Sh),
        );
        command.execute()?;
        pretty_assertions::assert_eq!(
            "first:hello\nsecond:world\n".to_string(),
            command.outputs.join("")
        );

        let mut command = CommandTask::new(
            script.to_string(),
            Some(vec!["hello".to_string(), "world".to_string()]),
            None,
            Shell::Sh(Sh),
        );
        command.execute()?;
        pretty_assertions::assert_eq!(
            "first:hello\nsecond:world\n".to_string(),
            command.outputs.join("")
        );

        Ok(())
    }
}
