// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Detailed coupon information
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketCouponRelatedCoupon {
    /// Account ID associated with the coupon
    pub account: f64,
    /// Whether the coupon was auto-generated
    pub auto_generated: f64,
    /// Basket type restriction (e.g., 'both', 'single', 'subscription')
    pub basket_type: String,
    /// Coupon code
    pub code: String,
    /// Fixed discount amount
    pub discount_amount: f64,
    /// Method for applying the discount
    pub discount_application_method: f64,
    /// Discount percentage amount
    pub discount_percentage: f64,
    /// Type of discount (e.g., 'percentage', 'amount')
    pub discount_type: String,
    /// Expiration limit (null if not applicable)
    pub expire_limit: f64,
    /// Expiration timestamp (null if not applicable)
    pub expire_timestamp: f64,
    /// Expiration type (e.g., 'limit', 'timestamp')
    pub expire_type: String,
    /// Unique identifier for the coupon
    pub id: f64,
    /// Minimum purchase amount required
    pub minimum: f64,
    /// Additional notes about the coupon
    pub note: String,
    /// Type of coupon (e.g., 'cart')
    #[serde(rename = "type")]
    pub r#type: String,
    /// Coupon start time timestamp
    pub start_time: f64,
    /// User-specific usage limit
    pub user_limit: f64,
    /// Specific username ID restriction (null if not applicable)
    pub username_id: f64,
}