use crate::models::gateway::OpCode;

use super::channel::ChannelData;
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

#[derive(Clone, Debug)]
pub enum ChannelCreateData {
    /// The channel that was created.
    pub channel: Channel,
}

impl<'de> Deserialize<'de> for ChannelCreateEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            channel: ChannelData::deserialize(deserializer)?,
        })
    }
}

#[derive(Clone, Debug)]
pub enum ChannelUpdateData {
    /// The channel that was updated.
    pub channel: Channel,
}

impl<'de> Deserialize<'de> for ChannelUpdateEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            channel: ChannelData::deserialize(deserializer)?,
        })
    }
}

#[derive(Clone, Debug)]
pub enum ChannelDeleteData {
    /// The channel that was deleted.
    pub channel: Channel,
}

impl<'de> Deserialize<'de> for ChannelDeleteEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            channel: ChannelData::deserialize(deserializer)?,
        })
    }
}

#[derive(Clone, Debug)]
pub enum ThreadCreateData {
    /// The thread that was created.
    pub thread: Channel,
}

impl<'de> Deserialize<'de> for ThreadCreateEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            thread: ChannelData::deserialize(deserializer)?,
        })
    }
}

#[derive(Clone, Debug)]
pub enum ThreadUpdateData {
    /// The thread that was updated.
    pub thread: Channel,
}

impl<'de> Deserialize<'de> for ThreadUpdateEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            thread: ChannelData::deserialize(deserializer)?,
        })
    }
}

#[derive(Clone, Debug)]
pub enum ThreadDeleteData {
    /// The thread that was deleted.
    pub thread: Channel,
}



#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum WsDispatchEvent {
    Ready(ReadyData),
}
