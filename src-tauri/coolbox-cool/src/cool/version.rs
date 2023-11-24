use schemars::JsonSchema;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub enum CoolVersion {
    Latest,
    Specific(String),
}

impl Serialize for CoolVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CoolVersion::Latest => serializer.serialize_str("latest"),
            CoolVersion::Specific(version) => serializer.serialize_str(version),
        }
    }
}

impl<'de> Deserialize<'de> for CoolVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version = String::deserialize(deserializer)?;
        if version == "latest" {
            Ok(CoolVersion::Latest)
        } else {
            Ok(CoolVersion::Specific(version))
        }
    }
}
