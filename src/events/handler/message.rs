use entity::prelude::Guilds;
use serenity::model::channel::Message;
use serenity::prelude::Context;

use crate::events::event_handler::Handler;
use crate::{DatabaseMangerContainer, RedisMangerContainer};

impl Handler {
    pub async fn handle_message(&self, ctx: Context, _msg: Message) {
        let mut data = ctx.data.write().await;

        let connection = data
            .get_mut::<RedisMangerContainer>()
            .expect("[EVENT/MESSAGE] failed to get the 'RedisMangerContainer'");

        match connection.ping().await {
            Ok(_) => println!("pinged redis -> PONG"),
            Err(_) => panic!("failed to ping redis"),
        }

        let db = data.get::<DatabaseMangerContainer>().expect("failed");

        // Find by primary key
        let cheese = Guilds::find_by_guild_id(784408056997216327)
            .one(db.get().unwrap())
            .await
            .unwrap();

        println!("{:#?}", cheese);

        // TODO ?
        // self.send_log(&ctx).await;
    }
}
