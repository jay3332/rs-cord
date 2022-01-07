use int_enum::IntEnum;

pub const DISCORD_API_URL: &'static str = "https://discord.com/api";

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
#[non_exhaustive]
pub enum OpCodes {
    /// An event was dispatched.
    Dispatch = 0,
    /// Fired periodically by the client to keep the connection alive.
    Heartbeat = 1,
    /// Starts a new session during initial handshake.
    Identify = 2,
    /// Update the client presence/status.
    StatusUpdate = 3,
    /// Used to join/move/leave voice channels.
    VoiceStateUpdate = 4,
    /// Resume a previous session that was disconnected.
    Resume = 6,
    /// Used to tell clients to reconnect to the gateway.
    Reconnect = 7,
    /// Used to request guild members.
    GetGuildMembers = 8,
    /// Used to notify clients that they have an invalid session Id.
    InvalidSession = 9,
    /// Sent immediately after connection, contains heartbeat + server info.
    Hello = 10,
    /// Sent in response to receiving a heartbeat to acknowledge that it has been received.
    HeartbeatAck = 11,
}
