// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::store_basket_address::StoreBasketAddress;
use super::store_basket_coupon::StoreBasketCoupon;
use super::store_basket_gift_card::StoreBasketGiftCard;
use super::store_basket_links::StoreBasketLinks;
use super::store_basket_payment::StoreBasketPayment;
use super::store_basket_price_details::StoreBasketPriceDetails;
use super::store_basket_row::StoreBasketRow;

/// The basket after the operation
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasket {
    /// Available actions for this basket (null if none)
    pub actions: serde_json::Value,
    /// Customer's billing and shipping address
    pub address: StoreBasketAddress,
    /// URL to redirect to if checkout is cancelled
    pub cancel_url: String,
    /// Whether the basket checkout has been completed
    pub complete: bool,
    /// Whether to automatically redirect after completion
    pub complete_auto_redirect: bool,
    /// URL to redirect to after successful checkout
    pub complete_url: String,
    /// Array of coupon objects applied to the basket
    pub coupons: Vec<StoreBasketCoupon>,
    /// Creator or referral code used
    pub creator_code: String,
    /// Custom data associated with this basket
    pub custom: serde_json::Value,
    /// Applied discounts (null if none)
    pub discounts: serde_json::Value,
    /// Whether the email address can be changed
    pub email_immutable: bool,
    /// Expiration date and time for this basket
    pub expire: String,
    /// Browser or device fingerprint for security
    pub fingerprint: String,
    /// In-game currency information (null if not applicable)
    pub game_currency: serde_json::Value,
    /// Array of gift card objects used in this basket
    pub giftcards: Vec<StoreBasketGiftCard>,
    /// Unique identifier for this basket
    pub ident: String,
    /// Whether this basket is for updating payment methods
    #[serde(rename = "isPaymentMethodUpdate")]
    pub is_payment_method_update: bool,
    /// Important URLs related to this basket
    pub links: StoreBasketLinks,
    /// Payment processing information
    pub payment: StoreBasketPayment,
    /// Total price of all items in the basket
    pub price: f64,
    /// Detailed breakdown of pricing and costs
    #[serde(rename = "priceDetails")]
    pub price_details: StoreBasketPriceDetails,
    /// Basket type classification (null if not applicable)
    #[serde(rename = "type")]
    pub r#type: serde_json::Value,
    /// Array of recurring subscription items
    pub recurring_items: Vec<serde_json::Value>,
    /// URL to return to after checkout (null if not specified)
    #[serde(rename = "returnUrl")]
    pub return_url: String,
    /// Charity round-up donation (null if not applicable)
    pub roundup: serde_json::Value,
    /// Array of items in the basket
    pub rows: Vec<StoreBasketRow>,
    /// Total tax amount for the basket
    pub tax: f64,
    /// Tebex account identifier (null if not linked)
    pub tebex_account_id: serde_json::Value,
    /// Username of the customer
    pub username: String,
    /// Verification status (null if not applicable)
    pub verified: serde_json::Value,
}