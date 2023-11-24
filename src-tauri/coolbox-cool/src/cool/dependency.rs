use crate::cool::CoolVersion;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct CoolDependency {
    pub name: String,
    pub version: CoolVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CoolDependencies(HashSet<CoolDependency>);

impl Hash for CoolDependencies {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.iter().for_each(|d| d.hash(state));
    }
}
