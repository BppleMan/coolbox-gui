pub use cool::*;
pub use dependency::*;
pub use formula::*;
pub use installer::*;
pub use shell::*;
pub use task::*;
pub use version::*;

mod cool;
mod dependency;
mod formula;
mod installer;
mod shell;
mod task;
mod version;

pub struct Template;

pub(crate) enum PlatformType {
    MacOS,
    Linux,
    Windows,
}

#[cfg(test)]
mod test {
    use crate::cool2::Cool2;
    use crate::init_backtrace;
    use crate::result::CoolResult;
    use color_eyre::eyre::Context;
    use std::path::PathBuf;

    #[test]
    fn generate_template() -> CoolResult<()> {
        init_backtrace();
        let template = super::Template::cool();

        let content = serde_yaml::to_string(&template)?;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/cools2/template.yaml");
        fs_extra::file::write_all(&path, &content).with_context(|| path.display().to_string())?;
        serde_yaml::from_str::<Cool2>(fs_extra::file::read_to_string(&path)?.as_str())?;

        let content = toml::to_string(&template)?;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/cools2/template.toml");
        fs_extra::file::write_all(&path, &content).with_context(|| path.display().to_string())?;

        let content = serde_json::to_string_pretty(&template)?;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/cools2/template.json");
        fs_extra::file::write_all(&path, &content).with_context(|| path.display().to_string())?;
        Ok(())
    }
}
