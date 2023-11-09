use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use color_eyre::eyre::eyre;
use dashmap::DashMap;
use include_dir::{Dir, include_dir};
use once_cell::sync::Lazy;
use tracing::error;

use crate::Cool;
use crate::error::TransformError;

#[derive(Debug, Clone)]
pub struct SafeCool(Arc<RwLock<Cool>>);

static COOL_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/cools");

pub static COOL_LIST: Lazy<DashMap<String, SafeCool>> = Lazy::new(|| {
    let map = DashMap::new();
    COOL_DIR.find("**/*.toml").unwrap().for_each(|entry| {
        if cfg!(target_os = "macos") {
            let parent = entry.path().parent().unwrap().to_string_lossy().to_string();
            if &parent == "brew"
                || &parent == "universal"
                || &parent == "cargo"
                || &parent == "flutter"
                || &parent == "shell"
            {
                match toml::from_str::<Cool>(entry.as_file().unwrap().contents_utf8().unwrap()) {
                    Ok(cool) => {
                        map.insert(cool.name.clone(), SafeCool(Arc::new(RwLock::new(cool))));
                    }
                    Err(e) => {
                        error!("{:?}\n{:?}", entry.path(), eyre!(e));
                    }
                }
            }
        } else {
            println!("is not macos");
        }
    });
    map
});

impl Deref for SafeCool {
    type Target = Arc<RwLock<Cool>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for SafeCool {
    type Err = TransformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match COOL_LIST.get(s) {
            None => Err(TransformError::NotFoundCool(s.to_string())),
            Some(cool) => Ok(cool.value().clone()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{COOL_LIST, init_backtrace};
    use crate::result::CoolResult;

    #[test]
    fn test_cool_list() -> CoolResult<()> {
        init_backtrace();
        println!("{:#?}", COOL_LIST);
        Ok(())
    }
}
