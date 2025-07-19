// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_profile::LauncherProfile;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchVanillaRequest {
    /// CPU Architecture of the requester
    pub arch: String,
    /// HWID of the requester
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid: Option<String>,
    /// HWID Private of the requester
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid_private: Option<String>,
    /// Installation ID of the requester
    pub installation_id: String,
    /// The type of launch, always set to OFFLINE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// Current launcher version of the requester
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launcher_version: Option<String>,
    /// The fabric/forge loader version to launch with
    #[serde(rename = "loaderVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loader_version: Option<String>,
    /// The loaders to launch with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loaders: Option<Vec<String>>,
    /// The module attempting to be launched with the current version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// Operating system of the requester
    pub os: String,
    /// Operating system release of the requester
    pub os_release: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<LauncherProfile>,
    /// Whether to use combined launch data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_combined_data: Option<bool>,
    /// The full Minecraft version to launch
    pub version: String,
}