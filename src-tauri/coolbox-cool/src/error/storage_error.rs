use color_eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("No valid home directory path could be retrieved from the operating system.\n{0:?}")]
    NotFoundHomeDir(Report),

    #[error(transparent)]
    FsExtraError(Report),

    #[error(transparent)]
    IoError(Report),
}
