#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ExecutableState {
    #[default]
    NotStarted,
    Running,
    Finished,
    Error,
}

pub enum ExecutableMessageType {
    Info,
    Warn,
    Error,
}

pub struct ExecutableMessage {
    pub message_type: ExecutableMessageType,
    pub message: String,
}

pub struct ExecutableReceiver {
    pub state: crossbeam::channel::Receiver<ExecutableState>,
    pub message: crossbeam::channel::Receiver<ExecutableMessage>,
}

pub struct ExecutableSender {
    pub state: crossbeam::channel::Sender<ExecutableState>,
    pub message: crossbeam::channel::Sender<ExecutableMessage>,
}

pub trait IntoMessage {
    fn into_info(self) -> ExecutableMessage;
    fn into_warn(self) -> ExecutableMessage;
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

impl<T> IntoMessage for T
where
    T: Into<String>,
{
    fn into_info(self) -> ExecutableMessage {
        ExecutableMessage::info(self)
    }

    fn into_warn(self) -> ExecutableMessage {
        ExecutableMessage::warn(self)
    }

    fn into_error(self) -> ExecutableMessage {
        ExecutableMessage::error(self)
    }
}
