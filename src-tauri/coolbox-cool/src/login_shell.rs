use crate::shell::ShellExecutor;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::error::ShellError;
use crate::USER_DIRS;
use color_eyre::eyre::eyre;

use crate::result::CoolResult;
use crate::shell::{Bash, Shell, Zsh};

#[derive(Debug)]
pub struct LoginShell {
    pub shell: Shell,
    pub user_profile: PathBuf,
}

impl LoginShell {
    pub fn detect() -> CoolResult<Self, ShellError> {
        let shell = Self::detect_shell()?;
        let user_profile = Self::detect_profile(&shell);
        Ok(Self {
            shell,
            user_profile,
        })
    }

    #[cfg(target_os = "macos")]
    fn detect_shell() -> CoolResult<Shell, ShellError> {
        let result = Command::new("dscl")
            .args([
                ".",
                "-read",
                format!("{}/", USER_DIRS.home_dir().display()).as_str(),
                "UserShell",
            ])
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| ShellError {
                shell: "dscl".to_string(),
                script: format!(". -read {}/ UserShell", USER_DIRS.home_dir().display()),
                envs: None,
                inner_error: Some(e.into()),
            })?
            .wait_with_output()
            .map_err(|e| ShellError {
                shell: "dscl".to_string(),
                script: format!(". -read {}/ UserShell", USER_DIRS.home_dir().display()),
                envs: None,
                inner_error: Some(e.into()),
            })?;
        let ls = String::from_utf8(result.stdout).unwrap();
        if ls.trim().ends_with("zsh") {
            Ok(Shell::Zsh(Zsh))
        } else if ls.trim().ends_with("bash") {
            Ok(Shell::Bash(Bash))
        } else {
            Err(ShellError {
                shell: "dscl".to_string(),
                script: format!(". -read {}/ UserShell", USER_DIRS.home_dir().display()),
                envs: None,
                inner_error: Some(eyre!("Unsupported login shell: {}", ls.trim())),
            })
        }
    }

    #[cfg(target_os = "linux")]
    fn detect_shell() -> CoolResult<Shell> {
        let result = Command::new("getent")
            .args(["passwd", env::var("LOGNAME")?.as_str()])
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?;
        let ls = String::from_utf8(result.stdout)?;
        if ls.ends_with("zsh") {
            Ok(Shell::Zsh(Zsh))
        } else if ls.ends_with("bash") {
            Ok(Shell::Bash(Bash))
        } else {
            Err(eyre!("Unsupported login shell: {}", ls))
        }
    }

    fn detect_profile(shell: &Shell) -> PathBuf {
        match &shell {
            Shell::Bash(_) => USER_DIRS.home_dir().join(".bashrc"),
            Shell::Zsh(_) => USER_DIRS.home_dir().join(".zshrc"),
            _ => panic!("{}", eyre!("Unsupported login shell: {}", shell.name())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::LoginShell;

    #[test]
    fn test() {
        let shell = LoginShell::detect().unwrap();
        println!("{:?}", shell);
    }
}
