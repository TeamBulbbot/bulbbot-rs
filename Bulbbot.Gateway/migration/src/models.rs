use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Guilds {
    Table,
    GuildId,
}

#[derive(Iden)]
pub enum Messages {
    Table,
    MessageId,
    ChannelId,
    AuthorId,
    AuthorTag,
    Content,
    Embed,
    Sticker,
    Attachements,
    GuildId,
}
