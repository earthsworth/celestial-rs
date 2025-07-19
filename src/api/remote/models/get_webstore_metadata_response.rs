// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_webstore_metadata_response_alert::GetWebstoreMetadataResponseAlert;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstoreMetadataResponse {
    pub alert: GetWebstoreMetadataResponseAlert,
}