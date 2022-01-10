use super::user::UserData;
use super::Snowflake;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MemberData {
    pub user: Option<UserData>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<Snowflake>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    pub communication_disabled_until: Option<String>,
    pub guild_id: Option<Snowflake>, // this extra field is present in gateway events ONLY
}
