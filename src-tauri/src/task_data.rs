use cool::tasks::Task;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TaskData {
    pub name: String,
    pub description: String,
}

impl From<&Task> for TaskData {
    fn from(value: &Task) -> Self {
        Self {
            name: value.name().to_string(),
            description: format!("{}", value),
        }
    }
}
