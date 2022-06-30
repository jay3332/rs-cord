use serde::{Serialize, Deserialize};
use crate::snowflake::{GuildSnowflake, ChannelSnowflake};

type Snowflake = u64;

// Represents a websocket inbound event from Discord.
#[derive(Clone, Debug, Serialize)]
pub enum WsInboundEvent {
    /// The sequence and the event that was dispatched.
    Dispatch(u64, WsDispatchEvent), // (seq, event)

    /// Fired periodically to keep the connection alive.
    Heartbeat(u64), // (seq)

    /// Request to reconnect to the gateway.
    Reconnect,

    /// The session has been invalidated.
    InvalidSession(bool), // (should_resume)

    /// Sent immediately after connecting.
    Hello(u16), // (heartbeat_interval)

    /// Sent in response to receiving a heartbeat to acknowledge that it has been received.
    HeartbeatAck,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReadyData {
    pub v: u8,
    pub user: UserData,
    pub guilds: Vec<UnavailableGuildData>,
    pub session_id: String,
    pub shard: Option<Vec<[u32; 2]>>,
    pub application: PartialApplicationData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResumedData {
    #[serde(rename = "_trace")]
    pub trace: Vec<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ChannelCreateData {
    /// The channel that was created.
    pub channel: ChannelData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ChannelUpdateData {
    /// The channel that was updated.
    pub channel: ChannelData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ChannelDeleteData {
    /// The channel that was deleted.
    pub channel: ChannelData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ThreadCreateData {
    /// The thread that was created.
    pub thread: ChannelData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ThreadUpdateData {
    /// The thread that was updated.
    pub thread: ChannelData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ThreadDeleteData {
    /// The thread that was deleted.
    pub thread: ChannelData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreadListSyncData {
    /// The guild id that the thread list was synced for.
    pub guild_id: Snowflake,
    /// The parent channel ids whose threads are being synced. If omitted, then threads were synced for the entire guild.
    pub channel_ids: Option<Vec<Snowflake>>,
    /// All active threads in the given channels that the current user can access.
    pub threads: Vec<ChannelData>,
    /// All `ThreadMember` objects from the synced threads for the current user, indicating which threads the current user has been added to.
    pub members: Vec<ThreadMemberData>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ThreadMemberUpdateData {
    /// The `ThreadMember` that was updated.
    pub member: ThreadMemberData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreadMembersUpdateData {
    /// The thread id that the members were updated for.
    pub id: Snowflake,
    /// The guild id of the thread.
    pub guild_id: Snowflake,
    /// The approximate number of members in the thread, capped at 50.
    pub member_count: u8,
    /// The user who were added to the thread.
    pub added_members: Option<Vec<ThreadMemberData>>,
    /// The id of the user who were removed from the thread.
    pub removed_member_ids: Option<Vec<Snowflake>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChannelPinsUpdateData {
    /// The guild id of the channel's guild.
    pub guild_id: Option<Snowflake>,
    /// The channel id that the pins were updated for.
    pub channel_id: ChannelData,
    /// The message ids that were pinned.
    pub last_pin_timestamp: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GuildCreateData {
    /// The guild that was created.
    pub guild: GuildData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GuildUpdateData {
    /// The guild that was updated.
    pub guild: GuildData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildDeleteData {
    /// The guild that was deleted.
    pub id: Snowflake,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildBanAddData {
    /// The ID of the guild that the ban was added to.
    pub guild_id: Snowflake,

    /// The user that was banned.
    pub user: UserData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildBanRemoveData {
    /// The ID of the guild that the ban was removed from.
    pub guild_id: Snowflake,

    /// The user that was unbanned.
    pub user: UserData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildEmojisUpdateData {
    /// The ID of the guild that the emojis were updated for.
    pub guild_id: Snowflake,

    /// The emojis that were updated.
    pub emojis: Vec<EmojiData>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildStickersUpdateData {
    /// The ID of the guild that the stickers were updated for.
    pub guild_id: Snowflake,

    /// The stickers that were updated.
    pub stickers: Vec<StickerData>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildIntegrationsUpdateData {
    /// The ID of the guild that the integrations were updated for.
    pub guild_id: Snowflake,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GuildMemberAddData {
    pub member: MemberData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildMemberRemoveData {
    /// The ID of the guild that the member was removed from.
    pub guild_id: Snowflake,

    /// The user that was removed.
    pub user: UserData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GuildMemberUpdateData {
    /*
     * This is documented as a separate object,
     * however this object is exactly the same as the Member object
     * with a few fields required instead of optional.
     *
     * We can handle this by just `.unwrap()`ing them when we need to.
     */
    /// The member that was updated.
    pub member: MemberData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildMembersChunkData {
    pub guild_id: Snowflake,
    pub members: Vec<MemberData>,
    pub chunk_index: u32,
    pub chunk_count: u32,
    pub not_found: Option<Vec<Snowflake>>,
    pub presences: Option<Vec<PresenceUpdateData>>,
    pub nonce: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildRoleCreateData {
    /// The ID of the guild that the role was created in.
    pub guild_id: Snowflake,

    /// The role that was created.
    pub role: RoleData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildRoleUpdateData {
    /// The ID of the guild that the role was updated in.
    pub guild_id: Snowflake,

    /// The role that was updated.
    pub role: RoleData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildRoleDeleteData {
    /// The ID of the guild that the role was deleted from.
    pub guild_id: Snowflake,

    /// The ID of the role that was deleted.
    pub role_id: Snowflake,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GuildScheduledEventCreateData {
    pub event: ScheduledEventData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GuildScheduledEventUpdateData {
    pub event: ScheduledEventData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GuildScheduledEventDeleteData {
    pub event: ScheduledEventData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildScheduledEventUserAddData {
    pub guild_scheduled_event_id: Snowflake,
    pub user_id: Snowflake,
    pub guild_id: Snowflake,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildScheduledEventUserRemoveData {
    pub guild_scheduled_event_id: Snowflake,
    pub user_id: Snowflake,
    pub guild_id: Snowflake,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct IntegrationCreateData {
    pub integration: IntegrationData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct IntegrationUpdateData {
    pub integration: IntegrationData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IntegrationDeleteData {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub application_id: Option<Snowflake>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InviteCreateData {
    pub channel_id: Option<Snowflake>,
    pub code: String,
    pub created_at: String,
    pub guild_id: Option<Snowflake>,
    pub inviter: Option<UserData>,
    pub max_age: u64,
    pub max_uses: u64,
    pub target_type: Option<u8>,
    pub target_user: Option<UserData>,
    pub target_application: Option<ApplicationData>,
    pub temporary: bool,
    pub uses: Option<u64>, // always 0
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InviteDeleteData {
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub code: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MessageCreateData {
    pub message: MessageData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MessageUpdateData {
    pub message: ActualMessageUpdateData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageDeleteData {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageDeleteBulkData {
    pub ids: Vec<Snowflake>,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageReactionAddData {
    pub user_id: Snowflake,
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub member: Option<MemberData>,
    pub emoji: PartialEmojiData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageReactionRemoveData {
    pub user_id: Snowflake,
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub emoji: PartialEmojiData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageReactionRemoveAllData {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageReactionRemoveEmojiData {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub emoji: PartialEmojiData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypingStartData {
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub timestamp: u64,
    pub member: Option<MemberData>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UserUpdateData {
    pub user: UserData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VoiceStateUpdateData {
    pub voice_state: VoiceStateData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceServerUpdateData {
    pub token: String,
    pub guild_id: Snowflake,

    // if "None", disconnect from the voice server and
    // only attempt to reconnect when a new voice server is allocated.
    pub endpoint: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhooksUpdateData {
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
}

#[derive(Clone, Debug, Serialize)]
#[non_exhaustive]
pub enum WsDispatchEvent {
    Ready(ReadyData),
    Resumed(ResumedData),
    ChannelCreate(ChannelCreateData),
    ChannelUpdate(ChannelUpdateData),
    ChannelDelete(ChannelDeleteData),
    ThreadCreate(ThreadCreateData),
    ThreadUpdate(ThreadUpdateData),
    ThreadDelete(ThreadDeleteData),
    ThreadListSync(ThreadListSyncData),
    ThreadMemberUpdate(ThreadMemberUpdateData),
    ThreadMembersUpdate(ThreadMembersUpdateData),
    ChannelPinsUpdate(ChannelPinsUpdateData),
    GuildCreate(GuildCreateData),
    GuildUpdate(GuildUpdateData),
    GuildDelete(GuildDeleteData),
    GuildUnavailable(UnavailableGuildData),
    GuildBanAdd(GuildBanAddData),
    GuildBanRemove(GuildBanRemoveData),
    GuildEmojisUpdate(GuildEmojisUpdateData),
    GuildStickersUpdate(GuildStickersUpdateData),
    GuildIntegrationsUpdate(GuildIntegrationsUpdateData),
    GuildMemberAdd(GuildMemberAddData),
    GuildMemberRemove(GuildMemberRemoveData),
    GuildMemberUpdate(GuildMemberUpdateData),
    GuildMembersChunk(GuildMembersChunkData),
    GuildRoleCreate(GuildRoleCreateData),
    GuildRoleUpdate(GuildRoleUpdateData),
    GuildRoleDelete(GuildRoleDeleteData),
    GuildScheduledEventCreate(GuildScheduledEventCreateData),
    GuildScheduledEventUpdate(GuildScheduledEventUpdateData),
    GuildScheduledEventDelete(GuildScheduledEventDeleteData),
    GuildScheduledEventUserAdd(GuildScheduledEventUserAddData),
    GuildScheduledEventUserRemove(GuildScheduledEventUserRemoveData),
    IntegrationCreate(IntegrationCreateData),
    IntegrationUpdate(IntegrationUpdateData),
    IntegrationDelete(IntegrationDeleteData),
    InviteCreate(InviteCreateData),
    InviteDelete(InviteDeleteData),
    MessageCreate(MessageCreateData),
    MessageUpdate(MessageUpdateData),
    MessageDelete(MessageDeleteData),
    MessageDeleteBulk(MessageDeleteBulkData),
    MessageReactionAdd(MessageReactionAddData),
    MessageReactionRemove(MessageReactionRemoveData),
    MessageReactionRemoveAll(MessageReactionRemoveAllData),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmojiData),
    PresenceUpdate(PresenceUpdateData),
    TypingStart(TypingStartData),
    UserUpdate(UserUpdateData),
    VoiceStateUpdate(VoiceStateUpdateData),
    VoiceServerUpdate(VoiceServerUpdateData),
    WebhooksUpdate(WebhooksUpdateData),
}
