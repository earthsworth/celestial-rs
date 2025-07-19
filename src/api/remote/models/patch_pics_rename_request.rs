// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PatchPicsRenameRequest {
    /// Screenshot ID to rename
    pub id: String,
    /// New title for the screenshot
    pub title: String,
}