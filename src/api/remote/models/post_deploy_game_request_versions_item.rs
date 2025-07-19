// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_deploy_game_request_versions_item_assets::PostDeployGameRequestVersionsItemAssets;
use super::post_deploy_game_request_versions_item_modules_item::PostDeployGameRequestVersionsItemModulesItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployGameRequestVersionsItem {
    pub assets: PostDeployGameRequestVersionsItemAssets,
    /// If this is the default version
    pub default: bool,
    /// Display name of this version
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Major version group of this version
    pub group: String,
    /// ID of this version
    pub id: String,
    pub jres: serde_json::Value,
    /// Module groups available on this version
    pub modules: Vec<PostDeployGameRequestVersionsItemModulesItem>,
    /// Name of this version
    pub name: String,
    /// Sort order for this version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<f64>,
    /// If this version requires private access
    pub private: bool,
    /// If this is a Minecraft snapshot release
    pub snapshot: bool,
}