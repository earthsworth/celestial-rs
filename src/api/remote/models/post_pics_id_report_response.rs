// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct PostPicsIdReportResponse {
    /// Unique identifier for the submitted report
    pub report_id: String,
    /// Whether the report was successfully submitted
    pub success: bool,
}