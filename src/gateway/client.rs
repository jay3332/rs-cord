use crate::http::HttpClient;
use crate::internal::prelude::*;
use crate::types::gateway::{GetGatewayBotData, HelloData};
use crate::Intents;

use super::WsStream;

use flate2::read::ZlibDecoder;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};

use tokio_tungstenite::connect_async_with_config;
use tokio_tungstenite::tungstenite::protocol::{Message, WebSocketConfig};

use std::env::consts;
use std::sync::Arc;

pub struct Gateway {
    http: Arc<HttpClient>,
    info: GetGatewayBotData,
    pub(crate) intents: Intents,
    pub stream: WsStream,
    heartbeat_interval: u16,
}

impl Gateway {
    pub async fn new(http: Arc<HttpClient>, intents: Intents) -> Result<Self> {
        if http.token.is_none() {
            return Err(Error::from(
                "A token is required in order to initiate the gateway.",
            ));
        }

        let info = http.get_gateway_bot().await.map_err(Error::from)?;

        let (stream, _) = connect_async_with_config(
            (&info).url.clone(),
            Some(WebSocketConfig {
                max_send_queue: None,
                max_message_size: None,
                max_frame_size: None,
                accept_unmasked_frames: false,
            }),
        )
        .await?;

        Ok(Self {
            http,
            info: info.clone(),
            intents,
            stream,
            heartbeat_interval: None
        })
    }

    pub async fn connect(&mut self, _reconnect: bool) -> Result<()> {
        let hello: HelloData = self.recv_json().map(serde_json::from_value).map_err(Error::from).await?;
        self.heartbeat_interval = hello.heartbeat_interval;

        // start heartbeat

        self.identify().await?;

        loop {
            let _ = self.recv_json().await;
            // handle events
        }

        Ok(())
    }

    pub async fn recv_json(&mut self) -> Result<Option<Value>> {
        handle_ws_message(self.stream.next().await.transpose()?)
    }

    pub async fn send_json(&mut self, payload: &Value) -> Result<()> {
        Ok(serde_json::to_string(payload)
            .map(Message::Text)
            .map_err(Error::from)
            .map(|m| self.stream.send(m))?
            .await?)
    }
}

#[inline]
pub fn handle_ws_message(message: Option<Message>) -> Result<Option<Value>> {
    Ok(match message {
        Some(Message::Binary(bytes)) => serde_json::from_reader(ZlibDecoder::new(&bytes[..]))
            .map(Some)
            .map_err(Error::from)?,
        Some(Message::Text(text)) => serde_json::from_str(&text).map(Some).map_err(Error::from)?,
        Some(Message::Close(Some(_))) => None, // TODO: handle close
        _ => None,
    })
}
