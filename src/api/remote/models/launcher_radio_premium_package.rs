// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherRadioPremiumPackage {
    /// Name of the premium package
    pub name: String,
    /// The Tebex ID of the package
    #[serde(rename = "packageId")]
    pub package_id: f64,
    /// Amount of the price of the premium radio subscription
    #[serde(rename = "priceAmount")]
    pub price_amount: f64,
    /// Currency of the price of the premium radio subscription
    #[serde(rename = "priceCurrency")]
    pub price_currency: String,
}