use crate::{Asset, ClientState, impl_created_at};
use crate::types::user::UserData;

use bitflags::bitflags;

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

    /// The banner hash of this user's banner if this user has a banner.
    /// 
    /// # See
    /// - [`User::banner`]
    pub banner_hash: Option<String>,

    /// Whether or not this user is a bot account.
    pub bot: bool,

    /// Whether or not this user is a system account.
    pub system: bool,

    /// The special flags of this user which are exposed to the client.
    pub flags: UserFlags,
}

impl User {
    pub(crate) fn from_user_data(state: ClientState, data: UserData) -> Self {
        Self {
            state,
            id: data.id.parse().unwrap(),
            name: data.username,
            discriminator: data.discriminator,
            avatar_hash: data.avatar,
            banner_hash: data.banner,
            bot: data.bot.unwrap_or(false),
            system: data.system.unwrap_or(false),
            flags: UserFlags::from_bits_truncate(
                data.flags.unwrap_or(data.public_flags.unwrap_or(0))
            ),
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

    /// Whether or not this user is a system account. (E.g. Represents Discord officially)
    #[must_use]
    pub fn is_system(&self) -> bool {
        self.system
    }

    /// Whether or not this user is the client.
    #[must_use]
    pub fn is_me(&self) -> bool {
        self.id == self.state.user().id
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

    /// The banner that this user has. If this user does not have a banner, this will be [`None`].
    #[must_use]
    pub fn banner(&self) -> Option<Asset> {
        self.banner_hash.clone().map(|b| Asset::new(self.state.clone(), format!("banners/{}/{}", self.id, b), false))
    }

    /// The hypesquad house of this user.
    #[must_use]
    pub fn hypesquad_house(&self) -> Option<HypesquadHouse> {
        let flags = self.flags;

        if flags.contains(UserFlags::HYPESQUAD_ONLINE_HOUSE_1) {
            Some(HypesquadHouse::Bravery)
        }
        else if flags.contains(UserFlags::HYPESQUAD_ONLINE_HOUSE_2) {
            Some(HypesquadHouse::Brilliance)
        }
        else if flags.contains(UserFlags::HYPESQUAD_ONLINE_HOUSE_3) {
            Some(HypesquadHouse::Balance)
        }
        else {
            None
        }
    }
}

impl_created_at!(User);

bitflags! {
    /// A set of bitflags representing special flags of a user.
    /// 
    /// There are many utility methods on the [`User`] model that wrap around these flags,
    /// although these are perfectly fine to be used.
    pub struct UserFlags: u32 {
        /// This user is a Discord employee.
        const STAFF = 1 << 0;
        
        /// This user is a partnered server owner.
        const PARTNER = 1 << 1;

        /// This user is a HypeSquad events coordinator.
        const HYPESQUAD = 1 << 2;

        /// This user is a level 1 bug hunter.
        const BUGHUNTER_LEVEL_1 = 1 << 3;

        /// This user is a member of Hypesquad Bravery.
        /// 
        /// # See
        /// - [`User::hypesquad_house`]
        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;

        /// This user is a member of Hypesquad Brilliance.
        /// 
        /// # See
        /// - [`User::hypesquad_house`]
        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;

        /// This user is a member of Hypesquad Balance.
        /// 
        /// # See
        /// - [`User::hypesquad_house`]
        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;

        /// This user is an early Nitro supporter.
        const PREMIUM_EARLY_SUPPORTER = 1 << 9;

        /// This user is a team.
        const TEAM_PSEUDO_USER = 1 << 10;

        /// This user is a level 2 bug hunter.
        const BUG_HUNTER_LEVEL_2 = 1 << 14;

        /// This bot is verified.
        const VERIFIED_BOT = 1 << 16;

        /// This user is an Early Verified Bot Developer.
        const VERIFIED_DEVELOPER = 1 << 17;

        /// This user is a Discord Certified Moderator.
        const CERTIFIED_MODERATOR = 1 << 18;

        /// This bot only uses HTTP interactions and is shown in the online member list.
        const BOT_HTTP_INTERACTIONS = 1 << 19;
    }
}

/// Represents a user's Hypesquad house.
pub enum HypesquadHouse {
    /// Hypesquad Bravery.
    Bravery,

    /// Hypesquad Brilliance.
    Brilliance,

    /// Hypesquad Balance.
    Balance,
}
