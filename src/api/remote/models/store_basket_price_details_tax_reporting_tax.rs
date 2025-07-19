// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Tax information in reporting currency
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketPriceDetailsTaxReportingTax {
    /// Base currency code for the transaction
    #[serde(rename = "baseCurrency")]
    pub base_currency: String,
    /// Exchange rate between base and reporting currencies
    #[serde(rename = "exchangeRate")]
    pub exchange_rate: f64,
    /// Source or provider of the exchange rate
    #[serde(rename = "exchangeRateSource")]
    pub exchange_rate_source: String,
    /// Currency code used for reporting purposes
    #[serde(rename = "reportingCurrency")]
    pub reporting_currency: String,
    /// Tax amount in the reporting currency
    #[serde(rename = "taxAmount")]
    pub tax_amount: f64,
    /// Taxable amount in the reporting currency
    #[serde(rename = "taxableAmount")]
    pub taxable_amount: f64,
}