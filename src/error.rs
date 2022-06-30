use thiserror::Error as _Error;
use tokio_tungstenite::tungstenite::Error as TungsteniteError;
use reqwest::Error as ReqwestError;

#[derive(Debug, _Error)]
pub enum Error {
    #[error("WebSocket error: `{0}`")]
    WebSocket(#[from] TungsteniteError),

    #[error("HTTP Error: `{0}`")]
    Http(#[from] ReqwestError),

    #[error("rs-cord error: `{0}`")]
    Library(String),
}

pub type Result<T> = std::result::Result<T, Error>;
