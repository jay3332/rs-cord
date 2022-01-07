use crate::impl_created_at;
use crate::types::user::UserData;

/// Represents a Discord user.
pub struct User {
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
    pub(crate) fn from_user_data(data: UserData) -> Self {
        Self {
            id: data.id.parse().unwrap(),
            name: data.username,
            discriminator: data.discriminator,
            avatar_hash: data.avatar,
            bot: data.bot,
        }
    }

    /// The Discord tag of this user, formatted in `username#discriminator` format.
    pub fn tag(&self) -> String {
        format!("{}#{}", self.name, self.discriminator)
    }

    /// Whether or not this user is a bot account.
    pub fn is_bot(&self) -> bool {
        self.bot
    }
}

impl_created_at!(User);