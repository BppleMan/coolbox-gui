use crate::cool::CoolDependencies;
use crate::installer::Installer;
use crate::tasks::Tasks;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum CoolFormula {
    PackageManager {
        installer: Installer,
        name: String,
        post_install_tasks: Tasks,
        post_uninstall_tasks: Tasks,
    },
    Manual {
        #[serde(default, flatten)]
        dependencies: CoolDependencies,
        install_tasks: Tasks,
        uninstall_tasks: Tasks,
        check_tasks: Tasks,
    },
}
