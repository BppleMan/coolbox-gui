use crate::error::{EnvTaskError, TaskError};
use crate::result::CoolResult;
use crate::shell::{Bash, ShellExecutor};
use crate::tasks::Executable;
use crate::MessageSender;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct EnvTask {
    pub key: String,
    pub value: String,
}

impl EnvTask {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }

    // #[cfg(target_os = "windows")]
    // pub fn set(&self) -> CoolResult<(), ExecutableError> {
    //     let origin =
    //     Cmd.run(format!("setx {}"))
    // }

    #[cfg(target_os = "macos")]
    pub fn set(&self) -> CoolResult<(), TaskError> {
        let (tx1, rx1) = crossbeam::channel::unbounded();
        let (tx2, rx2) = crossbeam::channel::bounded(1);
        let task = self.clone();
        rayon::spawn(move || {
            let result = Bash
                .run("dscl . -read ~/ UserShell", None, Some(tx1))
                .map_err(|e| TaskError::EnvTaskError {
                    task,
                    source: EnvTaskError::ShellError(e),
                });
            tx2.send(result).unwrap();
        });
        let mut messages = vec![];
        while let Ok(message) = rx1.recv() {
            messages.push(message.message);
        }
        let login_shell = messages.join("");
        Ok(())
    }

    pub fn detect_shell(login_shell: String) {
        let shell = if login_shell.contains("zsh") {
            "zsh"
        } else if login_shell.contains("bash") {
            "bash"
        } else {
            "sh"
        };
        println!("login shell: {}", shell);
    }
}

impl Display for EnvTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a> Executable<'a> for EnvTask {
    fn execute(&self, send: Box<MessageSender<'a>>) -> CoolResult<(), TaskError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::env;
    use std::process::Command;

    #[test]
    fn test() {
        println!("{:?}", env::current_dir());
        Command::new("zsh")
            .arg("-c")
            .arg("source ../coolrc && env")
            .env_clear()
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
