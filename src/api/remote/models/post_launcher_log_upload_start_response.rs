// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::log_upload_output_file::LogUploadOutputFile;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostLauncherLogUploadStartResponse {
    /// Files the client should upload, with a URL for each
    pub files: Vec<LogUploadOutputFile>,
    /// Upload ID which can be provided to LC staff to debug
    #[serde(rename = "uploadId")]
    pub upload_id: String,
}