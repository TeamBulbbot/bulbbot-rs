use entity::prelude::GuildLoggings;
use entity::sea_orm::EntityTrait;
use entity::DatabaseConnection;

#[derive(Debug)]
pub enum LogType {
    MessageUpdate,
    MessageDelete,
    GuildMemberAddition,
    GuildMemberRemoval,
    MuteRemove,
}

pub async fn database_column(
    db: &DatabaseConnection,
    guild_id: u64,
    log_type: &LogType,
) -> Option<String> {
    let logging_result = GuildLoggings::find_by_id(guild_id.to_string())
        .one(db)
        .await;

    let logging_model = logging_result
        .expect("[LOGGER] Database error when retreving channel id")
        .expect("[LOGGER] Logging table is empty for");

    match log_type {
        LogType::MessageDelete | LogType::MessageUpdate => logging_model.message,
        LogType::GuildMemberAddition | LogType::GuildMemberRemoval => logging_model.member,
        LogType::MuteRemove => logging_model.mod_action,
    }
}

// Used for the Redis cache key, should be something related to the name in the database column
pub fn database_column_name(log_type: &LogType) -> String {
    match log_type {
        LogType::MessageDelete | LogType::MessageUpdate => String::from("Message"),
        LogType::GuildMemberAddition | LogType::GuildMemberRemoval => String::from("Member"),
        LogType::MuteRemove => String::from("ModAction"),
    }
}
