use crate::result::ExecutableResult;
use crate::shell::{Cmd, ShellExecutor};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct EnvTask {
    pub key: String,
    pub value: String,
}

impl EnvTask {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }

    // #[cfg(target_os = "windows")]
    // pub fn set(&self) -> ExecutableResult {
    //     let origin =
    //     Cmd.run(format!("setx {}"))
    // }
}
