// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::get_pics_list_response_pagination::GetPicsListResponsePagination;
use super::get_pics_list_response_screenshots_item::GetPicsListResponseScreenshotsItem;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetPicsListResponse {
    /// Pagination metadata
    pub pagination: GetPicsListResponsePagination,
    /// List of screenshots
    pub screenshots: Vec<GetPicsListResponseScreenshotsItem>,
}