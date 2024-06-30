use crate::schema::messages;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = messages)]
#[diesel(primary_key(message_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Messages {
    pub message_id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub author_id: i64,
    pub content: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub message_id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub author_id: i64,
    pub content: Option<String>,
}
