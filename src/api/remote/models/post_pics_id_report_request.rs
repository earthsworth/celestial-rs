// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostPicsIdReportRequest {
    /// Detailed reason for the report
    pub reason: String,
    /// Type of report being submitted
    pub report_type: String,
    /// Cloudflare Turnstile token for spam protection
    #[serde(rename = "Turnstile-Token")]
    pub turnstile_token: String,
}

