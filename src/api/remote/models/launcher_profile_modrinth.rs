// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Information about the Modrinth-linked modpack
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherProfileModrinth {
    /// Modrinth Project ID
    pub id: String,
    /// Version ID of the Modrinth Project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}