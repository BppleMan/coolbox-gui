use std::process::Command;

use schemars::JsonSchema;

use crate::shell::ShellExecutor;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct LinuxSudo;

impl ShellExecutor for LinuxSudo {
    fn interpreter(&self) -> Command {
        let mut command = Command::new("pkexec");
        command.arg("bash");
        command
    }
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod test {
    use crate::result::CoolResult;
    use crate::shell::{LinuxSudo, ShellExecutor};

    #[test]
    fn test() -> CoolResult<()> {
        color_eyre::install()?;
        let script = reqwest::blocking::get("https://sh.rustup.rs")?.text()?;
        let result = LinuxSudo.run(format!("bash -c \"{}\" --help", script), Some(&[]), None)?;
        println!("{}", result);
        Ok(())
    }
}
