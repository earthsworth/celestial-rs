// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::store_basket_gift_card::StoreBasketGiftCard;
use super::store_basket_price_details_sales_item::StoreBasketPriceDetailsSalesItem;
use super::store_basket_price_details_tax::StoreBasketPriceDetailsTax;

/// Detailed breakdown of pricing and costs
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketPriceDetails {
    /// Outstanding balance or amount due
    pub balance: f64,
    /// ISO currency code for the transaction
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// Currency symbol for display purposes
    #[serde(rename = "currencySymbol")]
    pub currency_symbol: String,
    /// Array of discount objects applied to the basket
    pub discounts: Vec<serde_json::Value>,
    /// Full price before any discounts or modifications
    #[serde(rename = "fullPrice")]
    pub full_price: f64,
    /// Array of gift card objects used in payment
    pub giftcards: Vec<StoreBasketGiftCard>,
    /// Recurring payment information (null if not applicable)
    pub recurring: serde_json::Value,
    /// Round up charity donation information (null if not applicable)
    #[serde(rename = "roundUp")]
    pub round_up: serde_json::Value,
    /// Array of sales or promotional offers applied
    pub sales: Vec<StoreBasketPriceDetailsSalesItem>,
    /// Subtotal after discounts but before tax
    #[serde(rename = "subTotal")]
    pub sub_total: f64,
    /// Array of additional charges or fees
    pub surcharges: Vec<serde_json::Value>,
    /// Comprehensive tax information for the transaction
    pub tax: StoreBasketPriceDetailsTax,
    /// Final total amount including all taxes and fees
    pub total: f64,
}