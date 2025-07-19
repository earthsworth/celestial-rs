// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Minecraft player who uploaded the screenshot
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetGameScreenshotViewResponseUploader {
    /// Hex color of the uploader's rank
    #[serde(rename = "logoColor")]
    pub logo_color: String,
    /// Hex color of the uploader's Lunar+ plus
    #[serde(rename = "lunarPlusColor")]
    pub lunar_plus_color: String,
    /// Minecraft username of the uploader
    pub username: String,
    /// Minecraft UUID of the uploader
    pub uuid: String,
}