// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::store_basket_row_meta::StoreBasketRowMeta;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketRow {
    /// Basket ID this row belongs to
    pub basket: f64,
    /// Custom data associated with this item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<serde_json::Value>,
    /// User ID if this item is being gifted (null if not a gift)
    pub gift_username_id: f64,
    /// Unique identifier for this basket row
    pub id: f64,
    /// URL to the item's image
    pub image_url: String,
    /// Whether this item is part of a recurring subscription
    pub is_recurring: bool,
    /// Detailed metadata about this item
    pub meta: StoreBasketRowMeta,
    /// Configuration options for this item
    pub options: serde_json::Value,
    /// Package ID for this item
    pub package: f64,
    /// Final price for this row including quantity
    pub price: f64,
    /// Quantity of this item in the basket
    pub quantity: f64,
    /// Price override amount if applicable
    #[serde(rename = "override")]
    pub r#override: f64,
    /// Whether this item has recurring billing
    pub recurring: bool,
    /// Date of next recurring payment
    pub recurring_next_payment_date: String,
    /// Billing period for recurring items
    pub recurring_period: String,
    /// Price for recurring billing cycles
    pub recurring_price: f64,
    /// Specific server ID for this item (null if not server-specific)
    pub server: f64,
}