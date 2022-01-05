use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

use tokio_tungstenite::tungstenite::error::Error as TungsteniteError;

#[derive(debug)]
pub enum Error {
    Tungstenite(TungsteniteError),
}

impl From<TungsteniteError> for Error {
    fn from(e: TungsteniteError) -> Error {
        Error::Tungstenite(e)
    }
}
