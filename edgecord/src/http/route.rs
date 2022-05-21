#![allow(clippy::useless_format)]
use crate::model::id::marker::{
    ApplicationMarker, ChannelMarker, CommandMarker, EmojiMarker, GuildMarker, IntegrationMarker,
    InteractionMarker, MessageMarker, RoleMarker, ScheduledEventMarker, StickerMarker, UserMarker,
    WebhookMarker,
};
use crate::model::id::Id;
use crate::model::UrlEncodedEmoji;
use std::fmt::{Display, Formatter};

pub enum Routes {
    // https://discord.com/developers/docs/resources/audit-log
    /// `/guilds/{guild.id}/audit-logs`
    GuildAuditLogs(Id<GuildMarker>),
    // https://discord.com/developers/docs/resources/channel
    /// `/channels/{channel.id}`
    Channel(Id<ChannelMarker>),
    /// `/channels/{channel.id}/messages`
    ChannelMessages(Id<ChannelMarker>),
    /// `/channels/{channel.id}/messages/{message.id}`
    ChannelMessage(Id<ChannelMarker>, Id<MessageMarker>),
    /// `/channels/{channel.id}/messages/{message.id}/crosspost`
    ChannelMessageCrosspost(Id<ChannelMarker>, Id<MessageMarker>),
    /// `/channels/{channel.id}/messages/{message.id}/reactions/{emoji}/@me`
    ChannelMessageReactionEmojiMe(Id<ChannelMarker>, Id<MessageMarker>, UrlEncodedEmoji),
    /// `/channels/{channel.id}/messages/{message.id}/reactions/{emoji}/{user.id}`
    ChannelMessageReactionEmojiUser(
        Id<ChannelMarker>,
        Id<MessageMarker>,
        UrlEncodedEmoji,
        Id<UserMarker>,
    ),
    /// `/channels/{channel.id}/messages/{message.id}/reactions/{emoji}`
    ChannelMessageReactionEmoji(Id<ChannelMarker>, Id<MessageMarker>, UrlEncodedEmoji),
    /// `/channels/{channel.id}/messages/{message.id}/reactions`
    ChannelMessageReactions(Id<ChannelMarker>, Id<MessageMarker>),
    /// `/channels/{channel.id}/messages/bulk-delete`
    ChannelMessagesBulkDelete(Id<ChannelMarker>),
    /// `/channels/{channel.id}/permissions/{overwrite.id}`
    ChannelPermissionsOverwriteMember(Id<ChannelMarker>, Id<UserMarker>),
    /// `/channels/{channel.id}/permissions/{overwrite.id}`
    ChannelPermissionsOverwriteRole(Id<ChannelMarker>, Id<RoleMarker>),
    /// `/channels/{channel.id}/invites`
    ChannelInvites(Id<ChannelMarker>),
    /// `/channels/{channel.id}/followers`
    ChannelFollowers(Id<ChannelMarker>),
    /// `/channels/{channel.id}/typing`
    ChannelTyping(Id<ChannelMarker>),
    /// `/channels/{channel.id}/pins`
    ChannelPins(Id<ChannelMarker>),
    /// `/channels/{channel.id}/pins/{message.id}`
    ChannelPinMessage(Id<ChannelMarker>, Id<MessageMarker>),
    /// `/channels/{channel.id}/recipients/{user.id}`
    ChannelRecipientUser(Id<ChannelMarker>, Id<UserMarker>),
    /// `/channels/{channel.id}/messages/{message.id}/threads`
    ChannelMessageThreads(Id<ChannelMarker>, Id<MessageMarker>),
    /// `/channels/{channel.id}/threads`
    ChannelThreads(Id<ChannelMarker>),
    /// `/channels/{channel.id}/thread-members/@me`
    ChannelThreadMemberMe(Id<ChannelMarker>),
    /// `/channels/{channel.id}/thread-members/{user.id}`
    ChannelThreadMember(Id<ChannelMarker>, Id<UserMarker>),
    /// `/channels/{channel.id}/thread-members`
    ChannelThreadMembers(Id<ChannelMarker>),
    /// `/channels/{channel.id}/threads/archived/public`
    ChannelThreadsArchivedPublic(Id<ChannelMarker>),
    /// `/channels/{channel.id}/threads/archived/private`
    ChannelThreadsArchivedPrivate(Id<ChannelMarker>),
    /// `/channels/{channel.id}/users/@me/threads/archived/private`
    ChannelUserMeThreadArchivedPrivate(Id<ChannelMarker>),
    // https://discord.com/developers/docs/resources/emoji
    /// `/guilds/{guild.id}/emojis`
    GuildEmojis(Id<GuildMarker>),
    /// `/guilds/{guild.id}/emojis/{emoji.id}`
    GuildEmoji(Id<GuildMarker>, Id<EmojiMarker>),

