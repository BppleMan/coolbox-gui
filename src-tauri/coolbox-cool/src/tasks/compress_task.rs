use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use color_eyre::eyre::eyre;
use flate2::write::GzEncoder;
use serde::ser::Error;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

use coolbox_macros::State;

use crate::result::CoolResult;
use crate::tasks::{Executable, ExecutableState};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, State)]
pub struct CompressTask {
    #[serde(deserialize_with = "crate::render_str")]
    pub src: String,
    #[serde(deserialize_with = "crate::render_str")]
    pub dest: String,

    #[serde(skip)]
    state: ExecutableState,
    #[serde(skip)]
    outputs: Vec<String>,
    #[serde(skip)]
    errors: Vec<String>,
}

impl CompressTask {
    pub fn new(src: String, dest: String) -> Self {
        Self {
            src,
            dest,
            state: ExecutableState::NotStarted,
            outputs: vec![],
            errors: vec![],
        }
    }
    pub fn compress_zip(&self) -> CoolResult<()> {
        let src = PathBuf::from(&self.src);
        let parent = src
            .parent()
            .unwrap_or_else(|| panic!("{} has no parent", src.display()));
        let dest = File::create(&self.dest)?;
        let mut zip = ZipWriter::new(dest);
        for entry in WalkDir::new(&self.src) {
            let entry = entry?;
            if entry.file_type().is_dir() {
                zip.add_directory(
                    entry.path().strip_prefix(parent)?.display().to_string(),
                    FileOptions::default(),
                )?;
            } else if entry.file_type().is_file() {
                zip.start_file(
                    entry.path().strip_prefix(parent)?.display().to_string(),
                    FileOptions::default().compression_method(CompressionMethod::Stored),
                )?;
                let mut file = File::open(entry.path())?;
                let mut buf = vec![];
                file.read_to_end(&mut buf)?;
                zip.write_all(&buf)?;
            } else if entry.file_type().is_symlink() {
                zip.add_symlink(
                    entry.path().strip_prefix(parent)?.display().to_string(),
                    entry.path().read_link()?.display().to_string(),
                    FileOptions::default().compression_method(CompressionMethod::Stored),
                )?;
            }
        }
        zip.finish()?;

        Ok(())
    }

    pub fn compress_tar_gz(&self) -> CoolResult<()> {
        let src = PathBuf::from(&self.src);
        let dest = File::create(&self.dest)?;

        let gz = GzEncoder::new(dest, flate2::Compression::default());
        let mut tar = tar::Builder::new(gz);
        tar.follow_symlinks(false);
        tar.append_dir_all(src.file_name().unwrap(), &self.src)?;
        tar.finish()?;

        Ok(())
    }
}

impl Display for CompressTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.dest.ends_with(".zip") {
            write!(f, "zip -r {} {}", self.dest, self.src)
        } else if self.dest.ends_with(".tar.gz") {
            write!(f, "tar -czf {} {}", self.dest, self.src)
        } else {
            Err(fmt::Error::custom(eyre!("Not support")))
        }
    }
}

impl Executable for CompressTask {
    fn _run(&mut self) -> CoolResult<()> {
        if self.dest.ends_with(".zip") {
            self.compress_zip()
        } else if self.dest.ends_with(".tar.gz") {
            self.compress_tar_gz()
        } else {
            Err(eyre!("Not support"))
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::path::{Path, PathBuf};

    use tempfile::{Builder, TempDir};

    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::tasks::compress_task::CompressTask;
    use crate::tasks::decompress_task::DecompressTask;
    use crate::tasks::Executable;

    fn create_dir(base_dir: &TempDir) -> CoolResult<PathBuf> {
        let source_dir = base_dir.path().join("source");
        fs_extra::dir::create(&source_dir, true)?;
        let _child_file = File::create(source_dir.join("child_file"))?;
        let child_dir = source_dir.join("child_dir");
        fs_extra::dir::create(&child_dir, true)?;
        let _child_file1 = File::create(child_dir.join("child_file1"))?;
        let _child_file2 = File::create(child_dir.join("child_file2"))?;

        #[cfg(unix)]
        std::os::unix::fs::symlink(&child_dir, source_dir.join("child_symlink"))?;
        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(&child_dir, source_dir.join("child_symlink"))?;

        Ok(source_dir)
    }

    fn assert_result(dest: &Path) {
        assert!(dest.is_dir());
        assert!(dest.join("child_file").is_file());
        assert!(dest.join("child_dir").is_dir());
        assert!(dest.join("child_dir").join("child_file1").is_file());
        assert!(dest.join("child_dir").join("child_file2").is_file());
        assert!(dest.join("child_symlink").is_symlink());
    }

    #[test]
    fn zip() -> CoolResult<()> {
        init_backtrace();

        let base_dir = Builder::new()
            .prefix("cool")
            .suffix("compress_zip")
            .tempdir()?;
        let source_dir = create_dir(&base_dir)?;

        let zip_dest = base_dir.path().join("dest.zip");
        let mut compress = CompressTask::new(
            source_dir.to_string_lossy().to_string(),
            zip_dest.to_string_lossy().to_string(),
        );
        compress.execute()?;
        assert!(zip_dest.exists());

        let dest = base_dir.path().join("dest");
        let mut decompress = DecompressTask::new(
            zip_dest.to_string_lossy().to_string(),
            dest.to_string_lossy().to_string(),
        );
        decompress.execute()?;
        assert_result(&dest);

        Ok(())
    }

    #[test]
    fn tar_gz() -> CoolResult<()> {
        init_backtrace();

        let base_dir = Builder::new()
            .prefix("cool")
            .suffix("compress_tgz")
            .tempdir()?;
        let source_dir = create_dir(&base_dir)?;

        let tgz_dest = base_dir.path().join("dest.tar.gz");
        let mut compress = CompressTask::new(
            source_dir.to_string_lossy().to_string(),
            tgz_dest.to_string_lossy().to_string(),
        );
        compress.execute()?;
        assert!(tgz_dest.exists());

        let dest = base_dir.path().join("dest");
        let mut decompress = DecompressTask::new(
            tgz_dest.to_string_lossy().to_string(),
            dest.to_string_lossy().to_string(),
        );
        decompress.execute()?;
        assert_result(&dest);

        Ok(())
    }
}
