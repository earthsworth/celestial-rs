// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_nav_item_tooltip::LauncherNavItemTooltip;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherNavItem {
    /// Where the navigation item will take users (can be a deep link)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// Identifier for navigiation item
    pub id: String,
    /// Image to display as icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Tooltip to show on hover
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<LauncherNavItemTooltip>,
}