    // https://discord.com/developers/docs/resources/guild
    /// `/guilds`
    Guilds,
    /// `/guilds/{guild.id}`
    Guild(Id<GuildMarker>),
    /// `/guilds/{guild.id}/preview`
    GuildPreview(Id<GuildMarker>),
    /// `/guilds/{guild.id}/channels`
    GuildChannels(Id<GuildMarker>),
    /// `/guilds/{guild.id}/threads/active`
    GuildThreadsActive(Id<GuildMarker>),
    /// `/guilds/{guild.id}/members/{user.id}`
    GuildMember(Id<GuildMarker>, Id<UserMarker>),
    /// `/guilds/{guild.id}/members`
    GuildMembers(Id<GuildMarker>),
    /// `/guilds/{guild.id}/members/search`
    GuildMembersSearch(Id<GuildMarker>),
    /// `/guilds/{guild.id}/members/@me`
    GuildMemberMe(Id<GuildMarker>),
    /// `/guilds/{guild.id}/members/@me/nick`
    GuildMemberMeNick(Id<GuildMarker>),
    /// `/guilds/{guild.id}/members/{user.id}/roles/{role.id}`
    GuildMemberRole(Id<GuildMarker>, Id<UserMarker>, Id<RoleMarker>),
    /// `/guilds/{guild.id}/bans`
    GuildBans(Id<GuildMarker>),
    /// `/guilds/{guild.id}/bans/{user.id}`
    GuildBan(Id<GuildMarker>, Id<UserMarker>),
    /// `/guilds/{guild.id}/roles`
    GuildRoles(Id<GuildMarker>),
    /// `/guilds/{guild.id}/roles/{role.id}`
    GuildRole(Id<GuildMarker>, Id<RoleMarker>),
    /// `/guilds/{guild.id}/prune`
    GuildPrune(Id<GuildMarker>),
    /// `/guilds/{guild.id}/regions`
    GuildRegions(Id<GuildMarker>),
    /// `/guilds/{guild.id}/invites`
    GuildInvites(Id<GuildMarker>),
    /// `/guilds/{guild.id}/integrations`
    GuildIntegrations(Id<GuildMarker>),
    /// `/guilds/{guild.id}/integrations/{integration.id}`
    GuildIntegration(Id<GuildMarker>, Id<IntegrationMarker>),
    /// `/guilds/{guild.id}/widget`
    GuildWidget(Id<GuildMarker>),
    /// `/guilds/{guild.id}/widget.json`
    GuildWidgetJson(Id<GuildMarker>),
    /// `/guilds/{guild.id}/vanity-url`
    GuildVanityUrl(Id<GuildMarker>),
    /// `/guilds/{guild.id}/widget.png`
    GuildWidgetPng(Id<GuildMarker>),
    /// `/guilds/{guild.id}/welcome-screen`
    GuildWelcomeScreen(Id<GuildMarker>),
    /// `/guilds/{guild.id}/voice-states/@me`
    GuildVoiceStateMe(Id<GuildMarker>),
    /// `/guilds/{guild.id}/voice-states/{user.id}`
    GuildVoiceState(Id<GuildMarker>, Id<UserMarker>),

