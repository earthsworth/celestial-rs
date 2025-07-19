// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetStreamelementsCallbackResponse {
    /// A message describing the result of the operation
    pub message: String,
    /// Indicates if the operation was successful
    pub success: bool,
}