// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_optifine_launch_data_arguments::PostLauncherLaunchVanillaResponseOptifineLaunchDataArguments;
use super::post_launcher_launch_vanilla_response_optifine_launch_data_libraries_item::PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItem;

/// Optifine launch data provided by the optifine installer
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseOptifineLaunchData {
    /// Arguments to be passed to the game and JVM
    pub arguments: PostLauncherLaunchVanillaResponseOptifineLaunchDataArguments,
    /// Optifine version ID
    pub id: String,
    /// List of libraries to be downloaded
    pub libraries: Vec<PostLauncherLaunchVanillaResponseOptifineLaunchDataLibrariesItem>,
    /// Main class that gets executed
    #[serde(rename = "mainClass")]
    pub main_class: String,
    /// Release time of this Optifine version
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    /// Time of this Optifine version
    pub time: String,
}