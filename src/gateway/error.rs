use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

use tokio_tungstenite::tungstenite::protocol::frame::CloseFrame;

#[derive(Clone, Debug)]
pub enum GatewayError {
    NoSessionId,
    NoHello,
}

impl Display for GatewayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            GatewayError::NoSessionId => f.write_str("No session id found when needed"),
            GatewayError::NoHello => f.write_str("Didn't receive hello event"),
        }
    }
}

impl StdError for GatewayError {}
