// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Downloadable file information
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseForgeLaunchDataLibrariesItemDownloadsArtifact {
    /// Path that the file should be install to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// SHA1 of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    /// Size of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    /// URL of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}