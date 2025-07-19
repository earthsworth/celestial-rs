// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketPriceDetailsSalesItem {
    /// Discount amount applied by this sale
    #[serde(rename = "discountAmount")]
    pub discount_amount: f64,
    /// Unique identifier for the sale
    pub id: f64,
    /// Name or title of the sale
    pub name: String,
    /// Package ID associated with this sale
    pub package_id: f64,
    /// Type identifier for sale entries
    #[serde(rename = "type")]
    pub r#type: String,
}