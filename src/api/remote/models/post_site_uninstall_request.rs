// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostSiteUninstallRequest {
    /// User's response to the other question
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_feedback: Option<String>,
    /// User's response to the comeback question
    pub comeback: String,
    /// User's response to the easy to use question
    pub easy_to_use: String,
    /// User's installation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_id: Option<String>,
    /// User's response to the performance question
    pub performance: String,
    /// User's response to the reason question
    pub reason: String,
}