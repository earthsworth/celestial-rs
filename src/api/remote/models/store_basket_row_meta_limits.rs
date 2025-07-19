// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::store_basket_row_meta_limits_global::StoreBasketRowMetaLimitsGlobal;
use super::store_basket_row_meta_limits_user::StoreBasketRowMetaLimitsUser;

/// Purchase limits and restrictions for this item
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketRowMetaLimits {
    /// Global purchase limits
    pub global: StoreBasketRowMetaLimitsGlobal,
    /// Timestamp when the package expires
    #[serde(rename = "packageExpiryTime")]
    pub package_expiry_time: f64,
    /// User-specific purchase limits
    pub user: StoreBasketRowMetaLimitsUser,
}