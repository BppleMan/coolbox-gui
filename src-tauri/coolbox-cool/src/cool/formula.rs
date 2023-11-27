use crate::installer::Installer;
use crate::tasks::Tasks;
use crate::CoolDependency;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum CoolFormula {
    PackageManager(PackageManager),
    Manual(Manual),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct PackageManager {
    pub installer: Installer,
    pub name: String,
    pub post_install_tasks: Tasks,
    pub post_uninstall_tasks: Tasks,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Manual {
    #[serde(default)]
    pub dependencies: HashSet<CoolDependency>,
    pub install_tasks: Tasks,
    pub uninstall_tasks: Tasks,
    pub check_tasks: Tasks,
}

impl CoolFormula {
    pub fn package_manager(
        installer: Installer,
        name: impl Into<String>,
        post_install_tasks: Tasks,
        post_uninstall_tasks: Tasks,
    ) -> Self {
        let name = name.into();
        Self::PackageManager(PackageManager {
            installer,
            name,
            post_install_tasks,
            post_uninstall_tasks,
        })
    }

    pub fn manual(
        dependencies: HashSet<CoolDependency>,
        install_tasks: Tasks,
        uninstall_tasks: Tasks,
        check_tasks: Tasks,
    ) -> Self {
        Self::Manual(Manual {
            dependencies,
            install_tasks,
            uninstall_tasks,
            check_tasks,
        })
    }
}

impl Hash for Manual {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dependencies.iter().for_each(|dep| dep.hash(state));
        self.install_tasks.hash(state);
        self.uninstall_tasks.hash(state);
        self.check_tasks.hash(state);
    }
}
