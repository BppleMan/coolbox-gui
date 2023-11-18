use color_eyre::Report;
use thiserror::Error;

use crate::error::InnerError;

#[derive(Debug, Error)]
pub enum ExecutableError {
    #[error(transparent)]
    InnerError(#[from] InnerError),

    #[error(transparent)]
    NotAvailable(Report),

    #[error(transparent)]
    CommandNotFound(Report),

    #[error(transparent)]
    CreatePathError(Report),

    #[error(transparent)]
    FileNotExists(Report),

    #[error(transparent)]
    TargetIsFile(Report),

    #[error(transparent)]
    PathNoParent(Report),

    #[error(transparent)]
    UnsupportedCompressType(Report),
}
