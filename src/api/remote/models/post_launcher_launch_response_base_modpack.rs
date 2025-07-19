// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Launch Base Modpack
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchResponseBaseModpack {
    /// Unique hash for base modpack
    pub hash: String,
    /// URL for base modpack's .mrpack download
    #[serde(rename = "mrpackUrl")]
    pub mrpack_url: String,
    /// Time the base modpack was published
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    /// Version of the base modpack
    pub version: String,
}