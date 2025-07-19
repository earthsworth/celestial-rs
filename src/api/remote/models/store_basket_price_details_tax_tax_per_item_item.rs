// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketPriceDetailsTaxTaxPerItemItem {
    /// Unique identifier for the basket item
    pub basket_item_id: f64,
    /// Base price of the item before tax
    pub price: f64,
    /// Tax amount applied to this specific item
    pub tax: f64,
    /// Total price including tax for this item
    pub total: f64,
}