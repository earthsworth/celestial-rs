// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Fabric file metadata
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseFabricLaunchDataLibrariesItem {
    /// MD5 hash of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// Name of the file
    pub name: String,
    /// SHA1 hash of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    /// SHA256 hash of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    /// SHA512 hash of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha512: Option<String>,
    /// Size of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    /// URL of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}