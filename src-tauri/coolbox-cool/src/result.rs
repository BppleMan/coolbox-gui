use color_eyre::Report;

use crate::error::ExecutableError;

pub type CoolResult<T, E = Report> = color_eyre::Result<T, E>;

pub type ExecutableResult = CoolResult<(), ExecutableError>;
