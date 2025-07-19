// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Customer's billing and shipping address
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketAddress {
    /// Street address or location
    pub address: String,
    /// Country name or code
    pub country: String,
    /// Customer's email address
    pub email: String,
    /// Customer's first name
    pub first_name: String,
    /// Customer's last name
    pub last_name: String,
    /// Full name of the customer
    pub name: String,
    /// Postal or ZIP code
    pub postal_code: String,
    /// State or province identifier (null if not applicable)
    pub state_id: String,
}