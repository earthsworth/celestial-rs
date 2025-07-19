// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// WebOSR assets
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchResponseUiAssets {
    /// Root URL (CDN) to fetch the prepend
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    /// Hash of the UI asset index
    #[serde(rename = "indexSha1")]
    pub index_sha1: String,
    /// URL of the UI asset index
    #[serde(rename = "indexUrl")]
    pub index_url: String,
}