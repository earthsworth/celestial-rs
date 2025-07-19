// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_game_screenshot_view_response_uploader::GetGameScreenshotViewResponseUploader;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetGameScreenshotViewResponse {
    /// Image URL to display
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    /// Privacy setting of the screenshot
    pub privacy: String,
    /// Screenshot title
    pub title: String,
    /// Time this screenshot was uploaded
    #[serde(rename = "uploadedAt")]
    pub uploaded_at: String,
    /// Minecraft player who uploaded the screenshot
    pub uploader: GetGameScreenshotViewResponseUploader,
}