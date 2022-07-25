use crate::application::ApplicationData;
use crate::channel::ChannelData;
use crate::emoji::PartialEmojiData;
use crate::member::MemberData;
use crate::presence::ActivityData;
use crate::sticker::{StickerData, StickerItemData};
use crate::user::UserData;
use crate::Snowflake;

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
    pub application: Option<ApplicationData>,
    pub application_id: Option<Snowflake>,
    pub message_reference: Option<MessageReferenceData>,
    pub flags: Option<u64>,
    pub referenced_message: Option<Box<MessageData>>,
    pub interaction: Option<MessageInteractionData>,
    pub thread: Option<ChannelData>,
    pub components: Option<Vec<ComponentData>>,
    pub sticker_items: Option<Vec<StickerItemData>>,
    pub stickers: Option<Vec<StickerData>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageUpdateData {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub author: Option<UserData>,
    pub member: Option<MemberData>,
    pub content: Option<String>,
    pub timestamp: Option<String>,
    pub edited_timestamp: Option<String>,
    pub tts: Option<bool>,
    pub mention_everyone: Option<bool>,
    pub mentions: Option<Vec<UserData>>,
    pub mention_roles: Option<Vec<Snowflake>>,
    pub mention_channels: Option<Vec<ChannelMentionData>>,
    pub attachments: Option<Vec<AttachmentData>>,
    pub embeds: Option<Vec<EmbedData>>,
    pub reactions: Option<Vec<ReactionData>>,
    pub nonce: Option<Snowflake>,
    pub pinned: Option<bool>,
    pub webhook_id: Option<Snowflake>,
    pub r#type: Option<u8>,
    pub activity: Option<ActivityData>,
    pub application: Option<ApplicationData>,
    pub application_id: Option<Snowflake>,
    pub message_reference: Option<MessageReferenceData>,
    pub flags: Option<u64>,
    pub referenced_message: Option<Box<MessageData>>,
    pub interaction: Option<MessageInteractionData>,
    pub thread: Option<ChannelData>,
    pub components: Option<Vec<ComponentData>>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageInteractionData {
    pub id: Snowflake,
    pub r#type: u8,
    pub name: String,
    pub user: UserData,
}

// TODO: Own component file?

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComponentData {
    pub r#type: u8,
    pub custom_id: Option<String>,
    pub disabled: Option<bool>,
    pub style: Option<u8>,
    pub label: Option<String>,
    pub emoji: Option<PartialEmojiData>,
    pub url: Option<String>,
    pub options: Option<Vec<SelectOptionData>>,
    pub placeholder: Option<String>,
    pub min_values: Option<u8>,
    pub max_values: Option<u8>,
    pub components: Vec<Box<ComponentData>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SelectOptionData {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<PartialEmojiData>,
    pub default: Option<bool>,
}
