use bitflags::__impl_bitflags;
use int_enum::IntEnum;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// A set of bitflags which represent what gateway events the client should receive.
/// These can be constructed manually, or by using the [`intents!`] macro.
#[derive(Copy, Clone)]
pub struct Intents {
    /// The raw bits that represent the intent bitflag.
    /// This is used internally by the library, and should not be modified.
    pub bits: u32,
}

__impl_bitflags! {
    Intents: u32 {
        /// Enables all guild-related gateway events.
        GUILDS = 1 << 0;

        /// Enables all member-related gateway events.
        ///
        /// This is a privileged intent. You must toggle the intent in your Developer Portal in order to use this intent.
        GUILD_MEMBERS = 1 << 1;

        /// Enables all ban-related gateway events.
        GUILD_BANS = 1 << 2;

        /// Enables all gateway events related to emojis and stickers.
        GUILD_EMOJIS_AND_STICKERS = 1 << 3;

        /// Enables all gateway events related to guild integrations.
        GUILD_INTEGRATIONS = 1 << 4;

        /// Enables all gateway events related to webhooks.
        GUILD_WEBHOOKS = 1 << 5;

        /// Enables all gateway events related to invites.
        GUILD_INVITES = 1 << 6;

        /// Enables all gateway events related to voice states.
        GUILD_VOICE_STATES = 1 << 7;

        /// Enables all gateway events related to member presences.
        ///
        /// This is a privileged intent. You must toggle the intent in your Developer Portal in order to use this intent.
        /// Note that this intent is also a relatively heavy intent for larger bots - it is recommended to only enable this intent when absolutely necessary.
        GUILD_PRESENCES = 1 << 8;

        /// Enables all gateway events related to messages in guilds only.
        GUILD_MESSAGES = 1 << 9;

        /// Enables all gateway events related to message reactions in guilds only.
        GUILD_MESSAGE_REACTIONS = 1 << 10;

        /// Enables all typing-related gateway events in guilds only.
        GUILD_MESSAGE_TYPING = 1 << 11;

        /// Enables all gateway events related to messages in direct messages only.
        DIRECT_MESSAGES = 1 << 12;

        /// Enables all gateway events related to message reactions in direct messages only.
        DIRECT_MESSAGE_REACTIONS = 1 << 13;

        /// Enables all typing-related gateway events in direct messages only.
        DIRECT_MESSAGE_TYPING = 1 << 14;

        /// Enables all gateway events related to scheduled events.
        GUILD_SCHEDULED_EVENTS = 1 << 16;

        // Aliases

        /// An alias for [`Intents::GUILD_MEMBERS`].
        MEMBERS = Self::GUILD_MEMBERS.bits;

        /// An alias for [`Intents::GUILD_BANS`].
        BANS = Self::GUILD_BANS.bits;

        /// An alias for [`Intents::GUILD_EMOJIS_AND_STICKERS`].
        EMOJIS = Self::GUILD_EMOJIS_AND_STICKERS.bits;

        /// An alias for [`Intents::GUILD_EMOJIS_AND_STICKERS`].
        STICKERS = Self::GUILD_EMOJIS_AND_STICKERS.bits;

        /// An alias for [`Intents::GUILD_INTEGRATIONS`].
        INTEGRATIONS = Self::GUILD_INTEGRATIONS.bits;

        /// An alias for [`Intents::GUILD_WEBHOOKS`].
        WEBHOOKS = Self::GUILD_WEBHOOKS.bits;

        /// An alias for [`Intents::GUILD_INVITES`].
        INVITES = Self::GUILD_INVITES.bits;

        /// An alias for [`Intents::GUILD_VOICE_STATES`].
        VOICE_STATES = Self::GUILD_VOICE_STATES.bits;

        /// An alias for [`Intents::GUILD_PRESENCES`].
        PRESENCES = Self::GUILD_PRESENCES.bits;

        /// An alias for [`Intents::GUILD_SCHEDULED_EVENTS`].
        SCHEDULED_EVENTS = Self::GUILD_SCHEDULED_EVENTS.bits;

        // Multi-flag aliases

        /// A shortcut for enabling both [`Intents::GUILD_MESSAGES`] and [`Intents::DIRECT_MESSAGES`].
        MESSAGES = Self::GUILD_MESSAGES.bits | Self::DIRECT_MESSAGES.bits;

        /// A shortcut for enabling both [`Intents::GUILD_MESSAGE_REACTIONS`] and [`Intents::DIRECT_MESSAGE_REACTIONS`].
        REACTIONS = Self::GUILD_MESSAGE_REACTIONS.bits | Self::DIRECT_MESSAGE_REACTIONS.bits;

        /// A shortcut for enabling both [`Intents::GUILD_MESSAGE_TYPING`] and [`Intents::DIRECT_MESSAGE_TYPING`].
        TYPING = Self::GUILD_MESSAGE_TYPING.bits | Self::DIRECT_MESSAGE_TYPING.bits;
    }
}

impl Serialize for Intents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for Intents {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u32::deserialize(deserializer)?))
    }
}

impl Intents {
    /// Initializes a new set of intent flags that contain all possible intents that are not privileged.
    /// Note that it is generally recommended to explicitly specify intents rather than use this function.
    ///
    /// Currently, this is the default intent set when one is not specified.
    /// This is subject to change in the future to an empty intent set to encourage more explicit intent declarations.
    ///
    /// # See
    /// [`intents!`]
    #[must_use]
    pub fn non_privileged() -> Self {
        Self::all() & !Self::privileged()
    }

    // Private function because it's not really useful to expose this.
    fn privileged() -> Self {
        Self::GUILD_PRESENCES | Self::GUILD_MEMBERS
    }
}

/// Represents an op-code which can tell the gateway what operation a sent payload is for.
///
/// This should rarely be used as all gateway-related concepts are handled under the hood.
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, Serialize_repr, Deserialize_repr)]
#[non_exhaustive]
#[repr(u8)]
#[allow(clippy::use_self)]
pub enum OpCode {
    /// An event was dispatched.
    Dispatch = 0,

    /// Fired periodically by the client to keep the connection alive.
    Heartbeat = 1,

    /// Starts a new session during initial handshake.
    Identify = 2,

    /// Update the client presence/status.
    StatusUpdate = 3,

    /// Used to join, move, or leave voice channels.
    VoiceStateUpdate = 4,

    /// Resume a previous gateway session that was disconnected.
    Resume = 6,

    /// Used to tell clients to reconnect to the gateway.
    Reconnect = 7,

    /// Used to request guild members.
    RequestGuildMembers = 8,

    /// Used to notify clients that their session ID is invalid.
    InvalidSession = 9,

    /// Sent immediately after connection - contains heartbeat and server info.
    Hello = 10,

    /// Sent in response to receiving a heartbeat to acknowledge that it has been received.
    HeartbeatAck = 11,
}
