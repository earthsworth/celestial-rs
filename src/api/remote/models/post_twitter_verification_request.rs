// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostTwitterVerificationRequest {
    /// Twitter OAuth token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
    /// Twitter OAuth verifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_verifier: Option<String>,
}