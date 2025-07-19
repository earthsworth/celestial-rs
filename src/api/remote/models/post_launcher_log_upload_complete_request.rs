// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLogUploadCompleteRequest {
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
    /// Current launcher version of the requester
    pub launcher_version: String,
    /// Operating system of the requester
    pub os: String,
    /// Operating system release of the requester
    pub os_release: String,
    /// Upload ID which can be provided to LC staff to debug
    #[serde(rename = "uploadId")]
    pub upload_id: String,
}