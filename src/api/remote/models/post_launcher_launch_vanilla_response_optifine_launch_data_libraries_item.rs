// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_optifine_launch_data_libraries_item_downloads::PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloads;
use super::post_launcher_launch_vanilla_response_optifine_launch_data_libraries_item_extract::PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemExtract;
use super::post_launcher_launch_vanilla_response_optifine_launch_data_libraries_item_rules_item::PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemRulesItem;

/// Library to be downloaded
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItem {
    /// Downloadable artifact for the library
    pub downloads: PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemDownloads,
    /// Extraction information for the library
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract: Option<PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemExtract>,
    /// Whether the library should be included in the classpath
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_in_classpath: Option<bool>,
    /// The maven name of the library
    pub name: String,
    /// Information about native libraries bundled with this library. Appears only when there are classifiers for natives
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives: Option<serde_json::Value>,
    /// JVM rules for the library
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItemRulesItem>>,
}