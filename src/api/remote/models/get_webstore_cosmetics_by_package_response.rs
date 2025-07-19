// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_webstore_cosmetics_by_package_response_global_extras::GetWebstoreCosmeticsByPackageResponseGlobalExtras;
use super::get_webstore_cosmetics_by_package_response_particles::GetWebstoreCosmeticsByPackageResponseParticles;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstoreCosmeticsByPackageResponse {
    /// The global extras (molang) of the package.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_extras: Option<GetWebstoreCosmeticsByPackageResponseGlobalExtras>,
    /// The items in the package.
    pub items: Vec<serde_json::Value>,
    /// The particles of the package.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particles: Option<GetWebstoreCosmeticsByPackageResponseParticles>,
}