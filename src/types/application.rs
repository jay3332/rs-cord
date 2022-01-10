use super::user::UserData;
use super::Snowflake;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationData {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub rpc_origins: Option<Vec<String>>,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub owner: Option<UserData>,
    pub summary: String,
    pub verify_key: String,
    pub team: Option<TeamData>,
    pub guild_id: Option<Snowflake>,
    pub primary_sku_id: Option<Snowflake>,
    pub slug: Option<String>,
    pub cover_image: Option<String>,
    pub flags: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialApplicationData {
    pub id: Snowflake,
    pub flags: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamData {
    pub icon: Option<String>,
    pub id: Snowflake,
    pub members: Vec<TeamMemberData>,
    pub name: String,
    pub owner_user_id: Snowflake,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamMemberData {
    pub membership_state: u8,
    pub permissions: Vec<String>,  // always ["*"]
    pub team_id: Snowflake,
    pub user: UserData,
}
