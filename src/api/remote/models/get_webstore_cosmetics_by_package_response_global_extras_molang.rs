// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// The Molang data for the cosmetic
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetWebstoreCosmeticsByPackageResponseGlobalExtrasMolang {
    /// The Molang constants, for the cosmetic
    pub constants: String,
    /// The Molang functions, for the cosmetic
    pub functions: String,
}