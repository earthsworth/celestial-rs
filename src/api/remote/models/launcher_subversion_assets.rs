// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Assets that need to be downloaded with this subversion
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherSubversionAssets {
    /// Identifier of the asset
    pub id: String,
    /// Hash of the asset
    pub sha1: String,
    /// URL of the asset location
    pub url: String,
}