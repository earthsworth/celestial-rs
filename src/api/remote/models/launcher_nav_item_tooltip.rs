// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Tooltip to show on hover
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherNavItemTooltip {
    /// Subtitle of the tooltip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// Title of the tooltip
    pub title: String,
}