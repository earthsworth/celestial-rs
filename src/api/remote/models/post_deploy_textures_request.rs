// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployTexturesRequest {
    /// Git branch for this publish
    pub branch: String,
    /// Full Git commit hash for this publish
    #[serde(rename = "gitCommit")]
    pub git_commit: String,
    /// Hash of texture index file produced
    #[serde(rename = "indexHash")]
    pub index_hash: String,
    /// Hash of JIT texture index file produced
    #[serde(rename = "jitIndexHash")]
    pub jit_index_hash: String,
    /// If this deploy was manually triggered
    pub manual: bool,
}