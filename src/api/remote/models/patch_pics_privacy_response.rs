// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PatchPicsPrivacyResponse {
    /// The new privacy setting of the screenshot
    pub privacy: String,
    /// Whether the privacy update was successful
    pub success: bool,
}