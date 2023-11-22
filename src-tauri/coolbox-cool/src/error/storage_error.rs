use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("No valid home directory path could be retrieved from the operating system.")]
    NotFoundHomeDir,

    #[error(transparent)]
    FsExtraError(#[from] fs_extra::error::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
