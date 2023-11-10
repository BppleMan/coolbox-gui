use std::fmt::{Display, Formatter};

use crate::tasks::Task;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum MessageType {
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_type: MessageType,
    pub message: String,
}

pub type MessageSender<'a> = dyn FnMut(Message) + 'a;

pub type TasksMessageSender<'a> = dyn FnMut(usize, &'a Task, Message) + 'a;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaskEvent {
    pub cool_name: String,
    pub task_name: String,
    pub task_index: usize,
    pub message: Message,
}

pub enum CoolEvent {}

pub trait IntoInfo {
    fn into_info(self) -> Message;
}

pub trait IntoWarn {
    fn into_warn(self) -> Message;
}

pub trait IntoError {
    fn into_error(self) -> Message;
}

impl Message {
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message_type: MessageType::Info,
            message: message.into(),
        }
    }

    pub fn warn(message: impl Into<String>) -> Self {
        Self {
            message_type: MessageType::Warn,
            message: message.into(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message_type: MessageType::Error,
            message: message.into(),
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.message_type {
            MessageType::Info => write!(f, "[INFO] {}", self.message),
            MessageType::Warn => write!(f, "[WARN] {}", self.message),
            MessageType::Error => write!(f, "[ERROR] {}", self.message),
        }
    }
}

impl<T: Into<String>> IntoInfo for T {
    fn into_info(self) -> Message {
        Message::info(self)
    }
}

impl<T: Into<String>> IntoWarn for T {
    fn into_warn(self) -> Message {
        Message::warn(self)
    }
}

impl<T: Into<String>> IntoError for T {
    fn into_error(self) -> Message {
        Message::error(self)
    }
}

pub trait TransitProcessInfo {
    fn as_info(&self) -> Message;
}

impl TransitProcessInfo for fs_extra::TransitProcess {
    fn as_info(&self) -> Message {
        Message::info(format!(
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
    fn as_info(&self, file_name: impl AsRef<str>) -> Message;
}

impl FileTransitProcessInfo for fs_extra::file::TransitProcess {
    fn as_info(&self, file_name: impl AsRef<str>) -> Message {
        Message::info(format!(
            "{}({}/{})",
            file_name.as_ref(),
            self.copied_bytes,
            self.total_bytes,
        ))
    }
}

pub trait DirTransitProcessInfo {
    fn as_info(&self) -> Message;
}

impl DirTransitProcessInfo for fs_extra::dir::TransitProcess {
    fn as_info(&self) -> Message {
        Message::info(format!(
            "{}({}/{}) total:{}/{}",
            self.file_name,
            self.file_bytes_copied,
            self.file_total_bytes,
            self.copied_bytes,
            self.total_bytes,
        ))
    }
}
