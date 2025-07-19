// Generated Code
// DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};
use super::user_profile::UserProfile;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GetPicsUserResponse {
    pub user: UserProfile,
}