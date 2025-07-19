// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_launch_data_libraries_item_downloads_artifact::PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsArtifact;
use super::post_launcher_launch_vanilla_response_launch_data_libraries_item_downloads_classifiers::PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiers;

/// Downloadable artifact for the library
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloads {
    /// Downloadable file information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsArtifact>,
    /// Artifact classifiers for the library. Only appears in some libraries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemDownloadsClassifiers>,
}