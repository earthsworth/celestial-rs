// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::store_basket_row_meta_limits::StoreBasketRowMetaLimits;
use super::store_basket_row_meta_tax_per_item::StoreBasketRowMetaTaxPerItem;

/// Detailed metadata about this item
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketRowMeta {
    /// Whether gift cards can be used to purchase this item
    #[serde(rename = "allowsGiftCards")]
    pub allows_gift_cards: bool,
    /// Category ID this item belongs to
    pub category: f64,
    /// Types of deliverables included with this item
    #[serde(rename = "deliverableTypes")]
    pub deliverable_types: Vec<String>,
    /// Direct download link for deliverables
    #[serde(rename = "downloadLink")]
    pub download_link: String,
    /// Whether this item has digital deliverables
    #[serde(rename = "hasDeliverables")]
    pub has_deliverables: bool,
    /// Whether this item includes seller protection
    #[serde(rename = "hasSellerProtection")]
    pub has_seller_protection: bool,
    /// URL to the item's display image
    pub image: String,
    /// Original price before any modifications
    pub initialprice: f64,
    /// Whether this item's benefits stack with others
    #[serde(rename = "isCumulative")]
    pub is_cumulative: bool,
    /// Specific type classification of the item
    #[serde(rename = "itemType")]
    pub item_type: String,
    /// Purchase limits and restrictions for this item
    pub limits: StoreBasketRowMetaLimits,
    /// Display name of the package or item
    pub name: String,
    /// Whether this is a one-time purchase or subscription
    pub package_type: String,
    /// Whether purchasing this item generates a gift card
    #[serde(rename = "producesGiftCard")]
    pub produces_gift_card: bool,
    /// Actual calculated price after all modifications
    pub realprice: f64,
    /// Billing period for recurring items
    pub repeat_period: String,
    /// Array of package IDs required before purchasing this item
    #[serde(rename = "requiredPackages")]
    pub required_packages: Vec<f64>,
    /// Whether only one of the required packages is needed (true) or all (false)
    #[serde(rename = "requiresAny")]
    pub requires_any: bool,
    /// Revenue sharing configuration for this item
    pub revenue_share: Vec<f64>,
    /// Individual row price for this item
    pub rowprice: f64,
    /// Array of server IDs where this item is available
    pub servers: Vec<f64>,
    /// Tax breakdown specific to this item
    #[serde(rename = "taxPerItem")]
    pub tax_per_item: StoreBasketRowMetaTaxPerItem,
    /// Whether this item uses tiered pricing
    pub tiered: bool,
}