use super::Template;
use crate::shell::Shell;
use crate::template::PlatformType;

impl Template {
    pub(crate) fn shell(plat: &PlatformType) -> Shell {
        match plat {
            PlatformType::MacOS => Shell::zsh(),
            PlatformType::Linux => Shell::bash(),
            PlatformType::Windows => Shell::cmd(),
        }
    }
}
