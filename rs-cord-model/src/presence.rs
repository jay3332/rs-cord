use crate::emoji::ActivityEmojiData;
use crate::user::UserData;
use crate::Snowflake;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PresenceUpdateData {
    pub user: UserData,
    pub guild_id: Snowflake,
    pub status: String,
    pub activities: Vec<ActivityData>,
    pub client_status: ClientStatusData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityData {
    pub name: String,
    pub r#type: u8,
    pub url: Option<String>,
    pub created_at: u64, // unix timestamp
    pub timestamps: Option<ActivityTimestampsData>,
    pub application_id: Option<Snowflake>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<ActivityEmojiData>,
    pub party: Option<ActivityPartyData>,
    pub assets: Option<ActivityAssetsData>,
    pub secrets: Option<ActivitySecretsData>,
    pub instance: Option<bool>,
    pub flags: Option<u64>,
    pub buttoms: Option<Vec<ActivityButtonData>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityTimestampsData {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityPartyData {
    pub id: Option<String>, // NOT a snowflake
    pub size: Option<[u32; 2]>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityAssetsData {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivitySecretsData {
    pub join: Option<String>,
    pub spectate: Option<String>,
    pub r#match: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityButtonData {
    pub label: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientStatusData {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>,
}
