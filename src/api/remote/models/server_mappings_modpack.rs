// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Server's modpack
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct ServerMappingsModpack {
    /// Modrinth ID of the modpack
    pub id: String,
    /// If true, the user will be prompted to install the recommended mods/modpack before joining the server in-game.
    #[serde(rename = "promptBeforeGameJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_before_game_join: Option<bool>,
    /// If true, the user will be prompted to install the recommended mods/modpack before joining the server in-game.
    #[serde(rename = "promptBeforeLauncherJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_before_launcher_join: Option<bool>,
    /// If blank, will use 'Fabric' loader or the next available loader if Fabric is not available.
    #[serde(rename = "recommendedLoader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_loader: Option<String>,
    /// If blank, will use 'Release' release channel.
    #[serde(rename = "recommendedReleaseChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_release_channel: Option<String>,
    /// The recommended Modrinth Modpack version. If blank, will use latest version.
    #[serde(rename = "recommendedVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_version: Option<String>,
}