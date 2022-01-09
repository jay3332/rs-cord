use crate::models::gateway::OpCode;

use super::Snowflake;
use super::channel::{ChannelData, ThreadMemberData};
use super::emoji::EmojiData;
use super::guild::{GuildData, UnavailableGuildData};
use super::member::MemberData;
use super::message::MessageData;
use super::presence::PresenceUpdateData;
use super::role::RoleData;
use super::sticker::StickerData;
use super::user::UserData;
use super::voice::{VoiceStateData, StageInstanceData};

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
                OpCode::Dispatch => Self::Dispatch(
                    json.remove("s")
                        .ok_or_else(|| DeserializeError::custom("Missing sequence"))
                        .and_then(u64::deserialize)
                        .map_err(DeserializeError::custom)?,
                    json.remove("d")
                        .ok_or_else(|| DeserializeError::custom("Missing data"))
                        .and_then(WsDispatchEvent::deserialize)
                        .map_err(DeserializeError::custom)?,
                ),
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
    pub application: (),  // TODO application object
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
#[non_exhaustive]
pub enum WsDispatchEvent {
    Ready(ReadyData),
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
}
