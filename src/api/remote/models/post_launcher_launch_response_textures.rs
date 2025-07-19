// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Textures location for specific launch configuration
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchResponseTextures {
    /// Root URL (CDN) to fetch the prepend
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    /// Hash of the textures index
    #[serde(rename = "indexSha1")]
    pub index_sha1: String,
    /// URL of the textures index
    #[serde(rename = "indexUrl")]
    pub index_url: String,
    /// Hash of the just-in-time textures index
    #[serde(rename = "jitIndexSha1")]
    pub jit_index_sha1: String,
    /// URL of the just-in-time textures index
    #[serde(rename = "jitIndexUrl")]
    pub jit_index_url: String,
}