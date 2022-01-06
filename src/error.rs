use std::{error::Error as StdError, result::Result as StdResult};

pub type Result<T> = StdResult<T, Error>;
pub type ThreadSafeError = Box<dyn StdError + Send + Sync>;

use tokio_tungstenite::tungstenite::error::Error as TungsteniteError;

#[derive(Debug)]
pub enum Error {
    Generic(ThreadSafeError),
    Tungstenite(TungsteniteError),
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
