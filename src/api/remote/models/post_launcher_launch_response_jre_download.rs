// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Download information of the JRE
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLaunchResponseJreDownload {
    /// "Extension" of the JRE
    pub extension: String,
    /// Fallback URL location of the JRE, if the normal URL fails
    #[serde(rename = "fallbackUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_url: Option<String>,
    /// URL location of the JRE
    pub url: String,
}