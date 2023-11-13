use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use fs_extra::dir::{CopyOptions, TransitProcessResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{DirTransitProcessInfo, FileTransitProcessInfo};
use crate::result::ExecutableResult;
use crate::tasks::{Executable, MessageSender};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct MoveTask {
    #[serde(deserialize_with = "crate::template_string")]
    pub src: String,
    #[serde(deserialize_with = "crate::template_string")]
    pub dest: String,
}

impl MoveTask {
    pub fn new(src: String, dest: String) -> Self {
        Self { src, dest }
    }
}

impl Display for MoveTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if cfg!(target_os = "unix") {
            write!(f, "mv -f {} {}", self.src, self.dest)
        } else if cfg!(target_os = "windows") {
            write!(f, "move /Y {} {}", self.src, self.dest)
        } else {
            write!(f, "mv -f {} {}", self.src, self.dest)
        }
    }
}

impl<'a> Executable<'a> for MoveTask {
    fn execute(&self, mut send: Box<MessageSender<'a>>) -> ExecutableResult {
        let src = PathBuf::from_str(&self.src)?;
        let dest = PathBuf::from_str(&self.dest)?;
        if src.is_dir() {
            fs_extra::dir::move_dir_with_progress(
                &self.src,
                &self.dest,
                &CopyOptions::new().skip_exist(true).copy_inside(true),
                |transit| {
                    send(transit.as_info());
                    TransitProcessResult::OverwriteAll
                },
            )?;
        } else {
            let options = fs_extra::file::CopyOptions::new().skip_exist(true);
            if dest.is_dir() {
                let file_name = src.file_name().unwrap();
                fs_extra::file::move_file_with_progress(
                    &self.src,
                    dest.join(file_name),
                    &options,
                    |transit| {
                        send(transit.as_info(dest.to_string_lossy()));
                    },
                )?;
            } else {
                fs_extra::file::move_file_with_progress(
                    &self.src,
                    &self.dest,
                    &options,
                    |transit| {
                        send(transit.as_info(dest.to_string_lossy()));
                    },
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use tempfile::{Builder, NamedTempFile};

    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::tasks::spawn_task;

    #[test]
    fn move_file() -> CoolResult<()> {
        init_backtrace();
        let base_dir = Builder::new().prefix("cool").suffix("move").tempdir()?;
        let source_file = NamedTempFile::with_prefix_in("source", base_dir.path())?;

        let dest_path = base_dir.path().join("dest");
        let task = super::MoveTask::new(
            source_file.path().to_string_lossy().to_string(),
            dest_path.as_path().to_string_lossy().to_string(),
        );
        spawn_task(task, |_| {})?;
        assert!(!source_file.path().exists());
        assert!(dest_path.exists());
        Ok(())
    }

    #[test]
    fn move_dir() -> CoolResult<()> {
        init_backtrace();
        let base_dir = Builder::new().prefix("cool").suffix("move").tempdir()?;

        let source_dir = base_dir.path().join("source");
        fs_extra::dir::create(&source_dir, true)?;
        let _child_file = File::create(source_dir.join("child_file"))?;
        let child_dir = source_dir.join("child_dir");
        fs_extra::dir::create(&child_dir, true)?;
        let _child_file1 = File::create(child_dir.join("child_file1"))?;
        let _child_file2 = File::create(child_dir.join("child_file2"))?;

        let dest_path = base_dir.path().join("dest");
        let task = super::MoveTask::new(
            source_dir.to_string_lossy().to_string(),
            dest_path.as_path().to_string_lossy().to_string(),
        );
        spawn_task(task, |_| {})?;
        assert!(!source_dir.exists());
        assert!(dest_path.exists());
        assert!(dest_path.join("child_file").exists());
        assert!(dest_path.join("child_dir").exists());
        assert!(dest_path.join("child_dir").join("child_file1").exists());
        assert!(dest_path.join("child_dir").join("child_file2").exists());
        Ok(())
    }
}
