// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_forge_launch_data_libraries_item_downloads_artifact::PostLauncherLaunchVanillaResponseForgeLaunchDataLibrariesItemDownloadsArtifact;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseForgeLaunchDataLibrariesItemDownloads {
    /// Downloadable file information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<PostLauncherLaunchVanillaResponseForgeLaunchDataLibrariesItemDownloadsArtifact>,
}