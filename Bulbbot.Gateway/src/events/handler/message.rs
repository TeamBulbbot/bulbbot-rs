use entity::prelude::{Guilds, Messages};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use tracing::log::error;

use crate::events::event_handler::Handler;
use crate::manger_container_structs::DatabaseMangerContainer;

impl Handler {
    pub async fn handle_message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.guild_id.is_none() {
            return;
        }

        let data = ctx.data.write().await;
        let guild_id = u64::from(msg.guild_id.unwrap());

        /*
        let redis = data
            .get_mut::<RedisMangerContainer>()
            .expect("[EVENT/MESSAGE] failed to get the 'RedisMangerContainer'");

        match redis.ping().await {
            Ok(_) => println!("pinged redis -> PONG"),
            Err(_) => panic!("failed to ping redis"),
        }
         */

        // redis.set("secret", "Hello World").await;
        // redis
        //    .set_and_expire_seconds("secret", "Hello World", 5)
        //   .await;

        /* redis.incr("mykey").await;

        let secret_value = redis.get("mykey").await;
        println!("{:#?}", secret_value);
        */

        let db = data
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

        println!("{:#?}", inserted_message);
    }
}
