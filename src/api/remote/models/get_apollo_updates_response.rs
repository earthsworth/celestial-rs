// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_apollo_updates_response_assets::GetApolloUpdatesResponseAssets;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetApolloUpdatesResponse {
    pub assets: GetApolloUpdatesResponseAssets,
    /// The latest version of Apollo for the given artifact
    pub version: String,
}