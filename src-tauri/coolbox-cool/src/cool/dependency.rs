use crate::CoolVersion;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct CoolDependency {
    pub name: String,
    pub version: CoolVersion,
}

impl CoolDependency {
    pub fn new(name: impl Into<String>, version: impl Into<CoolVersion>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }
}
