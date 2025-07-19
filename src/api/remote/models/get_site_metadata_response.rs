// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_site_metadata_response_alert::GetSiteMetadataResponseAlert;
use super::online_players::OnlinePlayers;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetSiteMetadataResponse {
    pub alert: GetSiteMetadataResponseAlert,
    pub statistics: OnlinePlayers,
}