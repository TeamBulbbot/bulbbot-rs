use super::application::Application;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelInviteRequest {
    pub max_age: i32,
    pub max_uses: i32,
    pub temporary: bool,
    pub unique: bool,
    pub target_type: i32,
    pub target_application_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelInviteResponse {
    pub code: String,
    pub target_application: Application,
}
