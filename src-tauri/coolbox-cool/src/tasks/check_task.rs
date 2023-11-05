use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use crate::installer::{Installable, Installer};
use coolbox_macros::State;

use crate::result::CoolResult;
use crate::tasks::{Executable, ExecutableState};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, State)]
pub struct CheckTask {
    pub name: String,
    pub installer: Installer,

    #[serde(skip)]
    state: ExecutableState,
    #[serde(skip)]
    outputs: Vec<String>,
    #[serde(skip)]
    errors: Vec<String>,
}

impl CheckTask {
    pub fn new(name: String, installer: Installer) -> Self {
        Self {
            name,
            installer,
            state: ExecutableState::NotStarted,
            outputs: vec![],
            errors: vec![],
        }
    }
}

impl Display for CheckTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "check {}", self.name)
    }
}

impl Executable for CheckTask {
    fn _run(&mut self) -> CoolResult<()> {
        self.installer
            .check_available(&self.name, None)
            .and_then(|result| {
                if result {
                    self.outputs.push(format!("{} is available", &self.name));
                    Ok(())
                } else {
                    let msg = format!("{} is not available", &self.name);
                    self.errors.push(msg.clone());
                    Err(eyre!(msg))
                }
            })
    }
}
