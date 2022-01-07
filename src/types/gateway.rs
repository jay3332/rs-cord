use crate::models::gateway::OpCode;
use super::user::UserData;

use int_enum::IntEnum;
use serde::{Deserialize, Deserializer, Serialize, de::Error as DeserializeError};
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
#[derive(Clone, Debug, Serialize)]
#[non_exhaustive]
pub enum WsInboundEvent {
    /// An event was dispatched.
    Dispatch(u64, WsDispatchEvent),  // (seq, event)

    /// Fired periodically to keep the connection alive.
    Heartbeat(u64),  // (seq)

    /// Request to reconnect to the gateway.
    Reconnect,

    /// The session has been invalidated.
    InvalidSession(bool),  // (should_resume)

    /// Sent immediately after connecting.
    Hello(u16),  // (heartbeat_interval)
    
    /// Sent in response to receiving a heartbeat to acknowledge that it has been received.
    HeartbeatAck,
}

impl<'de> Deserialize<'de> for WsInboundEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut json = Map::deserialize(deserializer)?;

        Ok(
            match json.remove("op")
                .map(|o| OpCode::from_int(o.as_u64().unwrap() as u8).unwrap())
                .ok_or(DeserializeError::custom("Missing opcode"))?
            {
                // TODO: "dispatch" opcode for events
                OpCode::Heartbeat => Self::Heartbeat(
                    json.remove("s")
                        .ok_or(DeserializeError::custom("Missing sequence"))
                        .and_then(u64::deserialize)
                        .map_err(DeserializeError::custom)?,
                ),
                OpCode::Reconnect => Self::Reconnect,
                OpCode::Hello => {
                    let mut d = json.remove("d")
                        .ok_or(DeserializeError::custom("Missing data"))
                        .and_then(Map::deserialize)
                        .map_err(DeserializeError::custom)?;

                    Self::Hello(
                        d.remove("heartbeat_interval")
                            .ok_or(DeserializeError::custom("Missing heartbeat_interval"))
                            .and_then(u16::deserialize)
                            .map_err(DeserializeError::custom)?,
                    )
                },
                OpCode::HeartbeatAck => Self::HeartbeatAck,
                _ => return Err(DeserializeError::custom("Invalid opcode")),
            }
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum WsDispatchEvent {
    Ready {
        v: u8,
        user: UserData,
        guilds: Vec<()>,  // TODO guild object
        session_id: String,
        shard: Option<[u32; 2]>,
        application: (),  // TODO application object
    },
}
