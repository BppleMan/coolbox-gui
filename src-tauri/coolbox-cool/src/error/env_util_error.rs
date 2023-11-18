use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct EnvUtilError {}

impl Display for EnvUtilError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for EnvUtilError {}
