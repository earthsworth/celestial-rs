// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_webstore_collections_response_collections_item::GetWebstoreCollectionsResponseCollectionsItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstoreCollectionsResponse {
    /// Collections
    pub collections: Vec<GetWebstoreCollectionsResponseCollectionsItem>,
}