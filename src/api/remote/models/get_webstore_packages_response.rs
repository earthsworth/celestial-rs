// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_webstore_packages_response_packages_item::GetWebstorePackagesResponsePackagesItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstorePackagesResponse {
    /// Packages
    pub packages: Vec<GetWebstorePackagesResponsePackagesItem>,
}