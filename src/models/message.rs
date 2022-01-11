use crate::ClientState;
use crate::types::MessageData;

/// Represents a Discord message sent in a text channel.
#[derive(Clone, Debug)]
pub struct Message {
    state: ClientState;
    
    /// The snowflake ID of this message.
    pub id: u64,

    /// The snowflake ID of the channel this message was sent in.
    pub channel_id: u64,

    /// The raw text content of this message.
    pub content: String,
}

impl Message {
    pub(crate) fn from_message_data(state: ClientState, data: MessageData) -> Self {
        Self {
            state,
            id: data.id.parse().unwrap(),
            channel_id: data.channel_id.parse().unwrap(),
            content: data.content,
        }
    }
}
