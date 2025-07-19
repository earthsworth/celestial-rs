// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::store_basket_coupon_meta::StoreBasketCouponMeta;
use super::store_basket_coupon_related_coupon::StoreBasketCouponRelatedCoupon;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketCoupon {
    /// Basket ID this coupon is applied to
    pub basket: f64,
    /// Coupon ID
    pub coupon: f64,
    /// Unique identifier for the basket coupon
    pub id: f64,
    /// Coupon metadata including savings
    pub meta: StoreBasketCouponMeta,
    /// Detailed coupon information
    pub related_coupon: StoreBasketCouponRelatedCoupon,
}