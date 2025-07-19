// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployLicensesRequestLicensesItem {
    /// Name of this license
    pub file: String,
    /// Last modification time of this license
    pub mtime: i64,
    /// SHA1 hash of this license
    pub sha1: String,
    /// Size, in bytes, of this license
    pub size: i64,
    /// URL to download this license
    pub url: String,
}