// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Language information
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetLauncherMetadataMiscResponseLanguage {
    /// Map of language codes to their latest SHA1 hashes
    #[serde(rename = "latestHashes")]
    pub latest_hashes: serde_json::Value,
}