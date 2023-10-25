use serenity::model::{channel::Message, prelude::UserId, user::User};

#[derive(Debug)]
pub struct BulbbotUser {
    pub id: UserId,
    pub username: String,
}

impl BulbbotUser {
    pub fn create_user_from_message(msg: &Message) -> Self {
        BulbbotUser {
            id: msg.author.id,
            username: msg.author.name.clone(),
        }
    }

    pub fn create_user_from_user(user: &User) -> Self {
        BulbbotUser {
            id: user.id,
            username: user.name.clone(),
        }
    }
}
