// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::launcher_profile::LauncherProfile;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchRequest {
    /// CPU Architecture of the requester
    pub arch: String,
    /// Flags attempting to be used in the launch sequence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// The branch that is attempting to be launched, will reset to master if the user does not have access
    pub branch: String,
    /// The canary preference of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_preference: Option<String>,
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
    /// The module attempting to be launched with the current version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// Operating system of the requester
    pub os: String,
    /// Operating system release of the requester
    pub os_release: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<LauncherProfile>,
    /// The full Minecraft version to launch
    pub version: String,
}