use crate::error::ExecutableError;
use crate::result::ExecutableResult;
use crate::shell::{Bash, Cmd, ShellExecutor};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    // pub fn set(&self) -> ExecutableResult {
    //     let origin =
    //     Cmd.run(format!("setx {}"))
    // }

    #[cfg(target_os = "macos")]
    pub fn set(&self) -> ExecutableResult {
        let (tx1, rx1) = crossbeam::channel::unbounded();
        let (tx2, rx2) = crossbeam::channel::bounded(1);
        rayon::spawn(move || {
            let result = Bash
                .run("dscl . -read ~/ UserShell", None, Some(tx1))
                .map_err(ExecutableError::ShellError);
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

#[cfg(test)]
mod test {
    use crate::StringExt;
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
