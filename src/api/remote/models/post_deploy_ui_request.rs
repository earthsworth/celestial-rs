// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployUiRequest {
    /// Git branch for this publish
    pub branch: String,
    /// Full Git commit hash for this publish
    #[serde(rename = "gitCommit")]
    pub git_commit: String,
    /// SHA1 Hash of index file produced
    #[serde(rename = "indexSha1")]
    pub index_sha1: String,
    /// SHA1 Hash of source zip produced
    #[serde(rename = "sourceSha1")]
    pub source_sha1: String,
}