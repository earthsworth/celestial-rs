// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Base modpack to launch for this module group
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployGameRequestVersionsItemModulesItemBaseModpack {
    /// Mod loader this module group uses
    pub loader: String,
    /// A Semantic Versioning version.
    pub version: String,
}