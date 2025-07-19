// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetGameMetadataResponseBlogPostsItem {
    /// Image to display with the blog post
    pub image: String,
    /// URL that is opened upon clicking
    pub link: String,
    /// Title of the blog post (primarily for Analytics)
    pub title: String,
}