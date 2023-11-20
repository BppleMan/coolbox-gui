use crate::error::{ShellError, StorageError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
    #[error(transparent)]
    ShellError(#[from] ShellError),

    #[error(transparent)]
    StorageError(#[from] StorageError),

    #[error("Invalid env var: {0}")]
    InvalidEnvVar(String),

    #[error("Empty env variable key")]
    EmptyKey,

    #[error("Empty env variable value")]
    EmptyValue,

    #[error("Empty path value")]
    EmptyPathValue,

    #[error("Empty source value")]
    EmptySourceValue,
}
