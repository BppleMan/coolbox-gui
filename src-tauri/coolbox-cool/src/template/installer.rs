use super::Template;
use crate::installer::Installer;
use crate::template::PlatformType;

impl Template {
    pub(crate) fn installer(plat: &PlatformType) -> Installer {
        match plat {
            PlatformType::MacOS => Installer::brew(),
            PlatformType::Linux => Installer::apt(),
            PlatformType::Windows => Installer::win_get(),
        }
    }
}
