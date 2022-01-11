use super::application::ApplicationData;
use super::channel::ChannelData;
use super::guild::{GuildData, ScheduledEventData};
use super::member::MemberData;
use super::user::UserData;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InviteData {
    pub code: String,
    pub guild: Option<GuildData>,
    pub channel: Option<ChannelData>,
    pub inviter: Option<UserData>,
    pub target_type: Option<u8>,
    pub target_user: Option<UserData>,
    pub target_application: Option<ApplicationData>,
    pub approximate_presence_count: Option<u64>,
    pub approximate_member_count: Option<u64>,
    pub expires_at: Option<String>,
    pub stage_instance: Option<InviteStageInstanceData>,
    pub guild_scheduled_event: Option<ScheduledEventData>,

    // The following are part of "invite metadata"
    pub uses: Option<u64>,
    pub max_uses: Option<u64>,
    pub max_age: Option<u64>,
    pub temporary: Option<bool>,
    pub created_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InviteStageInstanceData {
    pub members: Vec<MemberData>,
    pub participant_count: u32,
    pub speaker_count: u32,
    pub topic: String,
}
