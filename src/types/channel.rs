use super::Snowflake;
use super::user::UserData;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChannelData {
    pub id: Snowflake,
    pub r#type: u8,
    pub guild_id: Option<Snowflake>,
    pub position: Option<u64>,
    pub permission_overwrites: Option<Vec<OverwriteData>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<Snowflake>,
    pub bitrate: Option<u64>,
    pub user_limit: Option<u64>,
    pub rate_limit_per_user: Option<u64>,
    pub recipients: Option<Vec<UserData>>,
    pub icon: Option<String>,
    pub owner_id: Option<Snowflake>,
    pub application_id: Option<Snowflake>,
    pub parent_id: Option<Snowflake>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<u64>,
    pub message_count: Option<u64>,
    pub member_count: Option<u64>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMemberData>,
    pub default_auto_archive_duration: Option<u64>,
    pub permissions: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OverwriteData {
    pub id: Snowflake,
    pub r#type: u8,  // either 0 (role) or 1 (member)
    pub allow: u64,
    pub deny: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u64,
    pub archive_timestamp: String,
    pub locked: bool,
    pub invitable: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreadMemberData {
    pub id: Option<Snowflake>,
    pub user_id: Option<Snowflake>,
    pub join_timestamp: String,
    pub flags: u64,
}
