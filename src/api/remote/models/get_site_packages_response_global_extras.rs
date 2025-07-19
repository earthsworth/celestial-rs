// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_site_packages_response_global_extras_molang::GetSitePackagesResponseGlobalExtrasMolang;

/// The global extras (molang) of the package.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetSitePackagesResponseGlobalExtras {
    /// The Molang data for the cosmetic
    pub molang: GetSitePackagesResponseGlobalExtrasMolang,
}