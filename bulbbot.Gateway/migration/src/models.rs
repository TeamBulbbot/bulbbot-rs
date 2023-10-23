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

#[derive(Iden)]
pub enum GuildConfigurations {
    Table,
    GuildId,
    LanguageISOCode, // ISO 639-1 - https://www.andiamo.co.uk/resources/iso-language-codes/
    HasPremium,
    AutoRole,
    ActionsOnInfo,
    RolesOnLeave,
    QuickReasons,
    ManualNickNameInfs,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum GuildLoggings {
    Table,
    GuildId,
    ModAction,
    AutoMod,
    Message,
    Role,
    Member,
    Channel,
    Thread,
    JoinLeave,
    Invite,
    Banpool,
    Other,
}