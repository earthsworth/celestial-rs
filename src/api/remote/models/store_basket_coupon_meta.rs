// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Coupon metadata including savings
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketCouponMeta {
    /// Total savings amount from this coupon
    pub savings: f64,
}