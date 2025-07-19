// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_webstore_cms_pages_response_cms_pages_item::GetWebstoreCmsPagesResponseCmsPagesItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstoreCmsPagesResponse {
    /// CMS pages
    #[serde(rename = "cmsPages")]
    pub cms_pages: Vec<GetWebstoreCmsPagesResponseCmsPagesItem>,
}