// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_launch_data_arguments::PostLauncherLaunchVanillaResponseLaunchDataArguments;
use super::post_launcher_launch_vanilla_response_launch_data_asset_index::PostLauncherLaunchVanillaResponseLaunchDataAssetIndex;
use super::post_launcher_launch_vanilla_response_launch_data_downloads::PostLauncherLaunchVanillaResponseLaunchDataDownloads;
use super::post_launcher_launch_vanilla_response_launch_data_java_version::PostLauncherLaunchVanillaResponseLaunchDataJavaVersion;
use super::post_launcher_launch_vanilla_response_launch_data_libraries_item::PostLauncherLaunchVanillaResponseLaunchDataLibrariesItem;
use super::post_launcher_launch_vanilla_response_launch_data_logging::PostLauncherLaunchVanillaResponseLaunchDataLogging;
use super::post_launcher_launch_vanilla_response_launch_data_processors_item::PostLauncherLaunchVanillaResponseLaunchDataProcessorsItem;

/// Combined launch data used by launcher version > 
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseLaunchData {
    /// Arguments to be passed to the game and JVM
    pub arguments: PostLauncherLaunchVanillaResponseLaunchDataArguments,
    /// Downloadable file information
    #[serde(rename = "assetIndex")]
    pub asset_index: PostLauncherLaunchVanillaResponseLaunchDataAssetIndex,
    /// Asset index id of the release
    pub assets: String,
    /// Additional data variables for forge based loaders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Version jars and mappings
    pub downloads: PostLauncherLaunchVanillaResponseLaunchDataDownloads,
    /// The version id
    pub id: String,
    /// Java version information
    #[serde(rename = "javaVersion")]
    pub java_version: PostLauncherLaunchVanillaResponseLaunchDataJavaVersion,
    /// List of libraries to be downloaded for the release
    pub libraries: Vec<PostLauncherLaunchVanillaResponseLaunchDataLibrariesItem>,
    /// Logging information for the release
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<PostLauncherLaunchVanillaResponseLaunchDataLogging>,
    /// Main class of the game to launch
    #[serde(rename = "mainClass")]
    pub main_class: String,
    /// Game arguments (legacy)
    #[serde(rename = "minecraftArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minecraft_arguments: Option<String>,
    /// List of processors for forge based loaders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<PostLauncherLaunchVanillaResponseLaunchDataProcessorsItem>>,
    /// Type of the release
    #[serde(rename = "type")]
    pub r#type: String,
    /// Time of the release
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}