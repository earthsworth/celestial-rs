// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_game_metadata_response_alert_active::GetGameMetadataResponseAlertActive;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetGameMetadataResponseAlert {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<GetGameMetadataResponseAlertActive>,
    pub colors: serde_json::Value,
}