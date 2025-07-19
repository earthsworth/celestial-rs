// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployModpacksRequestModpacksItem {
    /// Minecraft version this modpack supports
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    /// Unique hash for this version of the modpack
    pub hash: String,
    /// Mod loader this modpack supports
    pub loader: String,
    /// URL to download the .mrpack for this modpack
    #[serde(rename = "mrpackUrl")]
    pub mrpack_url: String,
    /// A Semantic Versioning version.
    pub version: String,
}