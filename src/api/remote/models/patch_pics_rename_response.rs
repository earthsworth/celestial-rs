// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PatchPicsRenameResponse {
    /// Whether the rename was successful
    pub success: bool,
    /// The new title of the screenshot
    pub title: String,
}