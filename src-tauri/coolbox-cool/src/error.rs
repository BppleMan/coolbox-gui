mod env_error;
mod inner_error;
mod shell_error;
mod storage_error;
mod task_error;

pub use env_error::*;
pub use inner_error::*;
pub use shell_error::*;
pub use storage_error::*;
pub use task_error::*;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum CoolError {
    #[error(
        "Cool [{cool_name}] error: execute task [{task_name}] index [{task_index}] error: {error}"
    )]
    ExecuteError {
        cool_name: String,
        task_name: String,
        task_index: usize,
        error: String,
    },

    #[error("Unsupported cool file: {file}")]
    UnsupportedCoolFile { file: String },

    #[error("Not found cool: {cool_name}")]
    NotFoundCool { cool_name: String },

    #[error("Cannot parse file: [{file}] as validate struct")]
    ParseError { file: String },

    #[error("Cool [{cool_name}] error: {error}")]
    UnknownError { cool_name: String, error: String },
}

impl CoolError {
    pub fn from(cool_name: String, task_name: String, task_index: usize, error: TaskError) -> Self {
        Self::ExecuteError {
            cool_name,
            task_name,
            task_index,
            error: error.to_string(),
        }
    }
}
