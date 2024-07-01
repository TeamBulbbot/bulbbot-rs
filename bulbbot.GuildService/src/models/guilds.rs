use crate::schema::guilds;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = guilds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Guilds {
    pub id: i64,
    pub premium: bool,
    pub developer: bool,
}

#[derive(Insertable)]
#[diesel(table_name = guilds)]
pub struct NewGuild {
    pub id: i64,
}
