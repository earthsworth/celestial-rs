// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PatchPicsPrivacyRequest {
    /// Screenshot ID to update privacy for
    pub id: String,
    /// Privacy setting for the screenshot
    pub privacy: String,
}