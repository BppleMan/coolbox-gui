use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use color_eyre::eyre::eyre;
use dashmap::DashMap;
use include_dir::{include_dir, Dir};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use tracing::error;

use crate::error::CoolError;
use crate::state::CoolState;
use crate::Cool;

#[derive(Debug, Clone)]
pub struct SafeCool(Arc<Mutex<Cool>>);

static COOL_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/cools");

pub static COOL_LIST: Lazy<DashMap<String, SafeCool>> = Lazy::new(|| {
    let map = DashMap::new();
    COOL_DIR.find("**/*.*").unwrap().for_each(|entry| {
        if cfg!(target_os = "macos") {
            let parent = entry.path().parent().unwrap().to_string_lossy().to_string();
            if &parent == "brew"
                || &parent == "universal"
                || &parent == "cargo"
                || &parent == "flutter"
                || &parent == "shell"
                || &parent == "env"
            {
                let file = entry.as_file().unwrap();
                let file_name = file
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                match file_name {
                    file_name if file_name.ends_with(".toml") => {
                        match toml::from_str::<Cool>(
                            entry.as_file().unwrap().contents_utf8().unwrap(),
                        ) {
                            Ok(cool) => {
                                map.insert(cool.name.clone(), SafeCool(Arc::new(Mutex::new(cool))));
                            }
                            Err(e) => {
                                error!("{:?}\n{:?}", entry.path(), eyre!(e));
                            }
                        }
                    }
                    file_name if file_name.ends_with(".yaml") => {
                        match serde_yaml::from_str::<Cool>(
                            entry.as_file().unwrap().contents_utf8().unwrap(),
                        ) {
                            Ok(cool) => {
                                map.insert(cool.name.clone(), SafeCool(Arc::new(Mutex::new(cool))));
                            }
                            Err(e) => {
                                error!("{:?}\n{:?}", entry.path(), eyre!(e));
                            }
                        }
                    }
                    _ => error!("Unsupported file type: {}", file.path().display()),
                }
            }
        } else {
            error!("is not macos");
        }
    });
    map
});

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
    use crate::result::CoolResult;
    use crate::{init_backtrace, COOL_LIST};
    use std::path::Path;

    #[test]
    fn test_cool_list() -> CoolResult<()> {
        init_backtrace();
        COOL_LIST
            .iter()
            .for_each(|c| println!("{:?}", c.lock().unwrap()));
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
