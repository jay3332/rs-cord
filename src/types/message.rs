use super::user::UserData;
use super::Snowflake;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageData {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub author: UserData,
    pub member: (), // TODO
    pub content: String,
    pub timestamp: (),                // TODO (timestamp object)
    pub edited_timestamp: Option<()>, // TODO ^
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<UserData>,
    pub mention_roles: Vec<Snowflake>,
    pub mention_channels: Vec<()>,  // TODO (channel mention object)
    pub attachments: Vec<()>,       // TODO (attachment object)
    pub embeds: Vec<()>,            // TODO (embed object)
    pub reactions: Option<Vec<()>>, // TODO (reaction object)
    pub nonce: Option<Snowflake>,
    pub pinned: bool,
    pub webhook_id: Option<Snowflake>,
    pub r#type: u8,              // TODO int enum for this?
    pub activity: Option<()>,    // TODO (activity object)
    pub application: Option<()>, // TODO (application object)
    pub application_id: Option<Snowflake>,
    pub message_reference: Option<()>, // TODO (message reference object)
    pub flags: Option<u64>,
    pub referenced_message: Option<Box<MessageData>>,
    pub interaction: Option<()>,     // TODO (message interaction object)
    pub thread: Option<()>,          // TODO (channel object)
    pub components: Option<Vec<()>>, // TODO (components)
    pub sticker_items: Option<Vec<()>>, // TODO (message sticker item object)
    pub stickers: Option<Vec<()>>,   // TODO (sticker object)
}
