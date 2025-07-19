// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct LauncherBlogPost {
    /// Type of badge to display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<String>,
    /// Hash associated with the contents of the blog post (for read/unread indicators)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// Image to display with the blog post
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// URL that is opened upon clicking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// Title of the blog post (primarily for Analytics)
    pub title: String,
}