use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug)]
pub enum GatewayError {
    NoSessionId,
}

impl Display for GatewayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            GatewayError::NoSessionId => f.write_str("No session id found when needed"),
        }
    }
}

impl StdError for GatewayError {}
