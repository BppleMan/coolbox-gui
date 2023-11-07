use crate::state::State;
use cool::Cool;
use serde::{Deserialize, Serialize};

use crate::task_data::TaskData;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct CoolData {
    pub name: String,
    pub description: String,
    pub state: State,
    pub dependencies: Vec<String>,
    pub install_tasks: Vec<TaskData>,
    pub uninstall_tasks: Vec<TaskData>,
    pub check_tasks: Vec<TaskData>,
}

impl From<&Cool> for CoolData {
    fn from(value: &Cool) -> Self {
        Self {
            name: value.name.clone(),
            description: value.description.clone(),
            dependencies: value.dependencies.clone(),
            install_tasks: value
                .install_tasks
                .0
                .iter()
                .map(|t| t.into())
                .collect::<Vec<_>>(),
            uninstall_tasks: value
                .uninstall_tasks
                .0
                .iter()
                .map(|t| t.into())
                .collect::<Vec<_>>(),
            check_tasks: value
                .check_tasks
                .0
                .iter()
                .map(|t| t.into())
                .collect::<Vec<_>>(),
        }
    }
}
