use std::{error::Error as StdError, result::Result as StdResult};

pub type Result<T> = StdResult<T, Error>;
pub type ThreadSafeError = Box<dyn StdError + Send + Sync>;
pub type ThreadSafeResult<T> = StdResult<T, ThreadSafeError>;

use crate::gateway::GatewayError;
use crate::manager::NotFoundError;

use tokio_tungstenite::tungstenite::error::Error as TungsteniteError;

/// Represents an error that occurs in rs-cord.
#[derive(Debug)]
pub enum Error {
    /// A gateway-related error.
    Gateway(GatewayError),

    /// A generic, uncategorized error that does not fit in any other category.
    Generic(ThreadSafeError),

    /// Some item was not found in the cache.
    ///
    /// This is NOT to be confused with the `NotFound` HTTP error. This is for the internal cache, stored in memory.
    NotFound(NotFoundError),

    /// An error related to serializing/deserializing JSON data occured.
    Serde(serde_json::Error),

    /// An error related to [`tungstenite`] occured.
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

impl From<GatewayError> for Error {
    fn from(err: GatewayError) -> Error {
        Error::Gateway(err)
    }
}
