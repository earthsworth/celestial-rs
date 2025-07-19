// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_webstore_partners_response_partners_item::GetWebstorePartnersResponsePartnersItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstorePartnersResponse {
    /// Partners
    pub partners: Vec<GetWebstorePartnersResponsePartnersItem>,
}