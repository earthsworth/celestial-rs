// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_forge_launch_data_arguments::PostLauncherLaunchVanillaResponseForgeLaunchDataArguments;
use super::post_launcher_launch_vanilla_response_forge_launch_data_libraries_item::PostLauncherLaunchVanillaResponseForgeLaunchDataLibrariesItem;
use super::post_launcher_launch_vanilla_response_forge_launch_data_processors_item::PostLauncherLaunchVanillaResponseForgeLaunchDataProcessorsItem;

/// Forge launch data provided by forge-installer
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseForgeLaunchData {
    /// Arguments to be passed to the game and JVM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<PostLauncherLaunchVanillaResponseForgeLaunchDataArguments>,
    /// Additional data variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Forge loader version ID
    pub id: String,
    /// Minecraft version that this loader inherits from
    #[serde(rename = "inheritsFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherits_from: Option<String>,
    /// List of libraries to be downloaded
    pub libraries: Vec<PostLauncherLaunchVanillaResponseForgeLaunchDataLibrariesItem>,
    /// Main class that gets executed
    #[serde(rename = "mainClass")]
    pub main_class: String,
    /// Game arguments (legacy)
    #[serde(rename = "minecraftArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minecraft_arguments: Option<String>,
    /// List of processors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<PostLauncherLaunchVanillaResponseForgeLaunchDataProcessorsItem>>,
    /// Release type
    #[serde(rename = "type")]
    pub r#type: String,
    /// Release time of this version of forge loader
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    /// Time of this version of forge loader
    pub time: String,
}