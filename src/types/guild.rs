use super::Snowflake;
use super::channel::ChannelData;
use super::emoji::EmojiData;
use super::member::MemberData;
use super::presence::PresenceUpdateData;
use super::role::RoleData;
use super::sticker::StickerData;
use super::user::UserData;
use super::voice::{VoiceStateData, StageInstanceData};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildData {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: Snowflake,
    pub permissions: Option<u64>,
    pub region: Option<String>,
    pub afk_channel_id: Option<Snowflake>,
    pub afk_timeout: u64,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<Snowflake>,
    pub verification_level: u64,
    pub default_message_notifications: u64,
    pub explicit_content_filter: u64,
    pub roles: Vec<RoleData>,
    pub emojis: Vec<EmojiData>,
    pub features: Vec<String>,
    pub mfa_level: u64,
    pub application_id: Option<Snowflake>,
    pub system_channel_id: Option<Snowflake>,
    pub system_channel_flags: u64,
    pub rules_channel_id: Option<Snowflake>,
    pub joined_at: Option<String>,
    pub large: Option<bool>,
    pub unavailable: Option<bool>,
    pub member_count: Option<u64>,
    pub voice_states: Option<VoiceStateData>,
    pub members: Option<Vec<MemberData>>,
    pub channels: Option<Vec<ChannelData>>,
    pub threads: Option<Vec<ChannelData>>,
    pub presences: Option<Vec<PresenceUpdateData>>,
    pub max_presences: Option<u64>,
    pub max_members: Option<u64>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: u64,
    pub premium_subscription_count: Option<u64>,
    pub preferred_locale: String,
    pub public_updates_channel_id: Option<Snowflake>,
    pub max_video_channel_users: Option<u64>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub welcome_screen: Option<WelcomeScreenData>,
    pub nsfw_level: u8,
    pub stage_instances: Option<Vec<StageInstanceData>>,
    pub stickers: Option<Vec<StickerData>>,
    pub guild_scheduled_events: Option<Vec<ScheduledEventData>>,
    pub premium_progress_bar_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WelcomeScreenData {
    pub description: Option<String>,
    pub welcome_channels: Vec<WelcomeScreenChannelData>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WelcomeScreenChannelData {
    pub channel_id: Snowflake,
    pub description: String,
    pub emoji_id: Option<Snowflake>,
    pub emoji_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScheduledEventData {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub channel_id: Option<Snowflake>,
    pub creator_id: Option<Snowflake>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_start_time: String,
    pub scheduled_end_time: Option<String>,
    pub privacy_level: u8,
    pub status: u8,
    pub entity_type: u8,
    pub entity_id: Option<Snowflake>,
    pub entity_metadata: Option<ScheduledEventMetadata>,
    pub creator: Option<UserData>,
    pub user_count: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScheduledEventMetadata {
    pub location: Option<String>,
}
