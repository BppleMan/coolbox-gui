use color_eyre::eyre::eyre;
use color_eyre::Report;
use serde::{Deserialize, Serialize};

#[cfg(unix)]
mod unix_env_util;
#[cfg(windows)]
mod win_env_util;

#[cfg(unix)]
pub use unix_env_util::*;
#[cfg(windows)]
pub use win_env_util::*;

pub trait EnvUtil {
    fn envs(&self) -> Vec<EnvVar>;
    fn export(&self, env_var: impl Into<EnvVar>);
    fn unset(&self, key: impl AsRef<str>);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

impl TryFrom<&str> for EnvVar {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split_items = value.split("=").collect::<Vec<&str>>();
        match split_items.len() {
            2 => Ok(EnvVar {
                key: split_items[0].to_string(),
                value: split_items[1].to_string(),
            }),
            _ => Err(eyre!("invalid env var: {}", value)),
        }
    }
}

impl<T, U> From<(T, U)> for EnvVar
where
    T: Into<String>,
    U: Into<String>,
{
    fn from(value: (T, U)) -> Self {
        EnvVar {
            key: value.0.into(),
            value: value.1.into(),
        }
    }
}

impl<T, U> From<[T; 2]> for EnvVar
where
    T: ToOwned<Owned = U>,
    U: Into<String>,
{
    fn from(value: [T; 2]) -> Self {
        EnvVar {
            key: value[0].to_owned().into(),
            value: value[1].to_owned().into(),
        }
    }
}
