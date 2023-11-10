use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use color_eyre::Result;

use crate::shell::ShellExecutor;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacOSSudo;

impl ShellExecutor for MacOSSudo {
    fn interpreter(&self) -> Command {
        let mut command = Command::new("osascript");
        command.arg("-e");
        command
    }

    fn command(&self, cmd: &str, args: Option<&[&str]>) -> Result<Command> {
        let mut command = self.interpreter();

        let mut content = "sh".to_string();
        if !PathBuf::from_str(cmd)?.exists() {
            content += " -c";
        }
        match args {
            None => content += &format!(" {}", cmd),
            Some(args) => content += &format!(" {} -- {}", cmd, args.join(" ")),
        }
        command.arg(format!(
            r#"do shell script "{}" with administrator privileges"#,
            content,
        ));

        Ok(command)
    }
}

#[cfg(test)]
#[cfg(target_os = "macos")]
#[cfg(feature = "macos-sudo")]
mod test {
    use std::io::Write;

    use crate::init_backtrace;
    use tempfile::NamedTempFile;

    use crate::result::CoolResult;
    use crate::shell::{MacOSSudo, ShellExecutor};

    #[test]
    fn test() -> CoolResult<()> {
        init_backtrace();
        let script = reqwest::blocking::get("https://sh.rustup.rs")?.text()?;
        let mut script_file = NamedTempFile::new()?;
        script_file.write_all(script.as_bytes())?;
        MacOSSudo.run(
            &format!("{}", script_file.path().display()),
            Some(&["-h"]),
            None,
            None,
        )?;
        Ok(())
    }
}
