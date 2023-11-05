use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fmt, fs, io};

use color_eyre::eyre::eyre;
use color_eyre::Report;
use serde::ser::Error;
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use coolbox_macros::State;

use crate::result::CoolResult;
use crate::tasks::{Executable, ExecutableState};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, State)]
pub struct DecompressTask {
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

impl DecompressTask {
    pub fn new(src: String, dest: String) -> Self {
        Self {
            src,
            dest,
            state: ExecutableState::NotStarted,
            outputs: vec![],
            errors: vec![],
        }
    }

    pub fn decompress_zip(&self) -> CoolResult<()> {
        let mut archive = ZipArchive::new(File::open(&self.src)?)?;
        let root_dirs = (0..archive.len()).try_fold(HashSet::new(), |mut set, i| {
            let entry = archive.by_index(i)?;
            let path = entry.enclosed_name().map(|p| p.to_path_buf());
            if let Some(Some(parent)) = path.as_ref().map(|p| p.parent()) {
                if parent.components().count() == 1 {
                    set.insert(parent.to_path_buf());
                }
            }
            Ok::<HashSet<PathBuf>, Report>(set)
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
                    } else {
                        if let Some(parent) = out_path.parent() {
                            if !parent.exists() {
                                fs::create_dir_all(parent)?;
                            }
                        }
                        if cfg!(target_os = "unix")
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
                            continue;
                        }
                        let mut outfile = File::create(&out_path)?;
                        io::copy(&mut file, &mut outfile)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn decompress_tar_gz(&self) -> CoolResult<()> {
        let file = fs::File::open(&self.src)?;
        let dest = PathBuf::from_str(&self.dest)?;
        if dest.is_file() {
            return Err(eyre!("[{}] is file", dest.display()));
        }
        let parent = dest.parent().ok_or_else(|| eyre!("No parent"))?;
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }

        let decoder = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(decoder);
        fs::create_dir_all(&dest)?;
        let entries = archive.entries()?.flatten().collect::<Vec<_>>();
        let root_dirs = entries.iter().try_fold(HashSet::new(), |mut set, entry| {
            if let Some(parent) = entry.path()?.parent() {
                if parent.components().count() == 1 {
                    set.insert(parent.to_path_buf());
                }
            }
            Ok::<HashSet<PathBuf>, Report>(set)
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
                }
                Some(root_dir) => {
                    let dest_path = dest.join(entry.path()?.strip_prefix(root_dir)?);
                    entry.unpack(dest_path)?;
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

impl Executable for DecompressTask {
    fn _run(&mut self) -> CoolResult<()> {
        if self.src.ends_with(".zip") {
            self.decompress_zip()
        } else if self.src.ends_with(".tar.gz") {
            self.decompress_tar_gz()
        } else {
            Err(eyre!("Not support"))
        }
    }
}
