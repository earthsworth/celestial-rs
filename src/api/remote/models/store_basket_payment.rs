// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Payment processing information
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StoreBasketPayment {
    /// Total payment amount
    pub amount: f64,
    /// Whether the payment method can be saved for future use
    #[serde(rename = "canPaymentMethodBeSaved")]
    pub can_payment_method_be_saved: bool,
    /// Currency used during checkout process
    #[serde(rename = "checkoutCurrency")]
    pub checkout_currency: String,
    /// URL to redirect to after payment completion
    #[serde(rename = "completeUrl")]
    pub complete_url: String,
    /// Country where the payment was made
    pub country: String,
    /// Currency code for the payment
    pub currency: String,
    /// Date and time of the payment
    pub date: String,
    /// URL for Discord login integration (null if not applicable)
    #[serde(rename = "discordLoginUrl")]
    pub discord_login_url: String,
    /// Email address associated with the payment
    pub email: String,
    /// Name of the person making the payment
    pub name: String,
    /// Whether the payment was made by a verified customer
    #[serde(rename = "paidByVerifiedCustomer")]
    pub paid_by_verified_customer: bool,
    /// Name or type of the payment method used
    #[serde(rename = "paymentMethodName")]
    pub payment_method_name: String,
    /// Step in the save payment method process (null if not applicable)
    #[serde(rename = "savePaymentMethodStep")]
    pub save_payment_method_step: String,
    /// Current status of the payment
    pub status: String,
    /// Unique transaction identifier
    #[serde(rename = "txnId")]
    pub txn_id: String,
}