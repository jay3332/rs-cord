use super::Snowflake;
use super::user::UserPayload;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessagePayload {
    id: Snowflake,
    channel_id: Snowflake,
    guild_id: Option<Snowflake>,
    author: UserPayload,
    member: (),  // TODO
    content: String,
    timestamp: (),  // TODO (timestamp object)
    edited_timestamp: Option<()>,  // TODO ^
    tts: bool,
    mention_everyone: bool,
    mentions: Vec<UserPayload>,
    mention_roles: Vec<Snowflake>,
    mention_channels: Vec<()>,  // TODO (channel mention object)
    attachments: Vec<()>,  // TODO (attachment object)
    embeds: Vec<()>,  // TODO (embed object)
    reactions: Option<Vec<()>>,  // TODO (reaction object)
    nonce: Option<Snowflake>,
    pinned: bool,
    webhook_id: Option<Snowflake>,
    r#type: u8,  // TODO int enum for this?
    activity: Option<()>,  // TODO (activity object)
    application: Option<()>,  // TODO (application object)
    application_id: Option<Snowflake>,
    message_reference: Option<()>,  // TODO (message reference object)
    flags: Option<u64>,
    referenced_message: Option<Message>,
    interaction: Option<()>,  // TODO (message interaction object)
    thread: Option<()>,  // TODO (channel object)
    components: Option<Vec<()>>,  // TODO (components)
    sticker_items: Option<Vec<()>>,  // TODO (message sticker item object)
    stickers: Option<Vec<()>>,  // TODO (sticker object)
}
