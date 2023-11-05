use serde::{Deserialize, Serialize};

use cool::tasks::Task;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CoolData {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub install_tasks: Vec<Task>,
    pub uninstall_tasks: Vec<Task>,
    pub check_tasks: Vec<Task>,
}
