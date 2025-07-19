// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Screenshot details
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetPicsListResponseScreenshotsItem {
    /// Screenshot ID
    pub id: String,
    /// Image URL to display
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    /// Screenshot title
    pub title: String,
    /// Time this screenshot was uploaded
    #[serde(rename = "uploadedAt")]
    pub uploaded_at: String,
    /// Minecraft username of the uploader
    #[serde(rename = "uploaderName")]
    pub uploader_name: String,
}