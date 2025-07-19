// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostGameScreenshotUploadRequest {
    /// Image contents, Base64-encoded
    #[serde(rename = "imageBase64")]
    pub image_base64: String,
    /// The locally generated ID of the screenshot
    #[serde(rename = "localId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,
    /// Privacy setting for the screenshot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    /// The title of the screenshot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}