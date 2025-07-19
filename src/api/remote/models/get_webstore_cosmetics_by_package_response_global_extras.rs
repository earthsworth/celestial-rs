// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_webstore_cosmetics_by_package_response_global_extras_molang::GetWebstoreCosmeticsByPackageResponseGlobalExtrasMolang;

/// The global extras (molang) of the package.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstoreCosmeticsByPackageResponseGlobalExtras {
    /// The Molang data for the cosmetic
    pub molang: GetWebstoreCosmeticsByPackageResponseGlobalExtrasMolang,
}