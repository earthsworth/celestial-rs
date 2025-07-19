// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_launch_license::LauncherLaunchLicense;
use super::post_launcher_launch_response_base_modpack::PostLauncherLaunchResponseBaseModpack;
use super::post_launcher_launch_response_error::PostLauncherLaunchResponseError;
use super::post_launcher_launch_response_jre::PostLauncherLaunchResponseJre;
use super::post_launcher_launch_response_launch_type_data::PostLauncherLaunchResponseLaunchTypeData;
use super::post_launcher_launch_response_textures::PostLauncherLaunchResponseTextures;
use super::post_launcher_launch_response_ui::PostLauncherLaunchResponseUi;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchResponse {
    /// Launch Base Modpack
    #[serde(rename = "baseModpack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_modpack: Option<PostLauncherLaunchResponseBaseModpack>,
    /// Canary token to pass back in analytics
    #[serde(rename = "canaryToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_token: Option<String>,
    /// If the launch request was unsuccessful, information will be populated regarding it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<PostLauncherLaunchResponseError>,
    /// JRE information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jre: Option<PostLauncherLaunchResponseJre>,
    /// Information about the specific launch configuration
    #[serde(rename = "launchTypeData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type_data: Option<PostLauncherLaunchResponseLaunchTypeData>,
    /// Licenses that are downloaded with the specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<LauncherLaunchLicense>>,
    /// Whether the request to launch was successful or not
    pub success: bool,
    /// Textures location for specific launch configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textures: Option<PostLauncherLaunchResponseTextures>,
    /// WebOSR UI related information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<PostLauncherLaunchResponseUi>,
    /// Whether the assets should update
    #[serde(rename = "updateAssets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_assets: Option<bool>,
}