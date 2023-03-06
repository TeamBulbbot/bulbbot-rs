use entity::prelude::{GuildConfigurations, GuildLoggings, Guilds};
use entity::sea_orm::ModelTrait;
use serenity::model::channel::Message;
use serenity::prelude::Context;

use crate::events::event_handler::Handler;
use crate::{DatabaseMangerContainer, RedisMangerContainer};

use super::loggers::LogType;

impl Handler {
    pub async fn handle_message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        /*let mut data = ctx.data.write().await;

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

        /*
        let db = data
            .get::<DatabaseMangerContainer>()
            .expect("failed")
            .get()
            .unwrap();

        let guild = Guilds::find_by_guild_id(784408056997216327)
            .one(db)
            .await
            .unwrap()
            .unwrap();

        println!("{}", guild.name);


        let config = guild
            .find_related(GuildConfigurations)
            .one(db)
            .await
            .unwrap()
            .unwrap();

        let logging = guild
            .find_related(GuildLoggings)
            .one(db)
            .await
            .unwrap()
            .unwrap();

        println!("{:#?}\n{:#?}\n{:#?}", guild, config, logging);

        */

        // TODO ?
        self.send_log(
            &ctx,
            msg.content.as_str(),
            msg.guild_id,
            LogType::MessageUpdate,
        )
        .await;
    }
}
