// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_site_packages_response_global_extras::GetSitePackagesResponseGlobalExtras;
use super::get_site_packages_response_particles::GetSitePackagesResponseParticles;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetSitePackagesResponse {
    /// The global extras (molang) of the package.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_extras: Option<GetSitePackagesResponseGlobalExtras>,
    /// The items in the package.
    pub items: Vec<serde_json::Value>,
    /// The particles of the package.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particles: Option<GetSitePackagesResponseParticles>,
}