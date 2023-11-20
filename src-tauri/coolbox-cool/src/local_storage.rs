use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use crate::env_manager::ShellProfile;
use color_eyre::eyre::{eyre, Context};
use directories::{ProjectDirs, UserDirs};
use once_cell::sync::Lazy;

use crate::error::StorageError;
use crate::result::CoolResult;

pub static PROJECT_DIRS: Lazy<ProjectDirs> =
    Lazy::new(|| match ProjectDirs::from("com", "dragit", "coolbox") {
        Some(project_dirs) => project_dirs,
        None => panic!("{}", StorageError::NotFoundHomeDir),
    });

pub static USER_DIRS: Lazy<UserDirs> = Lazy::new(|| match UserDirs::new() {
    Some(user_dirs) => user_dirs,
    None => panic!("{}", StorageError::NotFoundHomeDir),
});

pub struct LocalStorage;

impl LocalStorage {
    #[cfg(unix)]
    pub fn cool_profile(&self) -> ShellProfile {
        let cool_profile = PROJECT_DIRS.config_dir().join("cool_profile.sh");
        #[cfg(test)]
        let cool_profile = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("target")
            .join("tmp")
            .join("cool_profile_test.sh");
        if let Some(parent) = cool_profile.parent() {
            fs_extra::dir::create_all(parent, false)
                .unwrap_or_else(|_| panic!("Failed to create parent dir: {}", parent.display()));
        }
        if !cool_profile.exists() {
            fs::File::create(&cool_profile).expect("Failed to create cool_profile.sh");
        }
        ShellProfile::new(cool_profile)
    }
}
