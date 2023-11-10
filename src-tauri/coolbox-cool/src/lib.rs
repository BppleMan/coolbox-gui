pub use color_eyre::*;
pub use crossbeam::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer};
pub use tracing::*;

pub use cool::*;
pub use cool_event::*;
pub use cool_list::*;
pub use extension::*;
pub use trace::*;

mod cool;
mod cool_event;
mod cool_list;
pub mod error;
mod extension;
pub mod installer;
pub mod result;
pub mod shell;
pub mod state;
pub mod tasks;
mod trace;

lazy_static! {
    pub static ref DEFAULT_TEMP_DIR: std::path::PathBuf = std::env::temp_dir();
    pub static ref DEFAULT_TERA_CONTEXT: tera::Context = {
        let mut ctx = tera::Context::default();
        ctx.insert(
            "TEMP_DIR",
            &format!("{}coolbox", &DEFAULT_TEMP_DIR.to_string_lossy()),
        );
        ctx
    };
}

pub fn render_str<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(d).map(|s| s.render(&DEFAULT_TERA_CONTEXT, false).unwrap())
}
