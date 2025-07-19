// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Social media channels of the server
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct ServerMappingsSocials {
    /// Full discord invite link of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    /// Facebook slug of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook: Option<String>,
    /// Instagram handle of the server (without @)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram: Option<String>,
    /// Reddit slug of the server (without the leading r/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reddit: Option<String>,
    /// Telegram channel slug of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram: Option<String>,
    /// TikTok handle of the server (without @)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiktok: Option<String>,
    /// Twitch channel of the server (without @)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitch: Option<String>,
    /// Twitter handle of the server (without @)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    /// Youtube channel slug of the server (without /channel/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube: Option<String>,
}