use crate::models::gateway::OpCode;

use super::application::ApplicationData;
use super::channel::{ChannelData, ThreadMemberData};
use super::emoji::EmojiData;
use super::guild::{GuildData, ScheduledEventData, UnavailableGuildData};
use super::member::MemberData;
use super::message::MessageData;
use super::presence::PresenceUpdateData;
use super::role::RoleData;
use super::sticker::StickerData;
use super::user::UserData;
use super::voice::{StageInstanceData, VoiceStateData};
use super::Snowflake;

use int_enum::IntEnum;
use serde::{de::Error as DeserializeError, Deserialize, Deserializer, Serialize};
use serde_json::Map;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGatewayData {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SessionStartLimitData {
    pub total: u64,
    pub remaining: u64,
    pub reset_after: u64,
    pub max_concurrency: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGatewayBotData {
    pub url: String,
    pub shards: u64,
    pub session_start_limit: SessionStartLimitData,
}

/// Represents an inbound gateway event.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Serialize)]
#[non_exhaustive]
pub enum WsInboundEvent {
    /// An event was dispatched.
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

macro_rules! dispatch_event_de {
    ($i:ident, $t:ty, $d:ident) => {{
        WsDispatchEvent::$i(<$t>::deserialize($d).map_err(DeserializeError::custom)?)
    }};
}

impl<'de> Deserialize<'de> for WsInboundEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut json = Map::deserialize(deserializer)?;

        Ok(
            #[allow(clippy::cast_possible_truncation)]
            match json
                .remove("op")
                .map(|o| OpCode::from_int(o.as_u64().unwrap() as u8).unwrap())
                .ok_or_else(|| DeserializeError::custom("Missing opcode"))?
            {
                OpCode::Dispatch => {
                    let data = json
                        .remove("d")
                        .ok_or_else(|| DeserializeError::custom("Missing data"))?;

                    let d = match json
                        .remove("t")
                        .ok_or_else(|| DeserializeError::custom("Missing event type"))
                        .and_then(String::deserialize)
                        .map_err(DeserializeError::custom)?
                        .as_str()
                    {
                        "READY" => dispatch_event_de!(Ready, ReadyData, data),
                        "RESUMED" => dispatch_event_de!(Resumed, ResumedData, data),
                        "CHANNEL_CREATE" => {
                            dispatch_event_de!(ChannelCreate, ChannelCreateData, data)
                        }
                        "CHANNEL_UPDATE" => {
                            dispatch_event_de!(ChannelUpdate, ChannelUpdateData, data)
                        }
                        "CHANNEL_DELETE" => {
                            dispatch_event_de!(ChannelDelete, ChannelDeleteData, data)
                        }
                        "THREAD_CREATE" => dispatch_event_de!(ThreadCreate, ThreadCreateData, data),
                        "THREAD_UPDATE" => dispatch_event_de!(ThreadUpdate, ThreadUpdateData, data),
                        "THREAD_DELETE" => dispatch_event_de!(ThreadDelete, ThreadDeleteData, data),
                        "THREAD_LIST_SYNC" => {
                            dispatch_event_de!(ThreadListSync, ThreadListSyncData, data)
                        }
                        "THREAD_MEMBER_UPDATE" => {
                            dispatch_event_de!(ThreadMemberUpdate, ThreadMemberUpdateData, data)
                        }
                        "THREAD_MEMBERS_UPDATE" => {
                            dispatch_event_de!(ThreadMembersUpdate, ThreadMembersUpdateData, data)
                        }
                        "CHANNEL_PINS_UPDATE" => {
                            dispatch_event_de!(ChannelPinsUpdate, ChannelPinsUpdateData, data)
                        }
                        "GUILD_CREATE" => dispatch_event_de!(GuildCreate, GuildCreateData, data),
                        "GUILD_UPDATE" => dispatch_event_de!(GuildUpdate, GuildUpdateData, data),
                        "GUILD_DELETE" => {
                            if Map::deserialize(&data)
                                .map_err(DeserializeError::custom)?
                                .remove("unavailable")
                                .map(|o| o.as_bool().unwrap())
                                .unwrap_or(false)
                            {
                                dispatch_event_de!(GuildUnavailable, UnavailableGuildData, data)
                            } else {
                                dispatch_event_de!(GuildDelete, GuildDeleteData, data)
                            }
                        }
                        "GUILD_BAN_ADD" => dispatch_event_de!(GuildBanAdd, GuildBanAddData, data),
                        "GUILD_BAN_REMOVE" => {
                            dispatch_event_de!(GuildBanRemove, GuildBanRemoveData, data)
                        }
                        "GUILD_EMOJIS_UPDATE" => {
                            dispatch_event_de!(GuildEmojisUpdate, GuildEmojisUpdateData, data)
                        }
                        "GUILD_STICKERS_UPDATE" => {
                            dispatch_event_de!(GuildStickersUpdate, GuildStickersUpdateData, data)
                        }
                        "GUILD_INTEGRATIONS_UPDATE" => {
                            dispatch_event_de!(
                                GuildIntegrationsUpdate,
                                GuildIntegrationsUpdateData,
                                data
                            )
                        }
                        "GUILD_MEMBER_ADD" => {
                            dispatch_event_de!(GuildMemberAdd, GuildMemberAddData, data)
                        }
                        "GUILD_MEMBER_UPDATE" => {
                            dispatch_event_de!(GuildMemberUpdate, GuildMemberUpdateData, data)
                        }
                        "GUILD_MEMBER_REMOVE" => {
                            dispatch_event_de!(GuildMemberRemove, GuildMemberRemoveData, data)
                        }
                        "GUILD_MEMBERS_CHUNK" => {
                            dispatch_event_de!(GuildMembersChunk, GuildMembersChunkData, data)
                        }
                        "GUILD_ROLE_CREATE" => {
                            dispatch_event_de!(GuildRoleCreate, GuildRoleCreateData, data)
                        }
                        "GUILD_ROLE_UPDATE" => {
                            dispatch_event_de!(GuildRoleUpdate, GuildRoleUpdateData, data)
                        }
                        "GUILD_ROLE_DELETE" => {
                            dispatch_event_de!(GuildRoleDelete, GuildRoleDeleteData, data)
                        }
                        "GUILD_SCHEDULED_EVENTS_CREATE" => {
                            dispatch_event_de!(
                                GuildScheduledEventCreate,
                                GuildScheduledEventCreateData,
                                data
                            )
                        }
                        "GUILD_SCHEDULED_EVENTS_UPDATE" => {
                            dispatch_event_de!(
                                GuildScheduledEventUpdate,
                                GuildScheduledEventUpdateData,
                                data
                            )
                        }
                        "GUILD_SCHEDULED_EVENTS_DELETE" => {
                            dispatch_event_de!(
                                GuildScheduledEventDelete,
                                GuildScheduledEventDeleteData,
                                data
                            )
                        }
                        "GUILD_SCHEDULED_EVENT_USER_ADD" => {
                            dispatch_event_de!(
                                GuildScheduledEventUserAdd,
                                GuildScheduledEventUserAddData,
                                data
                            )
                        }
                        "GUILD_SCHEDULED_EVENT_USER_REMOVE" => {
                            dispatch_event_de!(
                                GuildScheduledEventUserRemove,
                                GuildScheduledEventUserRemoveData,
                                data
                            )
                        }
                        _ => return Err(DeserializeError::custom("unsupported event received")),
                    };

                    Self::Dispatch(
                        json.remove("s")
                            .ok_or_else(|| DeserializeError::custom("Missing sequence"))
                            .and_then(u64::deserialize)
                            .map_err(DeserializeError::custom)?,
                        d,
                    )
                }
                OpCode::Heartbeat => Self::Heartbeat(
                    json.remove("s")
                        .ok_or_else(|| DeserializeError::custom("Missing sequence"))
                        .and_then(u64::deserialize)
                        .map_err(DeserializeError::custom)?,
                ),
                OpCode::Reconnect => Self::Reconnect,
                OpCode::Hello => {
                    let mut d = json
                        .remove("d")
                        .ok_or_else(|| DeserializeError::custom("Missing data"))
                        .and_then(Map::deserialize)
                        .map_err(DeserializeError::custom)?;

                    Self::Hello(
                        d.remove("heartbeat_interval")
                            .ok_or_else(|| DeserializeError::custom("Missing heartbeat_interval"))
                            .and_then(u16::deserialize)
                            .map_err(DeserializeError::custom)?,
                    )
                }
                OpCode::HeartbeatAck => Self::HeartbeatAck,
                _ => return Err(DeserializeError::custom("Invalid opcode")),
            },
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReadyData {
    pub v: u8,
    pub user: UserData,
    pub guilds: Vec<UnavailableGuildData>,
    pub session_id: String,
    pub shard: Option<[u32; 2]>,
    pub application: ApplicationData,
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
}
