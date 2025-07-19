// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Call to action information associated with the creative
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct VideoPromotionCreativeCta {
    /// Text that is shown on the button
    #[serde(rename = "buttonText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    /// URL that is opened when the button is clicked
    #[serde(rename = "buttonUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_url: Option<String>,
    /// Hook text that is shown on the left of the button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Display hint of the URL that is opened positioned on the left of the button
    #[serde(rename = "urlDisplay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_display: Option<String>,
}