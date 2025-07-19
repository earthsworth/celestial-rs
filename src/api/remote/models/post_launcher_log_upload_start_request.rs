// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::log_upload_input_file::LogUploadInputFile;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLogUploadStartRequest {
    /// CPU Architecture of the requester
    pub arch: String,
    /// Canary token given to the user at launch time
    #[serde(rename = "canaryToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_token: Option<String>,
    /// Files that the client would like to upload
    pub files: Vec<LogUploadInputFile>,
    /// HWID of the requester
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid: Option<String>,
    /// HWID Private of the requester
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid_private: Option<String>,
    /// Initiator of the log upload
    pub initiator: String,
    /// Installation ID of the requester
    pub installation_id: String,
    /// Current launcher version of the requester
    pub launcher_version: String,
    /// Operating system of the requester
    pub os: String,
    /// Operating system release of the requester
    pub os_release: String,
    /// Player who created these logs, unauthenticated users are not able to upload logs
    #[serde(rename = "playerUuid")]
    pub player_uuid: String,
    /// Sentry crash ID
    #[serde(rename = "sentryCrashId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentry_crash_id: Option<String>,
    /// Matched error solution ID
    #[serde(rename = "solutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_id: Option<String>,
}