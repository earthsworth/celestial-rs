// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::post_store_basket_create_request_utm_params::PostStoreBasketCreateRequestUtmParams;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostStoreBasketCreateRequest {
    /// A basket identifier to copy from, used for authenticating existing baskets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_basket: Option<String>,
    /// IP address of the user creating the basket
    pub ip: String,
    /// Username of the user creating the basket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// UTM parameters for tracking purposes
    pub utm_params: PostStoreBasketCreateRequestUtmParams,
}