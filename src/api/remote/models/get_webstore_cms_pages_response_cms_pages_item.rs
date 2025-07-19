// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// CMS Page
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstoreCmsPagesResponseCmsPagesItem {
    /// Page body
    pub body: String,
    /// Page slug
    pub slug: String,
    /// Page title
    pub title: String,
}