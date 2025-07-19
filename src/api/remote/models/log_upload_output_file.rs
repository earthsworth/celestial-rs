// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// File that should be uploaded
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LogUploadOutputFile {
    /// Name of the file on the player's machine
    pub name: String,
    /// URL the client should use to PUT the file's contents
    #[serde(rename = "uploadUrl")]
    pub upload_url: String,
}