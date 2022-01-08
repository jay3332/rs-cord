use super::Snowflake;
use super::user::UserData;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StickerData {
    pub id: Snowflake,
    pub pack_id: Option<Snowflake>,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,  // comma-separated list
    pub asset: Option<String>,  // deprecated
    pub r#type: u8,
    pub format_type: u8,
    pub available: Option<bool>,
    pub guild_id: Option<Snowflake>,
    pub user: Option<UserData>,
    pub sort_value: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StickerItemData {
    pub id: Snowflake,
    pub pack_id: Snowflake,
    pub format_type: u8,
}
