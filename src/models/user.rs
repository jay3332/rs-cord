use crate::{ClientState, impl_created_at};
use crate::types::user::UserData;

/// Represents a Discord user.
#[derive(Clone, Debug)]
pub struct User {
    state: ClientState,

    /// The snowflake ID of this user.
    pub id: u64,

    /// The username of this user.
    pub name: String,

    /// The discriminator of this user. Note that this is always a string, not an integer.
    pub discriminator: String,

    /// The avatar hash of this user if this user has an avatar.
    ///
    /// # See
    /// [`User::avatar`]
    /// [`User::default_avatar`]
    pub avatar_hash: Option<String>,

    /// Whether or not this user is a bot account.
    pub bot: bool,
}

impl User {
    pub(crate) fn from_user_data(state: ClientState, data: UserData) -> Self {
        Self {
            state,
            id: data.id.parse().unwrap(),
            name: data.username,
            discriminator: data.discriminator,
            avatar_hash: data.avatar,
            bot: data.bot.unwrap_or(false),
        }
    }

    /// Returns the Discord tag of this user, formatted in `username#discriminator` format.
    #[must_use]
    pub fn tag(&self) -> String {
        format!("{}#{}", self.name, self.discriminator)
    }

    /// The mention of this user, which if allowed, pings them.
    #[must_use]
    pub fn mention(&self) -> String {
        format!("<@{}>", self.id)
    }

    /// Whether or not this user is a bot account.
    #[must_use]
    pub fn is_bot(&self) -> bool {
        self.bot
    }
}

impl_created_at!(User);
