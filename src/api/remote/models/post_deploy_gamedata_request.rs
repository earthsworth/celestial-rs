// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_deploy_gamedata_request_gamedata_item::PostDeployGamedataRequestGamedataItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployGamedataRequest {
    /// Game Data for this publish
    pub gamedata: Vec<PostDeployGamedataRequestGamedataItem>,
}