#![allow(non_snake_case)]

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "AutomodPunishmentType"))]
    pub struct AutomodPunishmentType;
}

diesel::table! {
    _prisma_migrations (id) {
        id -> Varchar,
        checksum -> Varchar,
        finished_at -> Nullable<Timestamptz>,
        migration_name -> Varchar,
        logs -> Nullable<Text>,
        rolled_back_at -> Nullable<Timestamptz>,
        started_at -> Timestamptz,
        applied_steps_count -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AutomodPunishmentType;

    automods (id) {
        id -> Int4,
        enabled -> Bool,
        websiteWhitelist -> Nullable<Array<Nullable<Varchar>>>,
        inviteWhitelist -> Nullable<Array<Nullable<Varchar>>>,
        wordBlacklist -> Nullable<Array<Nullable<Varchar>>>,
        limitMentions -> Nullable<Int4>,
        limitMessages -> Nullable<Int4>,
        wordBlacklistToken -> Nullable<Array<Nullable<Varchar>>>,
        timeoutMentions -> Nullable<Int4>,
        timeoutMessages -> Nullable<Int4>,
        ignoreChannels -> Nullable<Array<Nullable<Varchar>>>,
        ignoreRoles -> Nullable<Array<Nullable<Varchar>>>,
        ignoreUsers -> Nullable<Array<Nullable<Varchar>>>,
        avatarHashes -> Nullable<Array<Nullable<Varchar>>>,
        punishmentWebsite -> Nullable<AutomodPunishmentType>,
        punishmentInvites -> Nullable<AutomodPunishmentType>,
        punishmentWords -> Nullable<AutomodPunishmentType>,
        punishmentMentions -> Nullable<AutomodPunishmentType>,
        punishmentMessages -> Nullable<AutomodPunishmentType>,
        punishmentAvatarBans -> Nullable<AutomodPunishmentType>,
    }
}

diesel::table! {
    banpoolSubscribers (id) {
        id -> Int4,
        guildId -> Varchar,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        banpoolId -> Int4,
    }
}

diesel::table! {
    banpools (id) {
        id -> Int4,
        name -> Varchar,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        guildId -> Int4,
    }
}

diesel::table! {
    blacklists (id) {
        id -> Int4,
        isGuild -> Bool,
        name -> Varchar,
        snowflakeId -> Varchar,
        reason -> Varchar,
        developerId -> Varchar,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

diesel::table! {
    experiments (id) {
        id -> Int4,
        name -> Varchar,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        guildId -> Int4,
    }
}

diesel::table! {
    guildConfigurations (id) {
        id -> Int4,
        language -> Varchar,
        premiumGuild -> Bool,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        autorole -> Nullable<Varchar>,
        timezone -> Varchar,
        actionsOnInfo -> Bool,
        rolesOnLeave -> Bool,
        quickReasons -> Nullable<Array<Nullable<Varchar>>>,
        manualNicknameInf -> Bool,
    }
}

diesel::table! {
    guildLoggings (id) {
        id -> Int4,
        modAction -> Nullable<Varchar>,
        automod -> Nullable<Varchar>,
        message -> Nullable<Varchar>,
        role -> Nullable<Varchar>,
        member -> Nullable<Varchar>,
        channel -> Nullable<Varchar>,
        joinLeave -> Nullable<Varchar>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        thread -> Nullable<Varchar>,
        invite -> Nullable<Varchar>,
        other -> Nullable<Varchar>,
        banpool -> Nullable<Varchar>,
    }
}

diesel::table! {
    guilds (id) {
        id -> Int4,
        guildId -> Varchar,
        name -> Varchar,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        guildConfigurationId -> Int4,
        guildLoggingId -> Int4,
        automodId -> Int4,
    }
}

diesel::table! {
    infractions (id) {
        id -> Int4,
        action -> Varchar,
        reason -> Varchar,
        target -> Varchar,
        targetId -> Varchar,
        moderator -> Varchar,
        moderatorId -> Varchar,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        guildId -> Int4,
        timeout -> Nullable<Varchar>,
        active -> Bool,
    }
}

diesel::table! {
    messageLogs (messageId) {
        messageId -> Varchar,
        channelId -> Varchar,
        authorId -> Varchar,
        authorTag -> Varchar,
        content -> Nullable<Varchar>,
        embed -> Nullable<Json>,
        sticker -> Nullable<Json>,
        attachments -> Nullable<Array<Nullable<Varchar>>>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        guildId -> Int4,
    }
}

diesel::table! {
    reminds (id) {
        id -> Int4,
        reason -> Varchar,
        expireTime -> Int8,
        userId -> Varchar,
        channelId -> Nullable<Varchar>,
        messageId -> Nullable<Varchar>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

diesel::table! {
    tempbans (id) {
        id -> Int4,
        targetTag -> Varchar,
        targetId -> Varchar,
        reason -> Varchar,
        expireTime -> Int8,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        guildId -> Int4,
        gId -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _prisma_migrations,
    automods,
    banpoolSubscribers,
    banpools,
    blacklists,
    experiments,
    guildConfigurations,
    guildLoggings,
    guilds,
    infractions,
    messageLogs,
    reminds,
    tempbans,
);
