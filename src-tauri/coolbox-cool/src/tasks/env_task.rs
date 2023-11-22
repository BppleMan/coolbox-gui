use crate::env_manager::{
    render_env_var, render_path, render_source, EnvLevel, EnvManager, EnvVar, COOL_PROFILE,
};
use crate::error::{EnvTaskError, TaskError};
use crate::login_shell::LoginShell;
use crate::result::CoolResult;
use crate::tasks::Executable;
use crate::{IntoInfo, MessageSender};
#[allow(unused_imports)]
use bitflags::Flags;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct EnvTask {
    pub command: EnvCommand,
}

impl EnvTask {
    pub fn new(command: EnvCommand) -> Self {
        Self { command }
    }
}

impl<'a> Executable<'a> for EnvTask {
    fn execute(&self, mut send: Box<MessageSender<'a>>) -> CoolResult<(), TaskError> {
        let mut env_manager = EnvManager {
            #[cfg(unix)]
            profile: unsafe { COOL_PROFILE.deref_mut() },
        };
        let _ = match &self.command {
            EnvCommand::Export(env_var) => {
                send(format!("export {}", env_var).into_info());
                env_manager.export(env_var.clone(), EnvLevel::all())
            }
            EnvCommand::Unset(value) => {
                send(format!("unset {}", value).into_info());
                env_manager.unset(value, EnvLevel::all())
            }
            EnvCommand::AppendPath(value) => {
                send(format!("append path {}", value).into_info());
                env_manager.append_path(value, EnvLevel::all())
            }
            EnvCommand::RemovePath(value) => {
                send(format!("remove path {}", value).into_info());
                env_manager.remove_path(value, EnvLevel::all())
            }
            #[cfg(unix)]
            EnvCommand::AddSource(value) => {
                send(format!("add source {}", value).into_info());
                env_manager.add_source(
                    value,
                    Some(
                        &LoginShell::detect()
                            .map_err(EnvTaskError::from)
                            .map_err(|e| TaskError::EnvTaskError {
                                task: self.clone(),
                                source: e,
                            })?,
                    ),
                )
            }
            #[cfg(unix)]
            EnvCommand::RemoveSource(value) => {
                send(format!("remove source {}", value).into_info());
                env_manager.remove_source(value)
            }
        };
        Ok(())
    }
}

impl Display for EnvTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum EnvCommand {
    Export(EnvVar),
    Unset(String),
    AppendPath(String),
    RemovePath(String),
    #[cfg(unix)]
    AddSource(String),
    #[cfg(unix)]
    RemoveSource(String),
}

impl Display for EnvCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvCommand::Export(env_var) => {
                if cfg!(unix) {
                    writeln!(
                        f,
                        "# will append the lines into [{}]",
                        unsafe { COOL_PROFILE.deref() }.file_path().display()
                    )?;
                    write!(f, "{}", render_env_var(env_var))
                } else {
                    writeln!(
                        f,
                        "# will add the env var into the user environment variables"
                    )?;
                    write!(f, "{}", env_var)
                }
            }
            EnvCommand::Unset(value) => {
                if cfg!(unix) {
                    writeln!(
                        f,
                        "# will delete the lines from [{}]",
                        unsafe { COOL_PROFILE.deref() }.file_path().display()
                    )?;
                    write!(
                        f,
                        "{}",
                        render_env_var(&EnvVar::new(value, "Any Value").unwrap())
                    )
                } else {
                    writeln!(
                        f,
                        "# will delete the env var from the user environment variables"
                    )?;
                    write!(f, "{}", value)
                }
            }
            EnvCommand::AppendPath(value) => {
                if cfg!(unix) {
                    writeln!(
                        f,
                        "# will append the lines into [{}]",
                        unsafe { COOL_PROFILE.deref() }.file_path().display()
                    )?;
                    write!(f, "{}", render_path(value))
                } else {
                    writeln!(
                        f,
                        "# will append the value into the user environment %PATH% variables"
                    )?;
                    write!(f, "{}", value)
                }
            }
            EnvCommand::RemovePath(value) => {
                if cfg!(unix) {
                    writeln!(
                        f,
                        "# will delete the lines from [{}]",
                        unsafe { COOL_PROFILE.deref() }.file_path().display()
                    )?;
                    write!(f, "{}", render_path(value))
                } else {
                    writeln!(
                        f,
                        "# will delete the value from the user environment %PATH% variables"
                    )?;
                    write!(f, "{}", value)
                }
            }
            #[cfg(unix)]
            EnvCommand::AddSource(value) => {
                writeln!(
                    f,
                    "# will append the lines into [{}]",
                    unsafe { COOL_PROFILE.deref() }.file_path().display()
                )?;
                write!(f, "{}", render_source(value))
            }
            #[cfg(unix)]
            EnvCommand::RemoveSource(value) => {
                writeln!(
                    f,
                    "# will delete the lines from [{}]",
                    unsafe { COOL_PROFILE.deref() }.file_path().display()
                )?;
                write!(f, "{}", render_source(value))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::env_manager::EnvVar;
    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::tasks::{spawn_task, EnvCommand, EnvTask};

    #[test]
    fn env_task_smoke() -> CoolResult<()> {
        init_backtrace();
        let task = EnvTask::new(EnvCommand::Export(EnvVar::new("COOL_TEST", "1")?));
        spawn_task(task, |_| {})?;
        pretty_assertions::assert_str_eq!("1", std::env::var("COOL_TEST")?);
        Ok(())
    }
}
