use serenity::all::GuildMemberUpdateEvent;
use serenity::async_trait;
use serenity::gateway::ShardStageUpdateEvent;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{
    ChannelId, Guild, GuildChannel, GuildId, InviteCreateEvent, InviteDeleteEvent, Member,
    MessageId, MessageUpdateEvent, PartialGuild, ResumedEvent, Role, RoleId, UnavailableGuild,
};
use serenity::model::user::User;
use serenity::prelude::{Context, EventHandler};

pub struct Handler;

// Event handler for the bot
// https://docs.rs/serenity/latest/serenity/client/trait.EventHandler.html
#[async_trait]
impl EventHandler for Handler {
    // Dispatched upon startup.
    async fn ready(&self, ctx: Context, ready: Ready) {
        self.handle_ready(ctx, ready).await
    }

    // Dispatched upon reconnection.
    async fn resume(&self, ctx: Context, event: ResumedEvent) {
        self.handle_resume(ctx, event).await
    }

    // Dispatched when a shard's connection stage is updated
    async fn shard_stage_update(&self, _ctx: Context, _: ShardStageUpdateEvent) {}

    // Dispatched when a guild is created;
    async fn guild_create(&self, _ctx: Context, _guild: Guild, _is_new: Option<bool>) {}

    // Dispatched when the guild is updated.
    async fn guild_update(
        &self,
        _ctx: Context,
        _old_data_if_available: Option<Guild>,
        _new_but_incomplete: PartialGuild,
    ) {
    }

    // Dispatched when a guild is deleted.
    async fn guild_delete(
        &self,
        _ctx: Context,
        _incomplete: UnavailableGuild,
        _full: Option<Guild>,
    ) {
    }

    // Dispatched when a role is created.
    async fn guild_role_create(&self, _ctx: Context, _new: Role) {}

    // Dispatched when a role is updated.
    async fn guild_role_update(
        &self,
        _ctx: Context,
        _old_data_if_available: Option<Role>,
        _new: Role,
    ) {
    }

    // Dispatched when a role is deleted.
    async fn guild_role_delete(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _removed_role_id: RoleId,
        _removed_role_data_if_available: Option<Role>,
    ) {
    }

    // Dispatched when a user joins a guild.
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        self.handle_guild_member_addition(ctx, new_member).await;
    }

    /// Dispatched when a member is updated (e.g their nickname is updated).
    async fn guild_member_update(
        &self,
        _ctx: Context,
        _old_if_available: Option<Member>,
        _new: Option<Member>,
        _event: GuildMemberUpdateEvent,
    ) {
    }

    // Dispatched when a user's membership ends by leaving, getting kicked, or being banned.
    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        _member_data_if_available: Option<Member>,
    ) {
        self.handle_guild_member_removal(ctx, guild_id, user).await;
    }

    // Dispatched when a channel is created.
    async fn channel_create(&self, _ctx: Context, _channel: GuildChannel) {}

    // Dispatched when a channel is updated.
    async fn channel_update(&self, _ctx: Context, _old: Option<GuildChannel>, _new: GuildChannel) {}

    // Dispatched when a channel is deleted.
    async fn channel_delete(
        &self,
        _ctx: Context,
        _channel: GuildChannel,
        _messages: Option<Vec<Message>>,
    ) {
    }

    // Dispatched when a category is created.
    async fn category_create(&self, _ctx: Context, _category: GuildChannel) {}

    // Dispatched when a category is deleted.
    async fn category_delete(&self, _ctx: Context, _category: GuildChannel) {}

    // Dispatched when a message is created.
    async fn message(&self, context: Context, message: Message) {
        self.handle_message(context, message).await
    }

    // Dispatched when a message is updated.
    async fn message_update(
        &self,
        ctx: Context,
        _old: Option<Message>,
        _new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        self.handle_message_update(ctx, event).await
    }

    // Dispatched when a message is deleted.
    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_message_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
        self.handle_message_delete(ctx, channel_id, deleted_message_id, guild_id)
            .await
    }

    // Dispatched when multiple messages were deleted at once.
    async fn message_delete_bulk(
        &self,
        _ctx: Context,
        _channel_id: ChannelId,
        _multiple_deleted_messages_ids: Vec<MessageId>,
        _guild_id: Option<GuildId>,
    ) {
    }

    // Dispatched when a user is banned from a guild.
    async fn guild_ban_addition(&self, _ctx: Context, _guild_id: GuildId, _banned_user: User) {}

    // Dispatched when a user's ban is lifted from a guild.
    async fn guild_ban_removal(&self, _ctx: Context, _guild_id: GuildId, _unbanned_user: User) {}

    // Dispatched when a invite is created.
    async fn invite_create(&self, _ctx: Context, _data: InviteCreateEvent) {}

    // Dispatched when a invite is deleted.
    async fn invite_delete(&self, _ctx: Context, _data: InviteDeleteEvent) {}
}
