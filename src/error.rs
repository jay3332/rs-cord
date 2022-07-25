use reqwest::Error as ReqwestError;
use thiserror::Error as _Error;
use tokio_tungstenite::tungstenite::Error as TungsteniteError;

#[derive(Debug, _Error)]
pub enum WebsocketError {
    #[error("Error from tungstenite: `{0}`")]
    Tungstenite(#[from] TungsteniteError),
    #[error("Didn't receive `HELLO` event")]
    NoHello,
}

#[derive(Debug, _Error)]
pub enum Error {
    #[error("WebSocket error: `{0}`")]
    WebSocket(#[from] WebsocketError),

    #[error("HTTP Error: `{0}`")]
    Http(#[from] ReqwestError),

    #[error("rs-cord error: `{0}`")]
    Library(String),
}

pub type Result<T> = std::result::Result<T, Error>;
