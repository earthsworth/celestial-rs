// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherModerationNsfwResponse {
    /// Whether the image is allowed or not
    pub allowed: bool,
    /// Why the image was disallowed
    pub reason: String,
}