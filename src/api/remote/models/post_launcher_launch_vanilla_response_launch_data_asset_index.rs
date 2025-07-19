// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Downloadable file information
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseLaunchDataAssetIndex {
    /// ID of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Path that the file should be install to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// SHA1 of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    /// Size of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    /// Total size of the file
    #[serde(rename = "totalSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<f64>,
    /// URL of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}