// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_libraries_item_downloads_artifact::PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsArtifact;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_libraries_item_downloads_classifiers::PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiers;

/// Downloadable artifact for the library
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloads {
    /// Downloadable file information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsArtifact>,
    /// Artifact classifiers for the library. Only appears in some libraries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItemDownloadsClassifiers>,
}