// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_deploy_licenses_request_licenses_item::PostDeployLicensesRequestLicensesItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployLicensesRequest {
    /// Licenses for this publish
    pub licenses: Vec<PostDeployLicensesRequestLicensesItem>,
}