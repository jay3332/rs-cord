use super::channel::ChannelData;
use super::emoji::PartialEmojiData;
use super::member::MemberData;
use super::presence::ActivityData;
use super::sticker::{StickerData, StickerItemData};
use super::user::UserData;
use super::Snowflake;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageData {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub author: UserData,
    pub member: Option<MemberData>,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<UserData>,
    pub mention_roles: Vec<Snowflake>,
    pub mention_channels: Option<Vec<ChannelMentionData>>,
    pub attachments: Vec<AttachmentData>,
    pub embeds: Vec<EmbedData>,
    pub reactions: Option<Vec<ReactionData>>,
    pub nonce: Option<Snowflake>,
    pub pinned: bool,
    pub webhook_id: Option<Snowflake>,
    pub r#type: u8,
    pub activity: Option<ActivityData>,
    pub application: Option<()>, // TODO (application object)
    pub application_id: Option<Snowflake>,
    pub message_reference: Option<MessageReferenceData>,
    pub flags: Option<u64>,
    pub referenced_message: Option<Box<MessageData>>,
    pub interaction: Option<()>, // TODO (message interaction object)
    pub thread: Option<ChannelData>,
    pub components: Option<Vec<()>>, // TODO (components)
    pub sticker_items: Option<Vec<StickerItemData>>,
    pub stickers: Option<Vec<StickerData>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChannelMentionData {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub r#type: u8,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttachmentData {
    pub id: Snowflake,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: u64,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub ephemeral: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedData {
    pub title: Option<String>,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<u64>,
    pub footer: Option<EmbedFooterData>,
    pub image: Option<EmbedImageData>,
    pub thumbnail: Option<EmbedThumbnailData>,
    pub video: Option<EmbedVideoData>,
    pub provider: Option<EmbedProviderData>,
    pub author: Option<EmbedAuthorData>,
    pub fields: Option<Vec<EmbedFieldData>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedFooterData {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedImageData {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedThumbnailData {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedVideoData {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedProviderData {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedAuthorData {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedFieldData {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReactionData {
    pub count: u64,
    pub me: bool,
    pub emoji: PartialEmojiData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageReferenceData {
    pub message_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub fail_if_not_exists: Option<bool>,
}
