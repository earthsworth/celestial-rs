// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_profile_modrinth::LauncherProfileModrinth;

/// The profile attempting to be launched
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherProfile {
    /// Local profile ID
    pub id: String,
    /// Information about the Modrinth-linked modpack
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modrinth: Option<LauncherProfileModrinth>,
    /// Name of the profile
    pub name: String,
}