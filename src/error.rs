use std::{error::Error as StdError, result::Result as StdResult};

pub type Result<T> = StdResult<T, Error>;
pub type ThreadSafeError = Box<dyn StdError + Send + Sync>;
pub type ThreadSafeResult<T> = StdResult<T, ThreadSafeError>;

use tokio_tungstenite::tungstenite::error::Error as TungsteniteError;

#[derive(Debug)]
pub enum Error {
    Generic(ThreadSafeError),
    Serde(serde_json::Error),
    Tungstenite(TungsteniteError),
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
