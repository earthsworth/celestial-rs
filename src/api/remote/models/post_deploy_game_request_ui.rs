// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// UI release to ship with this publish
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployGameRequestUi {
    /// Git branch for UI release
    pub branch: String,
    /// Git commit hash for UI release
    #[serde(rename = "commitHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_hash: Option<String>,
    /// If this is should use the latest UI release
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<bool>,
}