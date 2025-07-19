// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Compliance URLs of the server
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct ServerMappingsCompliance {
    /// URL to the server's Privacy Policy
    #[serde(rename = "privacyPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<String>,
    /// URL to the server's Rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    /// URL to the server's Support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<String>,
    /// URL to the server's Terms of Service
    #[serde(rename = "termsOfService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
}