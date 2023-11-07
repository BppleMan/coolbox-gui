use thiserror::Error;

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
