use super::Template;
use crate::template::PlatformType;
use crate::CoolFormula;

impl Template {
    pub(crate) fn formula(plat: &PlatformType) -> CoolFormula {
        match plat {
            PlatformType::MacOS => Template::package_manager(plat),
            PlatformType::Linux => Template::package_manager(plat),
            PlatformType::Windows => Template::manual(plat),
        }
    }

    pub(crate) fn package_manager(plat: &PlatformType) -> CoolFormula {
        CoolFormula::package_manager(
            Template::installer(plat),
            Template::name(),
            Template::tasks(plat),
            Template::tasks(plat),
        )
    }

    pub(crate) fn manual(plat: &PlatformType) -> CoolFormula {
        CoolFormula::manual(
            Template::dependencies(),
            Template::tasks(plat),
            Template::tasks(plat),
            Template::tasks(plat),
        )
    }
}
