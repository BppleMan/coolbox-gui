use std::process::Command;

use color_eyre::Result;
use schemars::JsonSchema;

use crate::shell::ShellExecutor;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct MacOSSudo;

impl ShellExecutor for MacOSSudo {
    fn interpreter(&self) -> Command {
        let mut command = Command::new("osascript");
        command.arg("-e");
        command
    }

    fn command(&self, script: &str, envs: Option<&[(&str, &str)]>) -> Result<Command> {
        let mut command = self.interpreter();

        command.arg(format!(
            r#"do shell script "bash -c '{}'" with administrator privileges"#,
            script,
        ));
        if let Some(envs) = envs {
            command.envs(envs.to_vec());
        }

        Ok(command)
    }
}

#[cfg(test)]
#[cfg(target_os = "macos")]
#[cfg(feature = "macos-sudo")]
mod test {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::shell::{MacOSSudo, ShellExecutor};

    #[test]
    fn smoke() -> CoolResult<()> {
        init_backtrace();
        let script = reqwest::blocking::get("https://sh.rustup.rs")?.text()?;
        let mut script_file = NamedTempFile::new()?;
        script_file.write_all(script.as_bytes())?;
        let (tx, rx) = crossbeam::channel::unbounded();
        rayon::spawn(move || {
            MacOSSudo
                .run(
                    &format!("bash {} --help", script_file.path().display()),
                    None,
                    Some(tx),
                )
                .unwrap();
        });
        while let Ok(message) = rx.recv() {
            println!("{}", message);
        }
        Ok(())
    }
}
