use crate::StringExt;
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer};

lazy_static! {
    pub static ref DEFAULT_TEMP_DIR: std::path::PathBuf = std::env::temp_dir();
    pub static ref DEFAULT_TERA_CONTEXT: tera::Context = {
        let mut ctx = tera::Context::default();
        ctx.insert(
            "TEMP_DIR",
            &DEFAULT_TEMP_DIR.join("cool").to_string_lossy().to_string(),
        );
        ctx.insert(
            "CURRENT_DIR",
            &std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .to_string_lossy(),
        );
        ctx
    };
}

pub fn template_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(d).map(|s| s.render(&DEFAULT_TERA_CONTEXT, false).unwrap())
}

pub fn template_args<'de, D>(d: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<Vec<String>>::deserialize(d).map(|args| {
        args.map(|args| {
            args.into_iter()
                .map(|arg| arg.render(&DEFAULT_TERA_CONTEXT, false).unwrap())
                .collect::<Vec<String>>()
        })
    })
}

pub fn template_envs<'de, D>(d: D) -> Result<Option<Vec<(String, String)>>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<Vec<(String, String)>>::deserialize(d).map(|envs| {
        envs.map(|envs| {
            envs.into_iter()
                .map(|(k, v)| {
                    (
                        k.render(&DEFAULT_TERA_CONTEXT, false).unwrap(),
                        v.render(&DEFAULT_TERA_CONTEXT, false).unwrap(),
                    )
                })
                .collect::<Vec<(String, String)>>()
        })
    })
}
