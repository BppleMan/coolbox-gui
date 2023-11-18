use color_eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Shell error: {shell:?} {script:?} {envs:?}\n{inner_error:?}")]
pub struct ShellError {
    pub shell: String,
    pub script: String,
    pub envs: Option<Vec<(String, String)>>,
    pub inner_error: Option<Report>,
}
