// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherServerModeFilter {
    /// Icon representing mode
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    /// Name of the mode
    pub name: String,
}