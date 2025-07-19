// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_optifine_launch_data_libraries_item_downloads_artifact::PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsArtifact;
use super::post_launcher_launch_vanilla_response_optifine_launch_data_libraries_item_downloads_classifiers::PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiers;

/// Downloadable artifact for the library
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloads {
    /// Downloadable file information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsArtifact>,
    /// Artifact classifiers for the library. Only appears in some libraries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloadsClassifiers>,
}