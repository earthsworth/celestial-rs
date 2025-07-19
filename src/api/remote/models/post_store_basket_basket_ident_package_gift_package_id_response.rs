// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::store_basket::StoreBasket;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostStoreBasketBasketIdentPackageGiftPackageIdResponse {
    pub basket: StoreBasket,
    /// Whether the request was successful
    pub success: bool,
}