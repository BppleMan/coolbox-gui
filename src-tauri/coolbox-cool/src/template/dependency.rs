use super::Template;
use crate::CoolDependency;
use std::collections::HashSet;

impl Template {
    pub(crate) fn dependencies() -> HashSet<CoolDependency> {
        (0..10)
            .map(|i| CoolDependency::new(format!("dependency-{}", i), Template::version()))
            .collect()
    }
}