    // https://discord.com/developers/docs/resources/guild-scheduled-event
    /// `/guilds/{guild.id}/scheduled-events`
    GuildScheduledEvents(Id<GuildMarker>),
    /// `/guilds/{guild.id}/scheduled-events/{guild_scheduled_event.id}`
    GuildScheduledEvent(Id<GuildMarker>, Id<ScheduledEventMarker>),
    /// `/guilds/{guild.id}/scheduled-events/{guild_scheduled_event.id}/users`
    GuildScheduledEventUsers(Id<GuildMarker>, Id<ScheduledEventMarker>),
    // https://discord.com/developers/docs/resources/guild-template
    /// `/guilds/{guild.id}/templates`
    GuildTemplates(Id<GuildMarker>),
    /// `/guilds/{guild.id}/templates/{template.code}`
    GuildTemplate(Id<GuildMarker>, String),

    // https://discord.com/developers/docs/resources/invite
    /// `/invites/{invite.code}`
    Invite(String),

    // https://discord.com/developers/docs/resources/stage-instance
    /// `/stage-instances`
    StageInstances,
    /// `/stage-instances/{channel.id}`
    StageInstance(Id<ChannelMarker>),

    // https://discord.com/developers/docs/resources/sticker
    /// `/stickers/{sticker.id}`
    Sticker(Id<StickerMarker>),
    /// `/sticker-packs`
    StickerPacks,
    /// `/guilds/{guild.id}/stickers`
    GuildStickers(Id<GuildMarker>),
    /// `/guilds/{guild.id}/stickers/{sticker.id}`
    GuildSticker(Id<GuildMarker>, Id<StickerMarker>),

    // https://discord.com/developers/docs/resources/user
    /// `/users/{user.id}`
    User(Id<UserMarker>),
    /// `/users/@me`
    UserMe,
    /// `/users/@me/guilds/{guild.id}/member`
    UserMeGuildMember(Id<GuildMarker>),
    /// `/users/@me/guilds/{guild.id}`
    UserMeGuild(Id<GuildMarker>),
    /// `/users/@me/channels`
    UserMeChannels,
    /// `/users/@me/connections`
    UserMeConnections,

    // https://discord.com/developers/docs/resources/voice
    /// `/voice/regions`
    VoiceRegions,

    // https://discord.com/developers/docs/resources/webhook
    /// `/channels/{channel.id}/webhooks`
    ChannelWebhooks(Id<ChannelMarker>),
    /// `/guilds/{guild.id}/webhooks`
    GuildWebhooks(Id<GuildMarker>),
    /// `/webhooks/{webhook.id}`
    Webhook(Id<WebhookMarker>),
    /// `/webhooks/{webhook.id}/{webhook.token}`
    WebhookWithToken(Id<WebhookMarker>, String),
    /// `/webhooks/{webhook.id}/{webhook.token}/slack`
    WebhookWithTokenSlack(Id<WebhookMarker>, String),
    /// `/webhooks/{webhook.id}/{webhook.token}/github`
    WebhookWithTokenGitHub(Id<WebhookMarker>, String),
    /// `/webhooks/{webhook.id}/{webhook.token}/messages/{message.id}`
    WebhookWithTokenMessage(Id<WebhookMarker>, String, Id<MessageMarker>),

