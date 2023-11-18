use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fmt, fs, io};

use color_eyre::eyre::eyre;
use schemars::JsonSchema;
use serde::ser::Error;
use serde::{Deserialize, Serialize};
use zip::result::ZipError;
use zip::ZipArchive;

use crate::error::ExecutableError;
use crate::result::ExecutableResult;
use crate::tasks::{Executable, MessageSender};
use crate::IntoInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct DecompressTask {
    #[serde(deserialize_with = "crate::template_string")]
    pub src: String,
    #[serde(deserialize_with = "crate::template_string")]
    pub dest: String,
}

impl DecompressTask {
    pub fn new(src: String, dest: String) -> Self {
        Self { src, dest }
    }

    pub fn decompress_zip(&self, mut send: Box<MessageSender>) -> ExecutableResult {
        let mut archive = ZipArchive::new(File::open(&self.src)?)?;
        let root_dirs = (0..archive.len()).try_fold(HashSet::new(), |mut set, i| {
            let entry = archive.by_index(i)?;
            let path = entry.enclosed_name().map(|p| p.to_path_buf());
            if let Some(Some(parent)) = path.as_ref().map(|p| p.parent()) {
                if parent.components().count() == 1 {
                    set.insert(parent.to_path_buf());
                }
            }
            Ok::<HashSet<PathBuf>, ZipError>(set)
        })?;
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
                archive.extract(&self.dest)?;
            }
            Some(root_dir) => {
                let dest = PathBuf::from_str(&self.dest)?;
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i)?;
                    let file_path = file.enclosed_name().unwrap();
                    let out_path = dest.join(file_path.strip_prefix(root_dir)?);
                    if file.name().ends_with('/') {
                        fs::create_dir_all(&out_path)?;
                        send(format!("create dir: {}", out_path.display()).into_info());
                    } else {
                        if let Some(parent) = out_path.parent() {
                            if !parent.exists() {
                                fs::create_dir_all(parent)?;
                                send(format!("create dir: {}", parent.display()).into_info());
                            }
                        }
                        if cfg!(unix)
                            && file.unix_mode().is_some()
                            && file.unix_mode().unwrap() & 0o120000 == 0o120000
                        {
                            use std::os::unix::fs::PermissionsExt;
                            let mut buf = String::new();
                            file.read_to_string(&mut buf)?;
                            std::os::unix::fs::symlink(buf, &out_path)?;
                            fs::set_permissions(
                                &out_path,
                                fs::Permissions::from_mode(file.unix_mode().unwrap()),
                            )?;
                            send(format!("create symlink: {}", out_path.display()).into_info());
                            continue;
                        }
                        let mut outfile = File::create(&out_path)?;
                        io::copy(&mut file, &mut outfile)?;
                        send(format!("create file: {}", out_path.display()).into_info());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn decompress_tar_gz(&self, mut send: Box<MessageSender>) -> ExecutableResult {
        let file = fs::File::open(&self.src)?;
        let dest = PathBuf::from_str(&self.dest)?;
        if dest.is_file() {
            return Err(ExecutableError::TargetIsFile(eyre!("{}", dest.display())));
        }
        let parent = dest
            .parent()
            .ok_or_else(|| ExecutableError::PathNoParent(eyre!("{}", dest.display())))?;
        if !parent.exists() {
            fs::create_dir_all(parent)?;
            send(format!("create dir: {}", parent.display()).into_info());
        }

        let decoder = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(decoder);
        fs::create_dir_all(&dest)?;
        send(format!("create dir: {}", dest.display()).into_info());
        let entries = archive.entries()?.flatten().collect::<Vec<_>>();
        let root_dirs = entries.iter().try_fold(HashSet::new(), |mut set, entry| {
            if let Some(parent) = entry.path()?.parent() {
                if parent.components().count() == 1 {
                    set.insert(parent.to_path_buf());
                }
            }
            Ok::<HashSet<PathBuf>, ZipError>(set)
        })?;
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
                    entry.unpack_in(&dest)?;
                    send(format!("create file: {}", entry.path()?.display()).into_info());
                }
                Some(root_dir) => {
                    let dest_path = dest.join(entry.path()?.strip_prefix(root_dir)?);
                    entry.unpack(&dest_path)?;
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
            Err(fmt::Error::custom(eyre!("Not support")))
        }
    }
}

impl<'a> Executable<'a> for DecompressTask {
    fn execute(&self, send: Box<MessageSender<'a>>) -> ExecutableResult {
        if self.src.ends_with(".zip") {
            self.decompress_zip(send)
        } else if self.src.ends_with(".tar.gz") {
            self.decompress_tar_gz(send)
        } else {
            Err(ExecutableError::UnsupportedCompressType(eyre!(
                "Not support: {}",
                self.src
            )))
        }
    }
}
