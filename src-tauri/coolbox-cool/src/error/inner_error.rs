use thiserror::Error;

#[derive(Debug, Error)]
pub enum InnerError {
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),

    #[error(transparent)]
    WalkDirError(#[from] walkdir::Error),

    #[error(transparent)]
    ConvertError(#[from] std::convert::Infallible),

    #[error(transparent)]
    FsExtraError(#[from] fs_extra::error::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}
