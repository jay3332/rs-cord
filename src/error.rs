use std::{error::Error as StdError, result::Result as StdResult};

pub type Result<T> = StdResult<T, Error>;
pub type ThreadSafeError = Box<dyn StdError + Send + Sync>;
pub type ThreadSafeResult<T> = StdResult<T, ThreadSafeError>;

use crate::gateway::GatewayError;

use tokio_tungstenite::tungstenite::error::Error as TungsteniteError;

#[derive(Debug)]
pub enum Error {
    Generic(ThreadSafeError),
    Serde(serde_json::Error),
    Tungstenite(TungsteniteError),
    Gateway(GatewayError),
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Error::Generic(ThreadSafeError::from(s))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde(err)
    }
}

impl From<ThreadSafeError> for Error {
    fn from(err: ThreadSafeError) -> Self {
        Error::Generic(err)
    }
}

impl From<TungsteniteError> for Error {
    fn from(err: TungsteniteError) -> Error {
        Error::Tungstenite(err)
    }
}

impl From<GatewayError> for Error {
    fn from(err: GatewayError) -> Error {
        Error::Gateway(err)
    }
}
