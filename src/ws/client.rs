use super::WsStream;

use std::env::consts;

use tokio_tungstenite::{connect_async_with_config, tungstenite::protocol::{Message, WebSocketConfig}};
use url::Url;

use flate2::read::ZlibDecoder;

use futures_util::{SinkExt, StreamExt};

use serde_json::{json, Value};

use crate::internal::prelude::*;

pub struct Gateway {
    pub stream: WsStream,
    pub url: Url,
    pub token: String,
}

impl Gateway {
    pub async fn new(url: &str, token: String) -> Result<Self> {
        let url = Url::parse(url).unwrap_or_else(|e| panic!("Failed to parse url: {}", e));

        let stream = connect_async_with_config(url, WebSocketConfig {
            max_send_queue: None,
            max_message_size: None,
            max_frame_size: None,
        }).await?;

        Ok(Ws { stream, url, token })
    }

    pub async fn connect(reconnect: bool) -> Result<()> {
        Ok(()) // TODO
    }

    pub async fn recv_json(&mut self) -> Result<Option<Value>> {
        handle_ws_message(self.stream.next().await)
    }

    pub async fn send_json(&mut self, payload: &Value) -> Result<()> {
        Ok(serde_json::to_string(payload)
            .map(Message::Text)
            .map_err(Error::from)
            .map(|m| self.stream.send(m))?
            .await?)
    }

    async fn identify(&mut self) -> Result<()> {
        Ok(self.send_json(
            &json!({
                "op": 2_u8,
                "d": {
                    "token": self.token,
                    "intents": self.intents.value,
                    "compress": true,
                    "large_threshold": 250_u8,
                    // shard
                    "presence": self.presence,
                    "properties": {
                        "$os": consts::OS,
                        "$browser": "rs-cord",
                        "$device": "rs-cord",
                    }
                }
            })
        ).await?)
    }
]

#[inline]
pub fn handle_ws_message(message: Option<Message>) -> Result<Option<Value>> {
    Ok(
        match message {
            Some(Message::Binary(bytes)) => {
                serde_json::from_reader(ZlibDecoder::new(&bytes[..])).map(Some).map_err(Error::from)?
            },
            Some(Message::Text(text)) => {
                serde_json::from_str(&text).map(Some).map_err(Error::from)?
            },
            Some(Message::Close(Some(frame))) => None, // TODO: handle close
            _ => None,
        }
    )
}
