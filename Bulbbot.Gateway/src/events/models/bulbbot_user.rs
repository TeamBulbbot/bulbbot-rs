use serenity::model::{channel::Message, prelude::UserId};

#[derive(Debug)]
pub struct BulbbotUser {
    pub id: UserId,
    pub username: String,
}

impl BulbbotUser {
    pub fn create_user_from_message(msg: &Message) -> BulbbotUser {
        BulbbotUser {
            id: msg.author.id,
            username: msg.author.name.clone(),
        }
    }
}
