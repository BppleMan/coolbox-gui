use bitflags::{bitflags, Flags};
use serde::{Deserialize, Serialize};
use std::ops::DerefMut;

#[cfg(unix)]
pub use shell_profile::*;
#[cfg(windows)]
pub use win_env_util::*;

use crate::error::EnvError;
use crate::login_shell::LoginShell;
use crate::result::CoolResult;

#[cfg(unix)]
mod shell_profile;
#[cfg(windows)]
mod win_env_util;

pub struct EnvManager;

impl EnvManager {
    pub fn envs(&self) -> Vec<EnvVariable> {
        std::env::vars()
            .map(|(k, v)| EnvVariable { key: k, value: v })
            .collect()
    }

    pub fn export(
        &mut self,
        env_var: impl Into<EnvVariable>,
        level: EnvLevel,
    ) -> CoolResult<(), EnvError> {
        let env_var = env_var.into();
        if level.contains(EnvLevel::Process) {
            let env_var = env_var.clone();
            std::env::set_var(env_var.key, env_var.value);
        }
        if level.contains(EnvLevel::User) {
            #[cfg(unix)]
            COOL_PROFILE.lock().unwrap().export(env_var)?;
        }
        Ok(())
    }

    pub fn unset(&mut self, key: impl AsRef<str>, level: EnvLevel) -> CoolResult<(), EnvError> {
        let key = key.as_ref();
        if key.is_empty() {
            return Err(EnvError::EmptyKey);
        }
        if level.contains(EnvLevel::Process) {
            std::env::remove_var(key);
        }
        if level.contains(EnvLevel::User) {
            #[cfg(unix)]
            COOL_PROFILE.lock().unwrap().unset(key)?;
        }
        Ok(())
    }

    pub fn append_path(
        &mut self,
        value: impl AsRef<str>,
        level: EnvLevel,
    ) -> CoolResult<(), EnvError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(EnvError::EmptyPathValue);
        }
        if level.contains(EnvLevel::Process) {
            let path = std::env::var("PATH").unwrap_or_default();
            let path = format!("{}:{}", value, path);
            std::env::set_var("PATH", path);
        }
        if level.contains(EnvLevel::User) {
            #[cfg(unix)]
            COOL_PROFILE.lock().unwrap().append_path(value)?;
        }
        Ok(())
    }

    pub fn remove_path(
        &mut self,
        value: impl AsRef<str>,
        level: EnvLevel,
    ) -> CoolResult<(), EnvError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(EnvError::EmptyPathValue);
        }
        if level.contains(EnvLevel::Process) {
            let path = std::env::var("PATH").unwrap_or_default();
            let path = path
                .split(':')
                .filter(|p| p != &value)
                .collect::<Vec<&str>>()
                .join(":");
            std::env::set_var("PATH", path);
        }
        if level.contains(EnvLevel::User) {
            #[cfg(unix)]
            COOL_PROFILE.lock().unwrap().remove_path(value)?;
        }
        Ok(())
    }

    #[cfg(unix)]
    pub fn add_source(&mut self, value: impl AsRef<str>) -> CoolResult<(), EnvError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(EnvError::EmptySourceValue);
        }
        #[cfg(unix)]
        COOL_PROFILE.lock().unwrap().add_source(value)
    }

    #[cfg(unix)]
    pub fn remove_source(&mut self, value: impl AsRef<str>) -> CoolResult<(), EnvError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(EnvError::EmptySourceValue);
        }
        #[cfg(unix)]
        COOL_PROFILE.lock().unwrap().remove_source(value)
    }

    #[cfg(unix)]
    pub fn source_profile<'a>(
        &mut self,
        login_shell: &LoginShell,
        profile: impl Into<&'a str>,
    ) -> CoolResult<(), EnvError> {
        let envs = ShellProfile::profile_envs(login_shell, profile, false)?;
        envs.into_iter()
            .for_each(|e| self.export(e, EnvLevel::Process).unwrap());
        Ok(())
    }
}

pub trait EnvManagerBackend {
    fn export(&mut self, env_var: impl Into<EnvVariable>) -> CoolResult<(), EnvError>;

    fn unset(&mut self, key: impl AsRef<str>) -> CoolResult<(), EnvError>;

    fn append_path(&mut self, value: impl AsRef<str>) -> CoolResult<(), EnvError>;

    fn remove_path(&mut self, value: impl AsRef<str>) -> CoolResult<(), EnvError>;

    #[cfg(unix)]
    fn add_source(&mut self, value: impl AsRef<str>) -> CoolResult<(), EnvError>;

    #[cfg(unix)]
    fn remove_source(&mut self, value: impl AsRef<str>) -> CoolResult<(), EnvError>;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct EnvLevel(u32);

bitflags! {
    impl EnvLevel: u32 {
        const Process = 0b00000001;
        const User = 0b00000010;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct EnvVariable {
    pub key: String,
    pub value: String,
}

impl EnvVariable {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> CoolResult<Self, EnvError> {
        let key = key.into();
        if key.is_empty() {
            return Err(EnvError::EmptyKey);
        }
        let value = value.into();
        if value.is_empty() {
            return Err(EnvError::EmptyValue);
        }
        Ok(Self { key, value })
    }
}

impl TryFrom<&str> for EnvVariable {
    type Error = EnvError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(EnvError::InvalidEnvVar(value.to_string()));
        }
        let split_items = value.split('=').collect::<Vec<&str>>();
        match split_items.len() {
            2 => EnvVariable::new(split_items[0].to_string(), split_items[1].to_string()),
            _ => Err(EnvError::InvalidEnvVar(value.to_string())),
        }
    }
}

impl<T, U> TryFrom<(T, U)> for EnvVariable
where
    T: Into<String>,
    U: Into<String>,
{
    type Error = EnvError;

    fn try_from(value: (T, U)) -> Result<Self, Self::Error> {
        EnvVariable::new(value.0, value.1)
    }
}

impl<T, U> TryFrom<[T; 2]> for EnvVariable
where
    T: ToOwned<Owned = U>,
    U: Into<String>,
{
    type Error = EnvError;

    fn try_from(value: [T; 2]) -> Result<Self, Self::Error> {
        EnvVariable::new(value[0].to_owned(), value[1].to_owned())
    }
}

#[cfg(test)]
mod test {
    use crate::env_manager::{EnvLevel, EnvManager, EnvVariable, COOL_PROFILE};
    use crate::init_backtrace;
    use crate::local_storage::LocalStorage;
    use crate::result::CoolResult;
    use std::ops::Deref;

    #[cfg(unix)]
    #[test]
    fn smoke() -> CoolResult<()> {
        init_backtrace();
        assert!(std::env::var("COOL_TEST").is_err());
        EnvManager.export(
            EnvVariable::try_from(["COOL_TEST", "1"])?,
            EnvLevel::Process,
        )?;
        pretty_assertions::assert_str_eq!("1".to_string(), std::env::var("COOL_TEST").unwrap());

        EnvManager.export(
            EnvVariable::try_from(["COOL_TEST", "2"])?,
            EnvLevel::Process | EnvLevel::User,
        )?;
        pretty_assertions::assert_str_eq!("2".to_string(), std::env::var("COOL_TEST").unwrap());

        let cool_profile = LocalStorage.cool_profile();
        pretty_assertions::assert_eq!(COOL_PROFILE.lock().unwrap().deref(), &cool_profile);
        Ok(())
    }
}
