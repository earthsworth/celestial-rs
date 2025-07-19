// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_deploy_game_request_modules_item::PostDeployGameRequestModulesItem;
use super::post_deploy_game_request_recommended_libraries_item::PostDeployGameRequestRecommendedLibrariesItem;
use super::post_deploy_game_request_ui::PostDeployGameRequestUi;
use super::post_deploy_game_request_versions_item::PostDeployGameRequestVersionsItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployGameRequest {
    /// All artifacts available
    pub artifacts: Vec<serde_json::Value>,
    /// Git branch for this publish
    pub branch: String,
    /// Full Git commit hash for this publish
    #[serde(rename = "commitHash")]
    pub commit_hash: String,
    /// Java main class to invoke when launching
    #[serde(rename = "mainClass")]
    pub main_class: String,
    /// Module groups available
    pub modules: Vec<PostDeployGameRequestModulesItem>,
    /// Recommended library versions
    #[serde(rename = "recommendedLibraries")]
    pub recommended_libraries: Vec<PostDeployGameRequestRecommendedLibrariesItem>,
    /// UI release to ship with this publish
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<PostDeployGameRequestUi>,
    /// If this publish should use the canary system or directly publish
    #[serde(rename = "useCanary")]
    pub use_canary: bool,
    /// Minecraft versions available
    pub versions: Vec<PostDeployGameRequestVersionsItem>,
}