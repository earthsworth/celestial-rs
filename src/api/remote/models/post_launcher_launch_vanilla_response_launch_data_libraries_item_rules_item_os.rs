// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// OS and architecture of the rule
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseLaunchDataLibrariesItemRulesItemOs {
    /// OS architecture of the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    /// Operating system name of the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}