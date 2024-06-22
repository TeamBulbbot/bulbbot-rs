use crate::models::guilds::Guilds;
use crate::schema::logging;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Clone,
    Copy,
)]
#[diesel(table_name = logging)]
#[primary_key(guilds_id)]
#[diesel(belongs_to(Guilds, foreign_key = guilds_id))]
pub struct Logging {
    pub guilds_id: i64,
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

#[derive(Insertable)]
#[diesel(table_name = logging)]
pub struct NewLogging {
    pub guilds_id: i64,
}
