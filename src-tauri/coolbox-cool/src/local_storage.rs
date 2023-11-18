use std::fs;

use color_eyre::eyre::{eyre, Context};
use directories::{ProjectDirs, UserDirs};
use once_cell::sync::Lazy;

use crate::error::StorageError;
use crate::result::CoolResult;
use crate::unix_env_util::CoolProfile;

pub static PROJECT_DIR: Lazy<Option<ProjectDirs>> =
    Lazy::new(|| ProjectDirs::from("com", "dragit", "coolbox"));

pub static USER_DIR: Lazy<Option<UserDirs>> = Lazy::new(UserDirs::new);

pub fn project_dir() -> CoolResult<&'static ProjectDirs, StorageError> {
    PROJECT_DIR
        .as_ref()
        .ok_or(StorageError::NotFoundHomeDir(eyre!("not found")))
}

pub fn user_dir() -> CoolResult<&'static UserDirs, StorageError> {
    USER_DIR
        .as_ref()
        .ok_or(StorageError::NotFoundHomeDir(eyre!("not found")))
}

pub struct LocalStorage;

impl LocalStorage {
    #[cfg(unix)]
    pub fn cool_profile(&self) -> CoolResult<CoolProfile, StorageError> {
        let cool_profile = project_dir()?.config_dir().join("coolrc");
        if let Some(parent) = cool_profile.parent() {
            fs_extra::dir::create_all(parent, false)
                .with_context(|| format!("path: {}", parent.display()))
                .map_err(StorageError::FsExtraError)?;
        }
        if !cool_profile.exists() {
            fs::File::create(&cool_profile)
                .with_context(|| format!("path: {}", cool_profile.display()))
                .map_err(StorageError::IoError)?;
        }
        CoolProfile::new(cool_profile)
    }
}
