// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::store_basket_price_details_tax_reporting_tax::StoreBasketPriceDetailsTaxReportingTax;
use super::store_basket_price_details_tax_tax_per_item_item::StoreBasketPriceDetailsTaxTaxPerItemItem;

/// Comprehensive tax information for the transaction
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketPriceDetailsTax {
    /// Tax document or reference code
    #[serde(rename = "documentCode")]
    pub document_code: String,
    /// Name or type of the tax being applied
    pub name: String,
    /// Tax information in reporting currency
    #[serde(rename = "reportingTax")]
    pub reporting_tax: StoreBasketPriceDetailsTaxReportingTax,
    /// Total tax amount for the transaction
    #[serde(rename = "taxAmount")]
    pub tax_amount: f64,
    /// Mode or method of tax calculation
    #[serde(rename = "taxMode")]
    pub tax_mode: String,
    /// Individual tax breakdown per basket item
    #[serde(rename = "taxPerItem")]
    pub tax_per_item: Vec<StoreBasketPriceDetailsTaxTaxPerItemItem>,
    /// Total amount subject to taxation
    #[serde(rename = "taxableAmount")]
    pub taxable_amount: f64,
}