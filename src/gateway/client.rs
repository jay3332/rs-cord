use super::GatewayError;
use crate::http::HttpClient;
use crate::internal::prelude::*;
use crate::types::gateway::{GetGatewayBotData, WsInboundEvent};
use crate::Intents;

use super::WsStream;

use flate2::read::ZlibDecoder;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;

use tokio_tungstenite::connect_async_with_config;
use tokio_tungstenite::tungstenite::protocol::{Message, WebSocketConfig};

use std::sync::Arc;
use std::time::Instant;

#[derive(Debug)]
pub struct Gateway {
    pub(crate) http: Arc<HttpClient>,
    pub(crate) intents: Intents,
    pub info: GetGatewayBotData,
    pub stream: WsStream,
    latency: Option<f64>,

    pub(crate) alive_since: Option<Instant>,
    pub(crate) heartbeat_interval: Option<u16>,
    pub(crate) last_heartbeat: Option<Instant>,
    pub(crate) session_id: Option<String>,
    pub(crate) seq: Option<u64>,
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
            latency: None,
            alive_since: None,
            heartbeat_interval: None,
            last_heartbeat: None,
            session_id: None,
            seq: None,
        })
    }

    #[allow(unreachable_code)]
    #[allow(unreachable_patterns)]
    pub async fn connect(&mut self, _reconnect: bool) -> Result<()> {
        self.alive_since = Some(Instant::now());

        match serde_json::from_value::<WsInboundEvent>(
            self.recv_json().await?.ok_or(GatewayError::NoHello)?,
        )? {
            WsInboundEvent::Hello(heartbeat_interval) => {
                self.heartbeat_interval = Some(heartbeat_interval);
            }
            _ => return Err(GatewayError::NoHello.into()),
        }

        self.identify().await?;
        self.start_recv().await?;

        Ok(())
    }

    pub async fn start_recv(&mut self) -> Result<()> {
        while let Some(msg) = self.recv_json().await? {
            self.try_heartbeat().await?;

            match serde_json::from_value::<WsInboundEvent>(msg)? {
                WsInboundEvent::HeartbeatAck => {
                    self.latency = Some(self.last_heartbeat.unwrap().elapsed().as_secs_f64());
                }
                _ => {}
            }
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

    pub async fn try_heartbeat(&mut self) -> Result<()> {
        if self.heartbeat_interval.is_none() {
            // Allow 15 seconds to receive the heartbeat interval
            if self.alive_since.unwrap().elapsed().as_secs() < 15 {
                return Ok(());
            }

            return Err(GatewayError::NoHello.into());
        }

        if let Some(h) = &self.last_heartbeat {
            // If it isn't time to heartbeat, then... don't heartbeat.
            if h.elapsed().as_secs() <= self.heartbeat_interval.unwrap() as u64 {
                return Ok(());
            }
        }

        self.last_heartbeat = Some(Instant::now());
        self.heartbeat().await
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
