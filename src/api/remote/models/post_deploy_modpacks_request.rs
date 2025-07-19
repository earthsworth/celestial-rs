// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_deploy_modpacks_request_modpacks_item::PostDeployModpacksRequestModpacksItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployModpacksRequest {
    /// Modpacks for this publish
    pub modpacks: Vec<PostDeployModpacksRequestModpacksItem>,
}