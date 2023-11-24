use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use color_eyre::eyre::eyre;
use flate2::write::GzEncoder;
use schemars::JsonSchema;
use serde::ser::Error;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::cool::IntoInfo;
use crate::error::{CompressTaskError, InnerError, TaskError};
use crate::result::CoolResult;
use crate::tasks::{Executable, MessageSender};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct CompressTask {
    #[serde(deserialize_with = "crate::cool::template_string")]
    pub src: String,
    #[serde(deserialize_with = "crate::cool::template_string")]
    pub dest: String,
}

impl CompressTask {
    pub fn new(src: String, dest: String) -> Self {
        Self { src, dest }
    }

    pub fn compress_zip(&self, mut send: Box<MessageSender>) -> CoolResult<(), TaskError> {
        let src = PathBuf::from(&self.src);
        let parent = src.parent().ok_or_else(|| TaskError::CompressTaskError {
            task: self.clone(),
            source: CompressTaskError::SourceNoParent(self.src.clone()),
        })?;
        let dest = File::create(&self.dest).map_err(|e| self.map_inner_error(e))?;
        let mut zip = ZipWriter::new(dest);
        for entry in WalkDir::new(&self.src) {
            let entry = entry.map_err(|e| self.map_inner_error(e))?;
            if entry.file_type().is_dir() {
                zip.add_directory(
                    entry
                        .path()
                        .strip_prefix(parent)
                        .map_err(|e| self.map_inner_error(e))?
                        .display()
                        .to_string(),
                    FileOptions::default(),
                )
                .map_err(|e| self.map_inner_error(e))?;
                send(format!("Compress directory {}", entry.path().display()).into_info());
            } else if entry.file_type().is_file() {
                zip.start_file(
                    entry
                        .path()
                        .strip_prefix(parent)
                        .map_err(|e| self.map_inner_error(e))?
                        .display()
                        .to_string(),
                    FileOptions::default().compression_method(CompressionMethod::Stored),
                )
                .map_err(|e| self.map_inner_error(e))?;
                let mut file = File::open(entry.path()).map_err(|e| self.map_inner_error(e))?;
                let mut buf = vec![];
                file.read_to_end(&mut buf)
                    .map_err(|e| self.map_inner_error(e))?;
                zip.write_all(&buf).map_err(|e| self.map_inner_error(e))?;
                send(format!("Compress file {}", entry.path().display()).into_info());
            } else if entry.file_type().is_symlink() {
                zip.add_symlink(
                    entry
                        .path()
                        .strip_prefix(parent)
                        .map_err(|e| self.map_inner_error(e))?
                        .display()
                        .to_string(),
                    entry
                        .path()
                        .read_link()
                        .map_err(|e| self.map_inner_error(e))?
                        .display()
                        .to_string(),
                    FileOptions::default().compression_method(CompressionMethod::Stored),
                )
                .map_err(|e| self.map_inner_error(e))?;
                send(format!("Compress symlink {}", entry.path().display()).into_info());
            }
        }
        zip.finish().map_err(|e| self.map_inner_error(e))?;

        Ok(())
    }

    pub fn compress_tar_gz(&self, mut send: Box<MessageSender>) -> CoolResult<(), TaskError> {
        let src = PathBuf::from(&self.src);
        let dest = File::create(&self.dest).map_err(|e| self.map_inner_error(e))?;

        let gz = GzEncoder::new(dest, flate2::Compression::default());
        let mut tar = tar::Builder::new(gz);
        tar.follow_symlinks(false);
        tar.append_dir_all(src.file_name().unwrap(), &self.src)
            .map_err(|e| self.map_inner_error(e))?;
        tar.finish().map_err(|e| self.map_inner_error(e))?;
        send(format!("Compress directory {}", &self.src).into_info());

        Ok(())
    }

    fn map_inner_error(&self, error: impl Into<InnerError>) -> TaskError {
        TaskError::CompressTaskError {
            task: self.clone(),
            source: CompressTaskError::InnerError(error.into()),
        }
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

impl<'a> Executable<'a> for CompressTask {
    fn execute(&self, send: Box<MessageSender<'a>>) -> CoolResult<(), TaskError> {
        if self.dest.ends_with(".zip") {
            self.compress_zip(send)
        } else if self.dest.ends_with(".tar.gz") {
            self.compress_tar_gz(send)
        } else {
            Err(TaskError::CompressTaskError {
                task: self.clone(),
                source: CompressTaskError::UnsupportedCompressType(self.dest.clone()),
            })
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
    use crate::tasks::spawn_task;

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
        let compress = CompressTask::new(
            source_dir.to_string_lossy().to_string(),
            zip_dest.to_string_lossy().to_string(),
        );
        spawn_task(compress, |msg| println!("{}", msg))?;
        assert!(zip_dest.exists());

        let dest = base_dir.path().join("dest");
        let decompress = DecompressTask::new(
            zip_dest.to_string_lossy().to_string(),
            dest.to_string_lossy().to_string(),
        );
        spawn_task(decompress, |msg| println!("{}", msg))?;
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
        let compress = CompressTask::new(
            source_dir.to_string_lossy().to_string(),
            tgz_dest.to_string_lossy().to_string(),
        );
        spawn_task(compress, |_| {})?;
        assert!(tgz_dest.exists());

        let dest = base_dir.path().join("dest");
        let decompress = DecompressTask::new(
            tgz_dest.to_string_lossy().to_string(),
            dest.to_string_lossy().to_string(),
        );
        spawn_task(decompress, |_| {})?;
        assert_result(&dest);

        Ok(())
    }
}