    // https://discord.com/developers/docs/interactions/application-commands
    /// `/applications/{application.id}/commands`
    ApplicationCommands(Id<ApplicationMarker>),
    /// `/applications/{application.id}/commands/{command.id}`
    ApplicationCommand(Id<ApplicationMarker>, Id<CommandMarker>),
    /// `/applications/{application.id}/guilds/{guild.id}/commands`
    ApplicationGuildCommands(Id<ApplicationMarker>, Id<GuildMarker>),
    /// `/applications/{application.id}/guilds/{guild.id}/commands/{command.id}`
    ApplicationGuildCommand(Id<ApplicationMarker>, Id<GuildMarker>, Id<CommandMarker>),
    /// `/applications/{application.id}/guilds/{guild.id}/commands/permissions`
    ApplicationGuildCommandsPermissions(Id<ApplicationMarker>, Id<GuildMarker>),
    /// `/applications/{application.id}/guilds/{guild.id}/commands/{command.id}/permissions`
    ApplicationGuildCommandPermissions(Id<ApplicationMarker>, Id<GuildMarker>, Id<CommandMarker>),

    // https://discord.com/developers/docs/interactions/receiving-and-responding
    /// `/interactions/{interaction.id}/{interaction.token}/callback`
    InteractionCallback(Id<InteractionMarker>, String),
    /// `/webhooks/{application.id}/{interaction.token}/messages/@original`
    ApplicationInteractionOriginalMessage(Id<ApplicationMarker>, String),
    /// `/webhooks/{application.id}/{interaction.token}`
    ApplicationInteraction(Id<ApplicationMarker>, String),
    /// `/webhooks/{application.id}/{interaction.token}/messages/{message.id}`
    ApplicationInteractionMessage(Id<ApplicationMarker>, String, Id<MessageMarker>),
}

