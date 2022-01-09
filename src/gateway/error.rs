use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

use tokio_tungstenite::tungstenite::protocol::frame::CloseFrame;

#[derive(Clone, Debug)]
pub enum GatewayError {
    NoSessionId,
    NoHello,
    Disconnected(Option<CloseFrame<'static>>),
}

impl Display for GatewayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            GatewayError::NoSessionId => f.write_str("No session id found when needed"),
            GatewayError::NoHello => f.write_str("Didn't receive hello event"),
            GatewayError::Disconnected(_) => {
                f.write_str("Disconnected from gateway, this is usually handled internally.")
            }
        }
    }
}

impl StdError for GatewayError {}
