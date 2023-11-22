use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use color_eyre::eyre::eyre;
use dashmap::DashMap;
use include_dir::{include_dir, Dir, DirEntry};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use tracing::error;

use crate::cool::state::CoolState;
use crate::error::CoolError;
use crate::result::CoolResult;
use crate::Cool;

#[derive(Debug, Clone)]
pub struct SafeCool(Arc<Mutex<Cool>>);

#[cfg(target_os = "macos")]
static PLATFORM_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/cools/macos");
#[cfg(target_os = "linux")]
static PLATFORM_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/cools/linux");
#[cfg(windows)]
static PLATFORM_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/cools/windows");
#[cfg(unix)]
static UNIX_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/cools/unix");

static UNIVERSAL_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/cools/universal");

pub static COOL_LIST: Lazy<DashMap<String, SafeCool>> = Lazy::new(|| {
    let map = DashMap::new();
    PLATFORM_DIR
        .find("**/*.*")
        .unwrap()
        .for_each(|entry| match parse_cool(entry) {
            Ok(cool) => {
                map.insert(cool.name.clone(), SafeCool::new(cool));
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
                map.insert(cool.name.clone(), SafeCool::new(cool));
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
                map.insert(cool.name.clone(), SafeCool::new(cool));
            }
            Err(e) => {
                error!("Error parsing cool: {}", e);
            }
        });
    map
});

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

impl SafeCool {
    pub fn new(cool: Cool) -> Self {
        Self(Arc::new(Mutex::new(cool)))
    }
}

impl Deref for SafeCool {
    type Target = Arc<Mutex<Cool>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for SafeCool {
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
        .map(|cool| {
            let cool = cool.lock().unwrap();
            (cool.name.clone(), cool.check())
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::result::CoolResult;
    use crate::{init_backtrace, COOL_LIST};

    #[test]
    fn test_cool_list() -> CoolResult<()> {
        init_backtrace();
        COOL_LIST
            .iter()
            .for_each(|c| println!("{:#?}", c.lock().unwrap()));
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
