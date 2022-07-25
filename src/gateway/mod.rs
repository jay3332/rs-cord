use crate::error::{Result, WebsocketError};
use crate::gateway::event::WsInboundEvents;
use crate::http::Http;
use crate::Error;
use rs_cord_model::gateway::inbound::WsInboundEvents;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::time::Duration;
use tokio_tungstenite::connect_async_with_config;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use tokio_tungstenite::tungstenite::Error as WsError;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use flate2::read::ZlibDecoder;

type WebsocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

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

impl Default for GatewayVersion {
    fn default() -> Self {
        Self::V10
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
    pub heartbeat_interval: Option<u32>,

    /// The session ID initally received by Discord. Used for reconnection purposes.
    pub session_id: Option<String>,

    /// The last sequence ID received by Discord, used during resumption
    /// which allows Discord to know the last event this client has received.
    pub seq: Option<u64>,

    /// The gateway version. Supports v9 and v10, and is represented as a
    /// [`GatewayVersion`] enum.
    pub version: GatewayVersion,

    decoder: ZlibDecoder<Vec<u8>>,
    sender: SplitStream<WebsocketStream>,
    receiver: SplitSink<WebsocketStream, std::result::Result<Message, WsError>>,
}

impl Gateway {
    const RECEIVE_TIMEOUT: Duration = Duration::from_millis(500);

    pub async fn new(http: Arc<Http>) -> Result<Self> {
        let url = "";

        let (stream, _) = connect_async_with_config(
            url,
            Some(WebSocketConfig {
                max_send_queue: None,
                max_message_size: None,
                max_frame_size: None,
                accept_unmasked_frames: false,
            }),
        )
        .await?;

        let (sender, receiver) = stream.split();

        Ok(Self {
            http,
            token: http.token,
            url,
            heartbeat_interval: None,
            session_id: None,
            seq: None,
            version: GatewayVersion::V10,
            decoder: ZlibDecoder::new(Vec::new()),
            sender,
            receiver,
        })
    }

    pub async fn prepare(&mut self) -> Result<()> {
        let hello = self
            .poll_event()
            .await
            .map_err(|_| WebsocketError::NoHello)?
            .ok_or(WebsocketError::NoHello)?;

        Ok(())
    }

    async fn poll_event(&mut self) -> Result<Option<WsInboundEvents>> {
        self.parse_event(
            match tokio::time::timeout(Self::RECEIVE_TIMEOUT, self.receiver.next()).await {
                Ok(Some(Ok(m))) => m,
                Ok(Some(Err(e))) => return Err(e),
                _ => {
                    return Err(Error::Library(
                        "Timeout while polling for event".to_string(),
                    ))
                }
            },
        )
    }

    fn parse_event(&mut self, message: Message) -> Result<Option<WsInboundEvents>> {
        todo!()
    }
}
