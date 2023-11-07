use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use fs_extra::dir::TransitProcessResult;
use serde::{Deserialize, Serialize};

use crate::result::ExecutableResult;
use crate::tasks::{Executable, ExecutableSender};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CopyTask {
    #[serde(deserialize_with = "crate::render_str")]
    pub src: String,
    #[serde(deserialize_with = "crate::render_str")]
    pub dest: String,
}

impl CopyTask {
    pub fn new(src: String, dest: String) -> Self {
        Self { src, dest }
    }
}

impl Display for CopyTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if cfg!(target_os = "unix") {
            write!(f, "cp -rf {} {}", self.src, self.dest)
        } else if cfg!(target_os = "windows") {
            write!(f, "xcopy /E /I /Y {} {}", self.src, self.dest)
        } else {
            write!(f, "cp -rf {} {}", self.src, self.dest)
        }
    }
}

impl Executable for CopyTask {
    fn _run(&mut self, sender: &ExecutableSender) -> ExecutableResult {
        let src = PathBuf::from_str(&self.src)?;
        let dest = PathBuf::from_str(&self.dest)?;
        if src.is_dir() {
            let options = fs_extra::dir::CopyOptions::new()
                .skip_exist(true)
                .copy_inside(true);
            fs_extra::dir::copy_with_progress(&self.src, &self.dest, &options, |transit| {
                sender
                    .outputs
                    .send(format!(
                        "{}({}/{}) total:{}/{}",
                        transit.file_name,
                        transit.copied_bytes,
                        transit.total_bytes,
                        transit.copied_bytes,
                        transit.total_bytes,
                    ))
                    .unwrap();
                TransitProcessResult::OverwriteAll
            })?;
        } else {
            let options = fs_extra::file::CopyOptions::new().skip_exist(true);
            if dest.is_dir() {
                let file_name = src.file_name().unwrap();
                fs_extra::file::copy_with_progress(
                    &self.src,
                    dest.join(file_name),
                    &options,
                    |transit| {
                        sender
                            .outputs
                            .send(format!(
                                "{}({}/{})",
                                file_name.to_string_lossy(),
                                transit.copied_bytes,
                                transit.total_bytes,
                            ))
                            .unwrap();
                    },
                )?;
            } else {
                fs_extra::file::copy_with_progress(&self.src, &self.dest, &options, |transit| {
                    sender
                        .outputs
                        .send(format!(
                            "{}({}/{})",
                            self.src, transit.copied_bytes, transit.total_bytes,
                        ))
                        .unwrap();
                })?;
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
    use crate::tasks::Executable;

    #[test]
    fn copy_file() -> CoolResult<()> {
        init_backtrace();

        let base_dir = Builder::new().prefix("cool").suffix("copy").tempdir()?;
        let source_file = NamedTempFile::with_prefix_in("source", base_dir.path())?;

        let dest_path = base_dir.path().join("dest");
        super::CopyTask::new(
            source_file.path().to_string_lossy().to_string(),
            dest_path.as_path().to_string_lossy().to_string(),
        )
            .execute()?;
        assert!(dest_path.exists());

        let dest_dir = Builder::new().prefix("dest").tempdir_in(base_dir.path())?;
        let dest_path = dest_dir.path();
        super::CopyTask::new(
            source_file.path().to_string_lossy().to_string(),
            dest_path.to_string_lossy().to_string(),
        )
            .execute()?;
        assert!(dest_path.exists());

        let dest_path = dest_dir.path().join("dest");
        super::CopyTask::new(
            source_file.path().to_string_lossy().to_string(),
            dest_path.as_path().to_string_lossy().to_string(),
        )
            .execute()?;
        assert!(dest_path.exists());

        Ok(())
    }

    #[test]
    fn copy_dir() -> CoolResult<()> {
        init_backtrace();

        let base_dir = Builder::new().prefix("cool").suffix("copy").tempdir()?;

        let source_dir = base_dir.path().join("source");
        fs_extra::dir::create(&source_dir, true)?;
        let _child_file = File::create(source_dir.join("child_file"))?;
        let child_dir = source_dir.join("child_dir");
        fs_extra::dir::create(&child_dir, true)?;
        let _child_file1 = File::create(child_dir.join("child_file1"))?;
        let _child_file2 = File::create(child_dir.join("child_file2"))?;

        let dest_dir = base_dir.path().join("dest");
        // fs_extra::dir::create(&dest_dir, true)?;
        super::CopyTask::new(
            source_dir.to_string_lossy().to_string(),
            dest_dir.to_string_lossy().to_string(),
        )
            .execute()?;

        assert!(dest_dir.exists());
        assert!(dest_dir.join("child_file").exists());
        assert!(dest_dir.join("child_dir").exists());
        assert!(dest_dir.join("child_dir").join("child_file1").exists());
        assert!(dest_dir.join("child_dir").join("child_file2").exists());

        Ok(())
    }
}
