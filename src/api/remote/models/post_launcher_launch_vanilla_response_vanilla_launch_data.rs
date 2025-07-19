// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_arguments::PostLauncherLaunchVanillaResponseVanillaLaunchDataArguments;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_asset_index::PostLauncherLaunchVanillaResponseVanillaLaunchDataAssetIndex;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_downloads::PostLauncherLaunchVanillaResponseVanillaLaunchDataDownloads;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_java_version::PostLauncherLaunchVanillaResponseVanillaLaunchDataJavaVersion;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_libraries_item::PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItem;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data_logging::PostLauncherLaunchVanillaResponseVanillaLaunchDataLogging;

/// Vanilla launch data provided by piston
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponseVanillaLaunchData {
    /// Arguments to be passed to the game and JVM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataArguments>,
    /// Downloadable file information
    #[serde(rename = "assetIndex")]
    pub asset_index: PostLauncherLaunchVanillaResponseVanillaLaunchDataAssetIndex,
    /// Asset index id of the release
    pub assets: String,
    /// Its value is 1 for all recent versions of the game (1.16.4 and above) or 0 for all others. This tag tells the launcher (mojang) whether it should urge the user to be careful since this version is older and might not support the latest player safety features.
    #[serde(rename = "complianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_level: Option<f64>,
    /// Downloadable files for the release
    pub downloads: PostLauncherLaunchVanillaResponseVanillaLaunchDataDownloads,
    /// Version id of the release
    pub id: String,
    /// Java version information
    #[serde(rename = "javaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_version: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataJavaVersion>,
    /// List of libraries to be downloaded for the release
    pub libraries: Vec<PostLauncherLaunchVanillaResponseVanillaLaunchDataLibrariesItem>,
    /// Logging information for the release
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<PostLauncherLaunchVanillaResponseVanillaLaunchDataLogging>,
    /// Main class of the game to launch
    #[serde(rename = "mainClass")]
    pub main_class: String,
    /// Game arguments (legacy)
    #[serde(rename = "minecraftArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minecraft_arguments: Option<String>,
    /// Minimum launcher (mojang) version required
    #[serde(rename = "minimumLauncherVersion")]
    pub minimum_launcher_version: f64,
    /// Type of the release
    #[serde(rename = "type")]
    pub r#type: String,
    /// Time of the release
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    /// Same as releaseTime
    pub time: String,
}