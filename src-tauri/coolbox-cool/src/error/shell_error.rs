use crate::shell::Shell;
use color_eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ShellError {
    pub shell: Shell,
    pub script: String,
    pub envs: Vec<(String, String)>,
    #[source]
    pub inner_error: Option<Report>,
}
