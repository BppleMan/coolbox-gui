use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum ExecutableMessageType {
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExecutableMessage {
    pub message_type: ExecutableMessageType,
    pub message: String,
}

pub type ExecutableReceiver = crossbeam::channel::Receiver<ExecutableMessage>;

pub type ExecutableSender = crossbeam::channel::Sender<ExecutableMessage>;

pub fn executable_channel() -> (ExecutableSender, ExecutableReceiver) {
    crossbeam::channel::unbounded()
}

pub trait IntoInfo {
    fn into_info(self) -> ExecutableMessage;
}

pub trait IntoWarn {
    fn into_warn(self) -> ExecutableMessage;
}

pub trait IntoError {
    fn into_error(self) -> ExecutableMessage;
}

impl ExecutableMessage {
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message_type: ExecutableMessageType::Info,
            message: message.into(),
        }
    }

    pub fn warn(message: impl Into<String>) -> Self {
        Self {
            message_type: ExecutableMessageType::Warn,
            message: message.into(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message_type: ExecutableMessageType::Error,
            message: message.into(),
        }
    }
}

impl Display for ExecutableMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.message_type {
            ExecutableMessageType::Info => write!(f, "[INFO] {}", self.message),
            ExecutableMessageType::Warn => write!(f, "[WARN] {}", self.message),
            ExecutableMessageType::Error => write!(f, "[ERROR] {}", self.message),
        }
    }
}

impl<T: Into<String>> IntoInfo for T {
    fn into_info(self) -> ExecutableMessage {
        ExecutableMessage::info(self)
    }
}

impl<T: Into<String>> IntoWarn for T {
    fn into_warn(self) -> ExecutableMessage {
        ExecutableMessage::warn(self)
    }
}

impl<T: Into<String>> IntoError for T {
    fn into_error(self) -> ExecutableMessage {
        ExecutableMessage::error(self)
    }
}

pub trait TransitProcessInfo {
    fn as_info(&self) -> ExecutableMessage;
}

impl TransitProcessInfo for fs_extra::TransitProcess {
    fn as_info(&self) -> ExecutableMessage {
        ExecutableMessage::info(format!(
            "{} {}({}/{}) total:{}/{}",
            self.dir_name,
            self.file_name,
            self.file_bytes_copied,
            self.file_total_bytes,
            self.copied_bytes,
            self.total_bytes,
        ))
    }
}

pub trait FileTransitProcessInfo {
    fn as_info(&self, file_name: impl AsRef<str>) -> ExecutableMessage;
}

impl FileTransitProcessInfo for fs_extra::file::TransitProcess {
    fn as_info(&self, file_name: impl AsRef<str>) -> ExecutableMessage {
        ExecutableMessage::info(format!(
            "{}({}/{})",
            file_name.as_ref(),
            self.copied_bytes,
            self.total_bytes,
        ))
    }
}

pub trait DirTransitProcessInfo {
    fn as_info(&self) -> ExecutableMessage;
}

impl DirTransitProcessInfo for fs_extra::dir::TransitProcess {
    fn as_info(&self) -> ExecutableMessage {
        ExecutableMessage::info(format!(
            "{}({}/{}) total:{}/{}",
            self.file_name,
            self.file_bytes_copied,
            self.file_total_bytes,
            self.copied_bytes,
            self.total_bytes,
        ))
    }
}
