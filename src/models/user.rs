use crate::{Asset, ClientState, impl_created_at};
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
    /// - [`User::avatar`]
    /// - [`User::default_avatar`]
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

    /// The default avatar of this user if this user does not have an avatar. This may not be their actual avatar.
    /// 
    /// # See
    /// [`User::avatar`]
    #[must_use]
    pub fn default_avatar(&self) -> Asset {
        Asset::new(self.state.clone(), format!("embed/avatars/{}", self.discriminator.parse::<u32>().unwrap() % 5), false)
    }

    /// The avatar of this user. If this user does not have an avatar, this will default to their [`default_avatar`][`User::default_avatar`].
    #[must_use]
    pub fn avatar(&self) -> Asset {
        if let Some(hash) = self.avatar_hash.clone() {
            Asset::new(self.state.clone(), format!("avatars/{}/{}", self.id, hash), hash.starts_with("a_"))
        } else {
            self.default_avatar()
        }
    }
}

impl_created_at!(User);
