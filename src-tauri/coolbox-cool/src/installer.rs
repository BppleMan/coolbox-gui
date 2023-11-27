use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

use crossbeam::channel::Sender;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::ShellError;
use crate::Message;
pub use apt::*;
pub use brew::*;
pub use cargo::*;
pub use dnf::*;
pub use rpm::*;
pub use winget::*;
pub use yum::*;

use crate::result::CoolResult;

mod apt;
mod brew;
mod cargo;
mod dnf;
mod rpm;
mod winget;
mod yum;

pub trait Installable {
    fn name(&self) -> &'static str;

    fn install(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<(), ShellError>;

    fn uninstall(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<(), ShellError>;

    fn check_available(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
    ) -> CoolResult<bool, ShellError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub enum Installer {
    Brew(Brew),
    Cargo(Cargo),
    Apt(Apt),
    Yun(Yum),
    Dnf(Dnf),
    Rpm(Rpm),
    WinGet(WinGet),
}

impl Installer {
    pub fn brew() -> Self {
        Self::Brew(Brew)
    }

    pub fn cargo() -> Self {
        Self::Cargo(Cargo)
    }

    pub fn apt() -> Self {
        Self::Apt(Apt)
    }

    pub fn yum() -> Self {
        Self::Yun(Yum)
    }

    pub fn dnf() -> Self {
        Self::Dnf(Dnf)
    }

    pub fn rpm() -> Self {
        Self::Rpm(Rpm)
    }

    pub fn win_get() -> Self {
        Self::WinGet(WinGet)
    }
}

impl AsRef<dyn Installable> for Installer {
    fn as_ref(&self) -> &(dyn Installable + 'static) {
        match self {
            Installer::Apt(apt) => apt,
            Installer::Brew(brew) => brew,
            Installer::Cargo(cargo) => cargo,
            Installer::Yun(yum) => yum,
            Installer::Dnf(dnf) => dnf,
            Installer::Rpm(rpm) => rpm,
            Installer::WinGet(win_get) => win_get,
        }
    }
}

impl AsMut<dyn Installable> for Installer {
    fn as_mut(&mut self) -> &mut (dyn Installable + 'static) {
        match self {
            Installer::Apt(apt) => apt,
            Installer::Brew(brew) => brew,
            Installer::Cargo(cargo) => cargo,
            Installer::Yun(yum) => yum,
            Installer::Dnf(dnf) => dnf,
            Installer::Rpm(rpm) => rpm,
            Installer::WinGet(win_get) => win_get,
        }
    }
}

impl Display for Installer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref().name())
    }
}

impl Installable for Installer {
    fn name(&self) -> &'static str {
        self.as_ref().name()
    }

    fn install(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<(), ShellError> {
        self.as_ref().install(name, args, envs, sender)
    }

    fn uninstall(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
        sender: Sender<Message>,
    ) -> CoolResult<(), ShellError> {
        self.as_ref().uninstall(name, args, envs, sender)
    }

    fn check_available(
        &self,
        name: &str,
        args: Option<&[&str]>,
        envs: Option<&[(&str, &str)]>,
    ) -> CoolResult<bool, ShellError> {
        self.as_ref().check_available(name, args, envs)
    }
}

impl Serialize for Installer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_ref().name())
    }
}

impl<'de> Deserialize<'de> for Installer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        match name.as_str() {
            name if name == Apt.name() => Ok(Installer::Apt(Apt)),
            name if name == Brew.name() => Ok(Installer::Brew(Brew)),
            name if name == Cargo.name() => Ok(Installer::Cargo(Cargo)),
            name if name == Yum.name() => Ok(Installer::Yun(Yum)),
            name if name == Dnf.name() => Ok(Installer::Dnf(Dnf)),
            name if name == Rpm.name() => Ok(Installer::Rpm(Rpm)),
            name if name == WinGet.name() => Ok(Installer::WinGet(WinGet)),
            _ => Err(serde::de::Error::custom(format!(
                "unknown installer {}",
                name
            ))),
        }
    }
}
