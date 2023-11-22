use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CoolState {
    Ready,
    Installing,
    Uninstalling,
    Installed,
}
