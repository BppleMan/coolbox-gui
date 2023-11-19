use crate::error::{ShellError, StorageError};
use std::error::Error;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
    #[error(transparent)]
    ShellError(#[from] ShellError),

    #[error(transparent)]
    StorageError(#[from] StorageError),

    #[error("Invalid env var: {0}")]
    InvalidEnvVar(String),
}
