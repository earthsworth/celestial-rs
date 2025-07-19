// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetLauncherMetadataMiscResponseTermsItem {
    /// SHA1 hash of the terms page
    pub hash: String,
    /// ID of the terms page
    pub id: String,
    /// Last updated date of the terms page
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
    /// Name of the terms page, in the event that the ID does not have a key in the lang
    pub name: String,
    /// URL of the terms page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}