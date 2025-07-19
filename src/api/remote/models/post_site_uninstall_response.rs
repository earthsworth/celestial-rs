// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostSiteUninstallResponse {
    /// Reason for success or failure
    pub reason: String,
    /// If this request was successful
    pub success: bool,
}