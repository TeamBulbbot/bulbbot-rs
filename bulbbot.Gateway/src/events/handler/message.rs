use crate::events::event_handler::Handler;
use crate::manger_container_structs::DatabaseMangerContainer;
use entity::prelude::{Guilds, Messages};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use tracing::info;
use tracing::log::error;

impl Handler {
    pub async fn handle_message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.guild_id.is_none() {
            return;
        }

        let data = ctx.clone();
        let data_read = data.data.read().await;
        let guild_id = u64::from(msg.guild_id.unwrap());

        let db = data_read
            .get::<DatabaseMangerContainer>()
            .expect("[EVENT/MESSAGE] failed to get the 'database manager container'")
            .get()
            .expect("[EVENT/MESSAGE] the database connection is None");

        let guild_db = Guilds::find_by_guild_id(guild_id).one(db).await;
        if guild_db.is_err() {
            error!("Database failed to get the guild: {:#?}", guild_db.err());
            return;
        }
        let _guild = match guild_db.unwrap() {
            Some(g) => g,
            None => Guilds::create_guild(&db, guild_id).await,
        };

        let inserted_message = match Messages::insert_message(&db, &msg, guild_id).await {
            Ok(result) => result,
            Err(err) => {
                error!("Database insert error on 'Messages::insert_message': {:#?} in guild {} and message id {}", &err, &guild_id, &msg.id);
                return;
            }
        };

        info!("inserted_message {:#?}", inserted_message);
    }
}
