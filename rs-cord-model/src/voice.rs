use crate::member::MemberData;
use crate::Snowflake;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceStateData {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub member: Option<MemberData>,
    pub session_id: Snowflake,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub self_video: bool,
    pub suppress: bool,
    pub request_to_speak_timestamp: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StageInstanceData {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub topic: String,
    pub privacy_level: u8,
    pub discoverable_disabled: bool,
}
