use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[cfg(unix)]
pub use unix_env_util::*;
#[cfg(windows)]
pub use win_env_util::*;

use crate::error::EnvError;
use crate::result::CoolResult;

#[cfg(unix)]
mod unix_env_util;
#[cfg(windows)]
mod win_env_util;

pub struct EnvManager {
    paths: Vec<String>,
    envs: HashMap<String, EnvVariable>,
    #[cfg(unix)]
    source_profiles: HashSet<String>,
}

pub trait EnvManagerBackend {
    fn envs(&self) -> Vec<EnvVariable> {
        std::env::vars()
            .map(|(k, v)| EnvVariable { key: k, value: v })
            .collect()
    }

    fn export(
        &mut self,
        env_var: impl Into<EnvVariable>,
        level: EnvLevel,
    ) -> CoolResult<(), EnvError>;

    fn unset(&mut self, key: impl AsRef<str>, level: EnvLevel) -> CoolResult<(), EnvError>;

    fn append_path(&mut self, value: impl AsRef<str>, level: EnvLevel) -> CoolResult<(), EnvError>;

    fn remove_path(&mut self, value: impl AsRef<str>, level: EnvLevel) -> CoolResult<(), EnvError>;

    #[cfg(unix)]
    fn add_source(&mut self, value: impl AsRef<str>, level: EnvLevel) -> CoolResult<(), EnvError>;

    #[cfg(unix)]
    fn remove_source(
        &mut self,
        value: impl AsRef<str>,
        level: EnvLevel,
    ) -> CoolResult<(), EnvError>;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum EnvLevel {
    Process,
    User,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct EnvVariable {
    pub key: String,
    pub value: String,
}

impl TryFrom<&str> for EnvVariable {
    type Error = EnvError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split_items = value.split('=').collect::<Vec<&str>>();
        match split_items.len() {
            2 => Ok(EnvVariable {
                key: split_items[0].to_string(),
                value: split_items[1].to_string(),
            }),
            _ => Err(EnvError::InvalidEnvVar(value.to_string())),
        }
    }
}

impl<T, U> From<(T, U)> for EnvVariable
where
    T: Into<String>,
    U: Into<String>,
{
    fn from(value: (T, U)) -> Self {
        EnvVariable {
            key: value.0.into(),
            value: value.1.into(),
        }
    }
}

impl<T, U> From<[T; 2]> for EnvVariable
where
    T: ToOwned<Owned = U>,
    U: Into<String>,
{
    fn from(value: [T; 2]) -> Self {
        EnvVariable {
            key: value[0].to_owned().into(),
            value: value[1].to_owned().into(),
        }
    }
}
