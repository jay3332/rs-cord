use super::WsStream;

use tokio_tungstenite::{connect_async_with_config, tungstenite::protocol::{Message, WebSocketConfig}};

use url::Url;

use crate::internal::prelude::*;

pub struct Ws {
    pub stream: WsStream,
    pub url: Url,
    pub token: String,
}

impl Ws {
    pub async fn new(url: &str, token: String) -> Result<Ws> {
        let url = Url::parse(url).unwrap_or_else(|e| panic!("Failed to parse url: {}", e));

        let stream = connect_async_with_config(url, WebSocketConfig {
            max_send_queue: None,
            max_message_size: None,
            max_frame_size: None,
        }).await?;

        Ok(Ws { stream, url, token })
    }
}