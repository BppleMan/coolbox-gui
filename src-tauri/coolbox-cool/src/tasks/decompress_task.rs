use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

use color_eyre::eyre::eyre;
use schemars::JsonSchema;
use serde::ser::Error;
use serde::{Deserialize, Serialize};
use zip::result::ZipError;
use zip::ZipArchive;

use crate::cool::IntoInfo;
use crate::error::{CompressTaskError, InnerError, TaskError};
use crate::result::CoolResult;
use crate::tasks::{Executable, MessageSender};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct DecompressTask {
    #[serde(deserialize_with = "crate::cool::template_string")]
    pub src: String,
    #[serde(deserialize_with = "crate::cool::template_string")]
    pub dest: String,
}

impl DecompressTask {
    pub fn new(src: String, dest: String) -> Self {
        Self { src, dest }
    }

    fn map_inner_error(&self, e: impl Into<InnerError>) -> TaskError {
        TaskError::DecompressTaskError {
            task: self.clone(),
            source: CompressTaskError::InnerError(e.into()),
        }
    }

    pub fn decompress_zip(&self, mut send: Box<MessageSender>) -> CoolResult<(), TaskError> {
        let mut archive =
            ZipArchive::new(File::open(&self.src).map_err(|e| self.map_inner_error(e))?)
                .map_err(|e| self.map_inner_error(e))?;
        let root_dirs = (0..archive.len())
            .try_fold(HashSet::new(), |mut set, i| {
                let entry = archive.by_index(i)?;
                let path = entry.enclosed_name().map(|p| p.to_path_buf());
                if let Some(Some(parent)) = path.as_ref().map(|p| p.parent()) {
                    if parent.components().count() == 1 {
                        set.insert(parent.to_path_buf());
                    }
                }
                Ok::<HashSet<PathBuf>, ZipError>(set)
            })
            .map_err(|e| self.map_inner_error(e))?;
        let root_dir = if root_dirs.len() == 1
            && !root_dirs
                .iter()
                .next()
                .unwrap()
                .display()
                .to_string()
                .is_empty()
        {
            root_dirs.into_iter().next()
        } else {
            None
        };
        match root_dir.as_ref() {
            None => {
                archive
                    .extract(&self.dest)
                    .map_err(|e| self.map_inner_error(e))?;
            }
            Some(root_dir) => {
                let dest = PathBuf::from_str(&self.dest).map_err(|e| self.map_inner_error(e))?;
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i).map_err(|e| self.map_inner_error(e))?;
                    let file_path = file.enclosed_name().unwrap();
                    let out_path = dest.join(
                        file_path
                            .strip_prefix(root_dir)
                            .map_err(|e| self.map_inner_error(e))?,
                    );
                    if file.name().ends_with('/') {
                        fs_extra::dir::create_all(&out_path, true)
                            .map_err(|e| self.map_inner_error(e))?;
                        send(format!("Decompress dir: {}", out_path.display()).into_info());
                    } else {
                        if let Some(parent) = out_path.parent() {
                            if !parent.exists() {
                                fs_extra::dir::create_all(&out_path, true)
                                    .map_err(|e| self.map_inner_error(e))?;
                                send(format!("Decompress dir: {}", parent.display()).into_info());
                            }
                        }
                        #[cfg(unix)]
                        if file.unix_mode().is_some()
                            && file.unix_mode().unwrap() & 0o120000 == 0o120000
                        {
                            use std::os::unix::fs::PermissionsExt;
                            let mut buf = String::new();
                            file.read_to_string(&mut buf)
                                .map_err(|e| self.map_inner_error(e))?;
                            std::os::unix::fs::symlink(buf, &out_path)
                                .map_err(|e| self.map_inner_error(e))?;
                            std::fs::set_permissions(
                                &out_path,
                                std::fs::Permissions::from_mode(file.unix_mode().unwrap()),
                            )
                            .map_err(|e| self.map_inner_error(e))?;
                            send(format!("Decompress symlink: {}", out_path.display()).into_info());
                            continue;
                        }
                        let mut outfile =
                            File::create(&out_path).map_err(|e| self.map_inner_error(e))?;
                        std::io::copy(&mut file, &mut outfile)
                            .map_err(|e| self.map_inner_error(e))?;
                        send(format!("Decompress file: {}", out_path.display()).into_info());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn decompress_tar_gz(&self, mut send: Box<MessageSender>) -> CoolResult<(), TaskError> {
        let file = std::fs::File::open(&self.src).map_err(|e| self.map_inner_error(e))?;
        let dest = PathBuf::from_str(&self.dest).map_err(|e| self.map_inner_error(e))?;
        if dest.is_file() {
            return Err(TaskError::DecompressTaskError {
                task: self.clone(),
                source: CompressTaskError::DestIsFile(self.dest.clone()),
            });
        }
        let parent = dest
            .parent()
            .ok_or_else(|| TaskError::DecompressTaskError {
                task: self.clone(),
                source: CompressTaskError::DestNoParent(self.dest.clone()),
            })?;
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| self.map_inner_error(e))?;
            send(format!("create dir: {}", parent.display()).into_info());
        }

        let decoder = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(decoder);
        std::fs::create_dir_all(&dest).map_err(|e| self.map_inner_error(e))?;
        send(format!("create dir: {}", dest.display()).into_info());
        let entries = archive
            .entries()
            .map_err(|e| self.map_inner_error(e))?
            .flatten()
            .collect::<Vec<_>>();
        let root_dirs = entries
            .iter()
            .try_fold(HashSet::new(), |mut set, entry| {
                if let Some(parent) = entry.path()?.parent() {
                    if parent.components().count() == 1 {
                        set.insert(parent.to_path_buf());
                    }
                }
                Ok::<HashSet<PathBuf>, ZipError>(set)
            })
            .map_err(|e| self.map_inner_error(e))?;
        let root_dir = if root_dirs.len() == 1
            && !root_dirs
                .iter()
                .next()
                .unwrap()
                .display()
                .to_string()
                .is_empty()
        {
            root_dirs.into_iter().next()
        } else {
            None
        };
        for mut entry in entries {
            match root_dir.as_ref() {
                None => {
                    entry
                        .unpack_in(&dest)
                        .map_err(|e| self.map_inner_error(e))?;
                    send(
                        format!(
                            "create file: {}",
                            entry.path().map_err(|e| self.map_inner_error(e))?.display()
                        )
                        .into_info(),
                    );
                }
                Some(root_dir) => {
                    let dest_path = dest.join(
                        entry
                            .path()
                            .map_err(|e| self.map_inner_error(e))?
                            .strip_prefix(root_dir)
                            .map_err(|e| self.map_inner_error(e))?,
                    );
                    entry
                        .unpack(&dest_path)
                        .map_err(|e| self.map_inner_error(e))?;
                    send(format!("create file: {}", dest_path.display()).into_info());
                }
            }
        }
        Ok(())
    }
}

impl Display for DecompressTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.src.ends_with(".zip") {
            write!(f, "unzip {} -d {}", self.src, self.dest)
        } else if self.src.ends_with(".tar.gz") {
            write!(f, "tar -xzf {} -C {}", self.src, self.dest)
        } else {
            Err(std::fmt::Error::custom(eyre!("Not support")))
        }
    }
}

impl<'a> Executable<'a> for DecompressTask {
    fn execute(&self, send: Box<MessageSender<'a>>) -> CoolResult<(), TaskError> {
        if self.src.ends_with(".zip") {
            self.decompress_zip(send)
        } else if self.src.ends_with(".tar.gz") {
            self.decompress_tar_gz(send)
        } else {
            Err(TaskError::DecompressTaskError {
                task: self.clone(),
                source: CompressTaskError::UnsupportedCompressType(self.src.clone()),
            })
        }
    }
}
