use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuildDto {
    pub id: i64,
    pub premium: bool,
    pub developer: bool,
    pub logging: Logging,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Logging {
    pub mod_action: Option<i64>,
    pub auto_mod: Option<i64>,
    pub message: Option<i64>,
    pub role: Option<i64>,
    pub member: Option<i64>,
    pub channel: Option<i64>,
    pub thread: Option<i64>,
    pub join_leave: Option<i64>,
    pub invite: Option<i64>,
    pub banpool: Option<i64>,
    pub other: Option<i64>,
}
