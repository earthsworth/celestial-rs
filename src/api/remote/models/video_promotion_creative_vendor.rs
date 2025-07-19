// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Vendor information associated with the creative
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct VideoPromotionCreativeVendor {
    /// Logo of the vendor
    pub image: String,
    /// Name of the vendor that is providing the creative (company)
    pub name: String,
}