use std::path::Path;
use std::str::FromStr;

use crate::cool::Cool;
use color_eyre::eyre::eyre;
use color_eyre::Report;
use dashmap::DashMap;
use include_dir::{include_dir, Dir, DirEntry};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use tracing::error;
use walkdir::WalkDir;

use crate::cool::state::CoolState;
use crate::error::CoolError;
use crate::local_storage::LOCAL_STORAGE;
use crate::result::CoolResult;

#[cfg(target_os = "macos")]
static PLATFORM_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/cools/macos");
#[cfg(target_os = "linux")]
static PLATFORM_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/cools/linux");
#[cfg(windows)]
static PLATFORM_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/cools/windows");
#[cfg(unix)]
static UNIX_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/cools/unix");

static UNIVERSAL_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/cools/universal");

pub static COOL_LIST: Lazy<DashMap<String, Cool>> = Lazy::new(|| {
    let map = DashMap::new();
    PLATFORM_DIR
        .find("**/*.*")
        .unwrap()
        .for_each(|entry| match parse_cool(entry) {
            Ok(cool) => {
                map.insert(cool.name.clone(), cool);
            }
            Err(e) => {
                error!("Error parsing cool: {}", e);
            }
        });
    UNIVERSAL_DIR
        .find("**/*.*")
        .unwrap()
        .for_each(|entry| match parse_cool(entry) {
            Ok(cool) => {
                map.insert(cool.name.clone(), cool);
            }
            Err(e) => {
                error!("Error parsing cool: {}", e);
            }
        });
    #[cfg(unix)]
    UNIX_DIR
        .find("**/*.*")
        .unwrap()
        .for_each(|entry| match parse_cool(entry) {
            Ok(cool) => {
                map.insert(cool.name.clone(), cool);
            }
            Err(e) => {
                error!("Error parsing cool: {}", e);
            }
        });
    find_cool(&map, LOCAL_STORAGE.cools());
    map
});

fn find_cool(map: &DashMap<String, Cool>, dir: impl AsRef<Path>) {
    WalkDir::new(dir)
        .min_depth(1)
        .follow_links(true)
        .into_iter()
        .flatten()
        .for_each(|entry| match Cool::try_from(entry) {
            Ok(cool) => {
                map.insert(cool.name.clone(), cool);
            }
            Err(e) => {
                error!("Error parsing cool: {:?}", e);
            }
        });
}

fn parse_cool(entry: &DirEntry) -> CoolResult<Cool> {
    let file = entry.as_file().unwrap();
    let file_name = file
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let cool = match file_name {
        file_name if file_name.ends_with(".toml") => {
            toml::from_str::<Cool>(entry.as_file().unwrap().contents_utf8().unwrap())?
        }
        file_name if file_name.ends_with(".yaml") => {
            serde_yaml::from_str(entry.as_file().unwrap().contents_utf8().unwrap())?
        }
        file_name if file_name.ends_with(".json") => {
            serde_json::from_str(entry.as_file().unwrap().contents_utf8().unwrap())?
        }
        _ => Err(eyre!("Unsupported file type: {}", file.path().display()))?,
    };
    Ok(cool)
}

impl<'a> TryFrom<include_dir::DirEntry<'a>> for Cool {
    type Error = Report;

    fn try_from(entry: include_dir::DirEntry<'a>) -> Result<Self, Self::Error> {
        let file = entry.as_file().unwrap();
        let file_name = file
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let content = file.contents_utf8().unwrap();
        let cool = match file_name {
            file_name if file_name.ends_with(".toml") => toml::from_str::<Cool>(content)?,
            file_name if file_name.ends_with(".yaml") => serde_yaml::from_str(content)?,
            file_name if file_name.ends_with(".json") => serde_json::from_str(content)?,
            _ => Err(CoolError::UnsupportedCoolFile {
                file: file.path().display().to_string(),
            })?,
        };
        Ok(cool)
    }
}

impl TryFrom<walkdir::DirEntry> for Cool {
    type Error = Report;

    fn try_from(entry: walkdir::DirEntry) -> Result<Self, Self::Error> {
        if entry.file_type().is_file() || entry.file_type().is_symlink() {
            let content = fs_extra::file::read_to_string(entry.path())?;
            let cool = match entry.file_name().to_string_lossy() {
                file_name if file_name.ends_with(".toml") => toml::from_str::<Cool>(&content)?,
                file_name if file_name.ends_with(".yaml") => serde_yaml::from_str(&content)?,
                file_name if file_name.ends_with(".json") => serde_json::from_str(&content)?,
                _ => Err(CoolError::UnsupportedCoolFile {
                    file: entry.path().display().to_string(),
                })?,
            };
            Ok(cool)
        } else {
            Err(CoolError::UnsupportedCoolFile {
                file: entry.path().display().to_string(),
            })?
        }
    }
}

impl FromStr for Cool {
    type Err = CoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match COOL_LIST.get(s) {
            None => Err(CoolError::NotFoundCool {
                cool_name: s.to_string(),
            }),
            Some(cool) => Ok(cool.value().clone()),
        }
    }
}

pub fn check_cool_states() -> Vec<(String, CoolState)> {
    COOL_LIST
        .par_iter()
        .map(|cool| (cool.name.clone(), cool.check()))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::cool::COOL_LIST;
    use crate::init_backtrace;
    use crate::result::CoolResult;

    #[test]
    fn test_cool_list() -> CoolResult<()> {
        init_backtrace();
        COOL_LIST.iter().for_each(|c| println!("{:#?}", c.value()));
        Ok(())
    }

    #[test]
    fn test_glob() -> CoolResult<()> {
        init_backtrace();
        let contents = fs_extra::dir::get_dir_content("assets/cools")?;
        let pattern = glob::Pattern::new("**/*.{toml,yaml}")?;
        contents
            .files
            .iter()
            .for_each(|c| println!("{:?} {}", c, pattern.matches_path(Path::new(c))));
        Ok(())
    }
}