impl Display for Routes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let path = match self {
            Routes::GuildAuditLogs(guild_id) => format!("/guilds/{guild_id}/audit-logs"),
            Routes::Channel(channel_id) => format!("/channels/{channel_id}"),
            Routes::ChannelMessages(channel_id) => format!("/channels/{channel_id}/messages"),
            Routes::ChannelMessage(channel_id, message_id) => {
                format!("/channels/{channel_id}/messages/{message_id}")
            }
            Routes::ChannelMessageCrosspost(channel_id, message_id) => {
                format!("/channels/{channel_id}/messages/{message_id}/crosspost")
            }
            Routes::ChannelMessageReactionEmojiMe(channel_id, message_id, emoji) => format!("/channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me"),
            Routes::ChannelMessageReactionEmojiUser(channel_id, message_id, emoji, user_id) => format!("/channels/{channel_id}/messages/{message_id}/reactions/{emoji}/{user_id}"),
            Routes::ChannelMessageReactionEmoji(channel_id, message_id, emoji) => format!("/channels/{channel_id}/messages/{message_id}/reactions/{emoji}"),
            Routes::ChannelMessageReactions(channel_id, message_id) => format!("/channels/{channel_id}/messages/{message_id}/reactions"),
            Routes::ChannelMessagesBulkDelete(channel_id) => format!("/channels/{channel_id}/messages/bulk-delete"),
            Routes::ChannelPermissionsOverwriteMember(channel_id, overwrite_id) => format!("/channels/{channel_id}/permissions/{overwrite_id}"),
            Routes::ChannelPermissionsOverwriteRole(channel_id, overwrite_id) => format!("/channels/{channel_id}/permissions/{overwrite_id}"),
            Routes::ChannelInvites(channel_id) => format!("/channels/{channel_id}/invites"),
            Routes::ChannelFollowers(channel_id) => format!("/channels/{channel_id}/followers"),
            Routes::ChannelTyping(channel_id) => format!("/channels/{channel_id}/typing"),
            Routes::ChannelPins(channel_id) => format!("/channels/{channel_id}/pins"),
            Routes::ChannelPinMessage(channel_id, message_id) => format!("/channels/{channel_id}/pins/{message_id}"),
            Routes::ChannelRecipientUser(channel_id, user_id) => format!("/channels/{channel_id}/recipients/{user_id}"),
            Routes::ChannelMessageThreads(channel_id, message_id) => format!("/channels/{channel_id}/messages/{message_id}/threads"),
            Routes::ChannelThreads(channel_id) => format!("/channels/{channel_id}/threads"),
            Routes::ChannelThreadMemberMe(channel_id) => format!("/channels/{channel_id}/thread-members/@me"),
            Routes::ChannelThreadMember(channel_id, user_id) => format!("/channels/{channel_id}/thread-members/{user_id}"),
            Routes::ChannelThreadMembers(channel_id) => format!("/channels/{channel_id}/thread-members"),
            Routes::ChannelThreadsArchivedPublic(channel_id) => format!("/channels/{channel_id}/threads/archived/public"),
            Routes::ChannelThreadsArchivedPrivate(channel_id) => format!("/channels/{channel_id}/threads/archived/private"),
            Routes::ChannelUserMeThreadArchivedPrivate(channel_id) => format!("/channels/{channel_id}/users/@me/threads/archived/private"),
            Routes::GuildEmojis(guild_id) => format!("/guilds/{guild_id}/emojis"),
            Routes::GuildEmoji(guild_id, emoji_id) => format!("/guilds/{guild_id}/emojis/{emoji_id}"),
            Routes::Guilds => format!("/guilds"),
            Routes::Guild(guild_id) => format!("/guilds/{guild_id}"),
            Routes::GuildPreview(guild_id) => format!("/guilds/{guild_id}/preview"),
            Routes::GuildChannels(guild_id) => format!("/guilds/{guild_id}/channels"),
            Routes::GuildThreadsActive(guild_id) => format!("/guilds/{guild_id}/threads/active"),
            Routes::GuildMember(guild_id, user_id) => format!("/guilds/{guild_id}/members/{user_id}"),
            Routes::GuildMembers(guild_id) => format!("/guilds/{guild_id}/members"),
            Routes::GuildMembersSearch(guild_id) => format!("/guilds/{guild_id}/members/search"),
            Routes::GuildMemberMe(guild_id) => format!("/guilds/{guild_id}/members/@me"),
            Routes::GuildMemberMeNick(guild_id) => format!("/guilds/{guild_id}/members/@me/nick"),
            Routes::GuildMemberRole(guild_id, user_id, role_id) => format!("/guilds/{guild_id}/members/{user_id}/roles/{role_id}"),
            Routes::GuildBans(guild_id) => format!("/guilds/{guild_id}/bans"),
            Routes::GuildBan(guild_id, user_id) => format!("/guilds/{guild_id}/bans/{user_id}"),
            Routes::GuildRoles(guild_id) => format!("/guilds/{guild_id}/roles"),
            Routes::GuildRole(guild_id, role_id) => format!("/guilds/{guild_id}/roles/{role_id}"),
            Routes::GuildPrune(guild_id) => format!("/guilds/{guild_id}/prune"),
            Routes::GuildRegions(guild_id) => format!("/guilds/{guild_id}/regions"),
            Routes::GuildInvites(guild_id) => format!("/guilds/{guild_id}/invites"),
            Routes::GuildIntegrations(guild_id) => format!("/guilds/{guild_id}/integrations"),
            Routes::GuildIntegration(guild_id, integration_id) => format!("/guilds/{guild_id}/integrations/{integration_id}"),
            Routes::GuildWidget(guild_id) => format!("/guilds/{guild_id}/widget"),
            Routes::GuildWidgetJson(guild_id) => format!("/guilds/{guild_id}/widget_json"),
            Routes::GuildVanityUrl(guild_id) => format!("/guilds/{guild_id}/vanity-url"),
            Routes::GuildWidgetPng(guild_id) => format!("/guilds/{guild_id}/widget_png"),
            Routes::GuildWelcomeScreen(guild_id) => format!("/guilds/{guild_id}/welcome-screen"),
            Routes::GuildVoiceStateMe(guild_id) => format!("/guilds/{guild_id}/voice-states/@me"),
            Routes::GuildVoiceState(guild_id, user_id) => format!("/guilds/{guild_id}/voice-states/{user_id}"),
            Routes::GuildScheduledEvents(guild_id) => format!("/guilds/{guild_id}/scheduled-events"),
            Routes::GuildScheduledEvent(guild_id, guild_scheduled_event_id) => format!("/guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}"),
            Routes::GuildScheduledEventUsers(guild_id, guild_scheduled_event_id) => format!("/guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}/users"),
            Routes::GuildTemplates(guild_id) => format!("/guilds/{guild_id}/templates"),
            Routes::GuildTemplate(guild_id, template_code) => format!("/guilds/{guild_id}/templates/{template_code}"),
            Routes::Invite(invite_code) => format!("/invites/{invite_code}"),
            Routes::StageInstances => format!("/stage-instances"),
            Routes::StageInstance(channel_id) => format!("/stage-instances/{channel_id}"),
            Routes::Sticker(sticker_id) => format!("/stickers/{sticker_id}"),
            Routes::StickerPacks => format!("/sticker-packs"),
            Routes::GuildStickers(guild_id) => format!("/guilds/{guild_id}/stickers"),
            Routes::GuildSticker(guild_id, sticker_id) => format!("/guilds/{guild_id}/stickers/{sticker_id}"),
            Routes::User(user_id) => format!("/users/{user_id}"),
            Routes::UserMe => format!("/users/@me"),
            Routes::UserMeGuildMember(guild_id) => format!("/users/@me/guilds/{guild_id}/member"),
            Routes::UserMeGuild(guild_id) => format!("/users/@me/guilds/{guild_id}"),
            Routes::UserMeChannels => format!("/users/@me/channels"),
            Routes::UserMeConnections => format!("/users/@me/connections"),
            Routes::VoiceRegions => format!("/voice/regions"),
            Routes::ChannelWebhooks(channel_id) => format!("/channels/{channel_id}/webhooks"),
            Routes::GuildWebhooks(guild_id) => format!("/guilds/{guild_id}/webhooks"),
            Routes::Webhook(webhook_id) => format!("/webhooks/{webhook_id}"),
            Routes::WebhookWithToken(webhook_id, webhook_token) => format!("/webhooks/{webhook_id}/{webhook_token}"),
            Routes::WebhookWithTokenSlack(webhook_id, webhook_token) => format!("/webhooks/{webhook_id}/{webhook_token}/slack"),
            Routes::WebhookWithTokenGitHub(webhook_id, webhook_token) => format!("/webhooks/{webhook_id}/{webhook_token}/github"),
            Routes::WebhookWithTokenMessage(webhook_id, webhook_token, message_id) => format!("/webhooks/{webhook_id}/{webhook_token}/messages/{message_id}"),
            Routes::ApplicationCommands(application_id) => format!("/applications/{application_id}/commands"),
            Routes::ApplicationCommand(application_id, command_id) => format!("/applications/{application_id}/commands/{command_id}"),
            Routes::ApplicationGuildCommands(application_id, guild_id) => format!("/applications/{application_id}/guilds/{guild_id}/commands"),
            Routes::ApplicationGuildCommand(application_id, guild_id, command_id) => format!("/applications/{application_id}/guilds/{guild_id}/commands/{command_id}"),
            Routes::ApplicationGuildCommandsPermissions(application_id, guild_id) => format!("/applications/{application_id}/guilds/{guild_id}/commands/permissions"),
            Routes::ApplicationGuildCommandPermissions(application_id, guild_id, command_id) => format!("/applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions"),
            Routes::InteractionCallback(interaction_id, interaction_token) => format!("/interactions/{interaction_id}/{interaction_token}/callback"),
            Routes::ApplicationInteractionOriginalMessage(application_id, interaction_token) => format!("/webhooks/{application_id}/{interaction_token}/messages/@original"),
            Routes::ApplicationInteraction(application_id, interaction_token) => format!("/webhooks/{application_id}/{interaction_token}"),
            Routes::ApplicationInteractionMessage(application_id, interaction_token, message_id) => format!("/webhooks/{application_id}/{interaction_token}/messages/{message_id}"),
        };
        write!(f, "{}", path)
    }
}
