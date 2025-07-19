// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_forge_launch_data_libraries_item_downloads::PostLauncherLaunchVanillaResponseForgeLaunchDataLibrariesItemDownloads;

/// Forge library
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseForgeLaunchDataLibrariesItem {
    pub downloads: PostLauncherLaunchVanillaResponseForgeLaunchDataLibrariesItemDownloads,
    /// Whether to include this library in the classpath
    pub include_in_classpath: bool,
    /// Library name
    pub name: String,
}