use bitflags::bitflags;
#[allow(unused_imports)]
use bitflags::Flags;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[cfg(unix)]
pub use shell_profile::*;
#[cfg(windows)]
pub use win_env_util::*;

use crate::error::EnvError;
use crate::result::CoolResult;
use crate::LoginShell;

#[cfg(unix)]
mod shell_profile;
#[cfg(windows)]
mod win_env_util;

pub struct EnvManager<'a> {
    #[cfg(unix)]
    pub profile: &'a mut ShellProfile,
}

impl<'a> EnvManager<'a> {
    pub fn envs(&self) -> Vec<EnvVar> {
        std::env::vars()
            .map(|(k, v)| EnvVar { key: k, value: v })
            .collect()
    }

    pub fn export(
        &mut self,
        env_var: impl Into<EnvVar>,
        level: EnvLevel,
    ) -> CoolResult<(), EnvError> {
        let env_var = env_var.into();
        if level.contains(EnvLevel::Process) {
            let env_var = env_var.clone();
            std::env::set_var(env_var.key, env_var.value);
        }
        if level.contains(EnvLevel::User) {
            #[cfg(unix)]
            self.profile.export(env_var)?;
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
            self.profile.unset(key)?;
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
            self.profile.append_path(value)?;
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
            self.profile.remove_path(value)?;
        }
        Ok(())
    }

    #[cfg(unix)]
    pub fn add_source(
        &mut self,
        value: impl AsRef<str>,
        login_shell: Option<&LoginShell>,
    ) -> CoolResult<(), EnvError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(EnvError::EmptySourceValue);
        }
        self.profile.add_source(value)?;
        if let Some(login_shell) = login_shell {
            self.source_profile(login_shell, value)?;
        }
        Ok(())
    }

    #[cfg(unix)]
    pub fn remove_source(&mut self, value: impl AsRef<str>) -> CoolResult<(), EnvError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(EnvError::EmptySourceValue);
        }
        self.profile.remove_source(value)
    }

    #[cfg(unix)]
    pub fn source_profile<'b>(
        &mut self,
        login_shell: &LoginShell,
        profile: impl Into<&'b str>,
    ) -> CoolResult<(), EnvError> {
        let envs = ShellProfile::profile_envs(login_shell, profile, false)?;
        envs.into_iter()
            .for_each(|e| self.export(e, EnvLevel::Process).unwrap());
        Ok(())
    }
}

pub trait EnvManagerBackend {
    fn export(&mut self, env_var: impl Into<EnvVar>) -> CoolResult<(), EnvError>;

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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

impl EnvVar {
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

impl TryFrom<&str> for EnvVar {
    type Error = EnvError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(EnvError::InvalidEnvVar(value.to_string()));
        }
        let split_items = value.split('=').collect::<Vec<&str>>();
        match split_items.len() {
            2 => EnvVar::new(split_items[0].to_string(), split_items[1].to_string()),
            _ => Err(EnvError::InvalidEnvVar(value.to_string())),
        }
    }
}

impl<T, U> TryFrom<(T, U)> for EnvVar
where
    T: Into<String>,
    U: Into<String>,
{
    type Error = EnvError;

    fn try_from(value: (T, U)) -> Result<Self, Self::Error> {
        EnvVar::new(value.0, value.1)
    }
}

impl<T, U> TryFrom<[T; 2]> for EnvVar
where
    T: ToOwned<Owned = U>,
    U: Into<String>,
{
    type Error = EnvError;

    fn try_from(value: [T; 2]) -> Result<Self, Self::Error> {
        EnvVar::new(value[0].to_owned(), value[1].to_owned())
    }
}

impl Display for EnvVar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

#[cfg(test)]
mod test {
    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::LOCAL_STORAGE;
    use crate::{EnvLevel, EnvManager, EnvVar, COOL_PROFILE};
    use std::ops::{Deref, DerefMut};

    #[cfg(unix)]
    #[test]
    fn smoke_unix() -> CoolResult<()> {
        init_backtrace();
        assert!(std::env::var("COOL_TEST").is_err());
        let mut env_manager = EnvManager {
            profile: unsafe { COOL_PROFILE.deref_mut() },
        };
        env_manager.export(
            EnvVar::try_from(["COOL_TEST", "1"])?,
            EnvLevel::Process | EnvLevel::User,
        )?;
        pretty_assertions::assert_str_eq!("1".to_string(), std::env::var("COOL_TEST").unwrap());
        let cool_profile = LOCAL_STORAGE.cool_profile();
        pretty_assertions::assert_eq!(unsafe { COOL_PROFILE.deref() }, &cool_profile);

        let origin_path = std::env::var("PATH")?;
        env_manager.append_path("/tmp", EnvLevel::Process | EnvLevel::User)?;
        pretty_assertions::assert_str_eq!(
            format!("/tmp:{}", origin_path),
            std::env::var("PATH").unwrap()
        );
        let cool_profile = LOCAL_STORAGE.cool_profile();
        pretty_assertions::assert_eq!(unsafe { COOL_PROFILE.deref() }, &cool_profile);

        env_manager.remove_path("/tmp", EnvLevel::Process | EnvLevel::User)?;
        pretty_assertions::assert_str_eq!(origin_path, std::env::var("PATH").unwrap());
        let cool_profile = LOCAL_STORAGE.cool_profile();
        pretty_assertions::assert_eq!(unsafe { COOL_PROFILE.deref() }, &cool_profile);
        Ok(())
    }
}
