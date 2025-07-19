// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostDeployLicensesResponse {
    /// Explanation for success or failure.
    pub message: String,
    /// If this deploy was successful
    pub success: bool,
}