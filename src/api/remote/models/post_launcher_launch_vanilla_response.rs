// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_launcher_launch_vanilla_response_error::PostLauncherLaunchVanillaResponseError;
use super::post_launcher_launch_vanilla_response_fabric_launch_data::PostLauncherLaunchVanillaResponseFabricLaunchData;
use super::post_launcher_launch_vanilla_response_forge_launch_data::PostLauncherLaunchVanillaResponseForgeLaunchData;
use super::post_launcher_launch_vanilla_response_launch_data::PostLauncherLaunchVanillaResponseLaunchData;
use super::post_launcher_launch_vanilla_response_optifine_launch_data::PostLauncherLaunchVanillaResponseOptifineLaunchData;
use super::post_launcher_launch_vanilla_response_vanilla_launch_data::PostLauncherLaunchVanillaResponseVanillaLaunchData;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaResponse {
    /// If the launch request was unsuccessful, information will be populated regarding it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<PostLauncherLaunchVanillaResponseError>,
    /// Fabric launch data provided by fabric meta
    #[serde(rename = "fabricLaunchData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric_launch_data: Option<PostLauncherLaunchVanillaResponseFabricLaunchData>,
    /// Forge launch data provided by forge-installer
    #[serde(rename = "forgeLaunchData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forge_launch_data: Option<PostLauncherLaunchVanillaResponseForgeLaunchData>,
    /// Combined launch data used by launcher version > 
    #[serde(rename = "launchData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_data: Option<PostLauncherLaunchVanillaResponseLaunchData>,
    /// Optifine launch data provided by the optifine installer
    #[serde(rename = "optifineLaunchData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optifine_launch_data: Option<PostLauncherLaunchVanillaResponseOptifineLaunchData>,
    /// Whether the request to launch was successful or not
    pub success: bool,
    /// Vanilla launch data provided by piston
    #[serde(rename = "vanillaLaunchData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanilla_launch_data: Option<PostLauncherLaunchVanillaResponseVanillaLaunchData>,
}