use super::Template;
use crate::CoolVersion;

impl Template {
    pub(crate) fn version() -> CoolVersion {
        CoolVersion::Latest
    }
}
