use std::sync::{Arc, RwLock};

use color_eyre::eyre::eyre;
use dashmap::DashMap;
use include_dir::{include_dir, Dir};
use once_cell::sync::Lazy;
use tracing::error;

use crate::Models;

pub type SafeCool = Arc<RwLock<Models>>;

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
                match toml::from_str::<Models>(entry.as_file().unwrap().contents_utf8().unwrap()) {
                    Ok(cool) => {
                        map.insert(cool.name.clone(), Arc::new(RwLock::new(cool)));
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

#[cfg(test)]
mod test {
    use crate::result::CoolResult;
    use crate::{init_backtrace, COOL_LIST};

    #[test]
    fn test_cool_list() -> CoolResult<()> {
        init_backtrace();
        println!("{:#?}", COOL_LIST);
        Ok(())
    }
}
