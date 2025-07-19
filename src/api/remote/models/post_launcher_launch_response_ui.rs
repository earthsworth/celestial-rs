// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_response_ui_assets::PostLauncherLaunchResponseUiAssets;

/// WebOSR UI related information
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchResponseUi {
    /// WebOSR assets
    pub assets: PostLauncherLaunchResponseUiAssets,
    /// Hash of the WebOSR source
    #[serde(rename = "sourceSha1")]
    pub source_sha1: String,
    /// URL of the WebOSR source
    #[serde(rename = "sourceUrl")]
    pub source_url: String,
}