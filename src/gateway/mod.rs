mod client;
mod error;
mod outbound_events;

use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub use client::Gateway;
pub use error::GatewayError;
