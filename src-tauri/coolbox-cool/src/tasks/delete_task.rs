use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::result::ExecutableResult;
use crate::tasks::{Executable, MessageSender};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeleteTask {
    #[serde(deserialize_with = "crate::render_str")]
    pub path: String,
}

impl DeleteTask {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl Display for DeleteTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rm -rf {}", self.path)
    }
}

impl<'a> Executable<'a> for DeleteTask {
    fn _run(&self, _send: Box<MessageSender<'a>>) -> ExecutableResult {
        fs_extra::remove_items(&[&self.path])?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use tempfile::{Builder, NamedTempFile};

    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::tasks::delete_task::DeleteTask;
    use crate::tasks::spawn_task;

    #[test]
    fn delete_file() -> CoolResult<()> {
        init_backtrace();
        let base_dir = Builder::new().prefix("cool").suffix("delete").tempdir()?;
        let path = NamedTempFile::new_in(base_dir.path())?;
        assert!(path.path().exists());
        let task = DeleteTask::new(path.path().to_string_lossy().to_string());
        spawn_task(task, |_| {})?;
        assert!(!path.path().exists());
        Ok(())
    }

    #[test]
    fn delete_dir() -> CoolResult<()> {
        init_backtrace();
        let base_dir = Builder::new().prefix("cool").suffix("delete").tempdir()?;

        let source_dir = base_dir.path().join("source");
        fs_extra::dir::create(&source_dir, true)?;
        let _child_file = File::create(source_dir.join("child_file"))?;
        let child_dir = source_dir.join("child_dir");
        fs_extra::dir::create(&child_dir, true)?;
        let _child_file1 = File::create(child_dir.join("child_file1"))?;
        let _child_file2 = File::create(child_dir.join("child_file2"))?;

        let task = DeleteTask::new(source_dir.to_string_lossy().to_string());
        spawn_task(task, |_| {})?;
        assert!(!source_dir.exists());

        Ok(())
    }
}
