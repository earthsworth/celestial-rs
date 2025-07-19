// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Images of the server
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherServerImages {
    /// URL to the server's background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    /// URL to the server's banner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    /// URL to the server's logo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// URL to the server's wordmark
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wordmark: Option<String>,
}