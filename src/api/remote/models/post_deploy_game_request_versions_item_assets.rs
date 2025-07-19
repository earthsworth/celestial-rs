// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployGameRequestVersionsItemAssets {
    /// Mojang asset ID for this version
    pub id: String,
    /// SHA1 hash of Mojang asset index
    pub sha1: String,
    /// URL for Mojang asset index for this version
    pub url: String,
}