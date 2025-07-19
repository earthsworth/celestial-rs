// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Artifact information relating to specific launch permutation
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LaunchTypeArtifact {
    /// Differential URL of the artifact asset (to use if previous hash exists)
    #[serde(rename = "differentialUrl")]
    pub differential_url: Option<String>,
    /// Last modification time of the artifact, in seconds since epoch
    pub mtime: f64,
    /// Name of the artifact
    pub name: String,
    /// Type of the artifact asset
    #[serde(rename = "type")]
    pub r#type: String,
    /// Hash of the artifact asset
    pub sha1: String,
    /// Size of the artifact, in bytes
    pub size: f64,
    /// URL of the artifact asset
    pub url: String,
}
