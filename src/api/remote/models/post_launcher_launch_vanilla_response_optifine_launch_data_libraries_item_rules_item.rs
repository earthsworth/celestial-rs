// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_optifine_launch_data_libraries_item_rules_item_os::PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemRulesItemOs;

/// JVM rule for the flag
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemRulesItem {
    /// Action of the rule (can be 'allow' or 'disallow')
    pub action: String,
    /// OS and architecture of the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemRulesItemOs>,
}