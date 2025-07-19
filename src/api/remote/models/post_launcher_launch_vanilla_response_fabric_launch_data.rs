// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_fabric_launch_data_arguments::PostLauncherLaunchVanillaResponseFabricLaunchDataArguments;
use super::post_launcher_launch_vanilla_response_fabric_launch_data_libraries_item::PostLauncherLaunchVanillaResponseFabricLaunchDataLibrariesItem;

/// Fabric launch data provided by fabric meta
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseFabricLaunchData {
    /// Arguments to be passed to the game and JVM
    pub arguments: PostLauncherLaunchVanillaResponseFabricLaunchDataArguments,
    /// Fabric loader version ID
    pub id: String,
    /// Minecraft version that this loader inherits from
    #[serde(rename = "inheritsFrom")]
    pub inherits_from: String,
    /// Libraries to be downloaded
    pub libraries: Vec<PostLauncherLaunchVanillaResponseFabricLaunchDataLibrariesItem>,
    /// Main class that gets executed
    #[serde(rename = "mainClass")]
    pub main_class: String,
    /// Release type
    #[serde(rename = "type")]
    pub r#type: String,
    /// Release time of this version of fabric loader
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}