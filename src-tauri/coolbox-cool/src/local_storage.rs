#[cfg(unix)]
use crate::env_manager::ShellProfile;
use directories::{ProjectDirs, UserDirs};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::error::StorageError;

pub static PROJECT_DIRS: Lazy<ProjectDirs> =
    Lazy::new(|| match ProjectDirs::from("com", "dragit", "coolbox") {
        Some(project_dirs) => project_dirs,
        None => panic!("{}", StorageError::NotFoundHomeDir),
    });

pub static USER_DIRS: Lazy<UserDirs> = Lazy::new(|| match UserDirs::new() {
    Some(user_dirs) => user_dirs,
    None => panic!("{}", StorageError::NotFoundHomeDir),
});

pub static LOCAL_STORAGE: Lazy<LocalStorage> = Lazy::new(LocalStorage::default);

#[cfg(unix)]
const COOL_PROFILE: &str = "cool_profile.sh";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalStorage {
    pub config_prefix: PathBuf,
    pub data_prefix: PathBuf,
}

impl LocalStorage {
    #[inline]
    fn generate_dir(&self, dir: PathBuf) -> PathBuf {
        fs_extra::dir::create_all(&dir, false)
            .unwrap_or_else(|_| panic!("Failed to create dir: {}", dir.display()));
        dir
    }

    #[inline]
    fn generate_file(&self, file: PathBuf) -> PathBuf {
        if let Some(parent) = file.parent() {
            fs_extra::dir::create_all(parent, false)
                .unwrap_or_else(|_| panic!("Failed to create parent dir: {}", parent.display()));
        }
        if !file.exists() {
            std::fs::File::create(&file)
                .unwrap_or_else(|_| panic!("Failed to create file: {}", file.display()));
        }
        file
    }

    pub fn cools(&self) -> PathBuf {
        self.generate_dir(self.config_prefix.join("cools"))
    }

    pub fn cools_macos(&self) -> PathBuf {
        self.generate_dir(self.cools().join("macos"))
    }

    pub fn cools_linux(&self) -> PathBuf {
        self.generate_dir(self.cools().join("linux"))
    }

    pub fn cools_windows(&self) -> PathBuf {
        self.generate_dir(self.cools().join("windows"))
    }

    pub fn cools_unix(&self) -> PathBuf {
        self.generate_dir(self.cools().join("unix"))
    }

    pub fn cools_universal(&self) -> PathBuf {
        self.generate_dir(self.cools().join("universal"))
    }
}

#[cfg(unix)]
impl LocalStorage {
    pub fn cool_profile(&self) -> ShellProfile {
        let cool_profile = self.generate_file(self.config_prefix.join(COOL_PROFILE));
        ShellProfile::new(cool_profile)
    }
}

impl Default for LocalStorage {
    fn default() -> Self {
        Self {
            config_prefix: PROJECT_DIRS.config_dir().to_path_buf(),
            data_prefix: PROJECT_DIRS.data_dir().to_path_buf(),
        }
    }
}
