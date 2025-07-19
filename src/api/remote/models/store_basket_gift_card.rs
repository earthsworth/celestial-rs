// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::store_basket_gift_card_last_consideration::StoreBasketGiftCardLastConsideration;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketGiftCard {
    /// Basket ID this gift card is applied to
    pub basket: f64,
    /// Amount deducted from the gift card
    pub deduction: String,
    /// Gift card identifier
    pub giftcard: f64,
    /// Gift card ID
    pub giftcard_id: f64,
    /// Unique identifier for the gift card
    pub id: f64,
    /// Last consideration details
    pub last_consideration: StoreBasketGiftCardLastConsideration,
}