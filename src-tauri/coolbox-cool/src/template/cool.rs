use super::Template;
use crate::cool2::{Cool2, Platform};
use crate::template::PlatformType;

impl Template {
    pub(crate) fn cool() -> Cool2 {
        Cool2 {
            name: Template::name().to_string(),
            identifier: Template::name().to_string(),
            description: Template::name().to_string(),
            macos: Some(Template::macos()),
            windows: Some(Template::win()),
            linux: Some(Template::linux()),
        }
    }

    pub(crate) fn macos() -> Platform {
        Platform {
            need_restart: false,
            formula: Template::formula(&PlatformType::MacOS),
        }
    }

    pub(crate) fn linux() -> Platform {
        Platform {
            need_restart: false,
            formula: Template::formula(&PlatformType::Linux),
        }
    }

    pub(crate) fn win() -> Platform {
        Platform {
            need_restart: false,
            formula: Template::formula(&PlatformType::Windows),
        }
    }
}
