// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_deploy_game_request_versions_item_modules_item_base_modpack::PostDeployGameRequestVersionsItemModulesItemBaseModpack;

/// Module group
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployGameRequestVersionsItemModulesItem {
    /// Base modpack to launch for this module group
    #[serde(rename = "baseModpack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_modpack: Option<PostDeployGameRequestVersionsItemModulesItemBaseModpack>,
    /// If this is the default module group for the version
    pub default: bool,
    /// Modules in this module group
    pub modules: Vec<String>,
    /// Name of this module group
    pub name: String,
    /// If this module group requires private access
    pub private: bool,
}