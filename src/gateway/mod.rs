mod event;

use std::sync::Arc;
use tokio::time::Duration;
use tokio_tungstenite::connect_async_with_config;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use futures_util::stream::{SplitSink, SplitStream, SinkExt, StreamExt};
use crate::Error;
use crate::http::Http;
use crate::error::Result;
use crate::gateway::event::WsInboundEvents;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GatewayVersion {
    V9 = 9,
    V10 = 10,
}

impl From<u8> for GatewayVersion {
    fn from(v: u8) -> Self {
        match v {
            9 => Self::V9,
            10 => Self::V10,
            _ => panic!("unsupported gateway version v{}", v),
        }
    }
}

/// Represents a websocket client for Discord's gateway. Supports V9 and V10.
#[derive(Debug)]
pub struct Gateway {
    http: Arc<Http>,

    /// The token which is used to connect to the gateway.
    pub token: String,

    /// The Websocket URI of the gateway.
    pub url: String,

    /// The heartbeat in received by Discord. The client will send a heartbeat
    /// to Discord's gateway at this interval to keep the websocket connection
    /// alive.
    pub heartbeat_interval: u32,

    /// The session ID initally received by Discord. Used for reconnection purposes.
    pub session_id: Option<String>,

    /// The last sequence ID received by Discord, used during resumption
    /// which allows Discord to know the last event this client has received.
    pub seq: Option<u64>,

    /// The gateway version. Supports v9 and v10, and is represented as a
    /// [`GatewayVersion`] enum.
    pub version: GatewayVersion,

    sender: SplitStream,
    receiver: SplitSink,
}

impl Gateway {
    const RECEIVE_TIMEOUT: Duration = Duration::from_millis(500);

    pub async fn new(http: Arc<Http>) -> Result<Self> {
        let (stream, _) = connect_async_with_config(
            todo!(),
            Some(WebSocketConfig {
                max_send_queue: None,
                max_message_size: None,
                max_frame_size: None,
                accept_unmasked_frames: false
            })
        ).await?;

        Ok(Self {
            http,
        })
    }

    pub async fn prepare(&mut self) -> Result<()> {
        self.        let (sender, receiver) = stream.split();

        self.sendeevenevent.rsevent.tswtfevent.rscrevnevent.rsr = sender;
        self.receiver = receiver;

        Ok(())
    }

    async fn poll_event(&mut self) -> Result<Option<WsInboundEvents>> {
        parse_event(match tokio::time::timeout(Self::RECEIVE_TIMEOUT, self.receiver.next()).await {
            Ok(Some(Ok(m))) => m,
            Ok(Some(Err(e))) => return Err(e),
            _ => return Err(Error::Library("Timeout while polling for event".to_string()))
        })
    }
}
