use std::convert::Infallible;
use std::io;
use std::path::StripPrefixError;

use color_eyre::Report;
use git2::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutableError {
    #[error(transparent)]
    ZipError(Report),

    #[error(transparent)]
    IOError(Report),

    #[error(transparent)]
    StripPrefixError(Report),

    #[error(transparent)]
    WalkDirError(Report),

    #[error(transparent)]
    PathBufError(Report),

    #[error(transparent)]
    FsExtraError(Report),

    #[error(transparent)]
    NotAvailable(Report),

    #[error(transparent)]
    ShellError(Report),

    #[error(transparent)]
    UnsupportedCompressType(Report),

    #[error(transparent)]
    PathNoParent(Report),

    #[error("Target is file: {0:?}")]
    TargetIsFile(Report),

    #[error(transparent)]
    ReqwestError(Report),

    #[error("File not exists: {0}")]
    FileNotExists(Report),

    #[error(transparent)]
    GitError(Report),

    #[error(transparent)]
    CommandNotFound(Report),

    #[error(transparent)]
    UnknownError(Report),
}

impl From<git2::Error> for ExecutableError {
    fn from(value: Error) -> Self {
        Self::GitError(value.into())
    }
}

impl From<reqwest::Error> for ExecutableError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value.into())
    }
}

impl From<fs_extra::error::Error> for ExecutableError {
    fn from(value: fs_extra::error::Error) -> Self {
        Self::FsExtraError(value.into())
    }
}

impl From<Infallible> for ExecutableError {
    fn from(value: Infallible) -> Self {
        Self::PathBufError(value.into())
    }
}

impl From<walkdir::Error> for ExecutableError {
    fn from(value: walkdir::Error) -> Self {
        Self::WalkDirError(value.into())
    }
}

impl From<StripPrefixError> for ExecutableError {
    fn from(value: StripPrefixError) -> Self {
        Self::StripPrefixError(value.into())
    }
}

impl From<io::Error> for ExecutableError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value.into())
    }
}

impl From<zip::result::ZipError> for ExecutableError {
    fn from(value: zip::result::ZipError) -> Self {
        Self::ZipError(value.into())
    }
}

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("{0} is already installing")]
    AlreadyInstalling(String),

    #[error("{0} is already uninstalling")]
    AlreadyUninstalling(String),
}

#[derive(Debug, Error)]
pub enum TransformError {
    #[error("Not found cool: {0}")]
    NotFoundCool(String),
}
