// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Pagination metadata
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetPicsListResponsePagination {
    /// Current page number
    #[serde(rename = "currentPage")]
    pub current_page: f64,
    /// Number of items per page
    #[serde(rename = "pageSize")]
    pub page_size: f64,
    /// Total number of items
    #[serde(rename = "totalItems")]
    pub total_items: f64,
    /// Total number of pages
    #[serde(rename = "totalPages")]
    pub total_pages: f64,
}