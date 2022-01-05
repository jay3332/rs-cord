use std::fmt::Display;

use crate::http::HttpClient;

/// An authenticated client which will be able to interact with the Discord API,
/// through both the REST and Gateway (websocket) APIs.
pub struct Client {
    /// The client for Discord's RESTful API.  
    pub http: HttpClient,

    /// The client for Discord's Gateway, or websocket API.
    pub gateway: (),
}

impl Client {
    /// Create a new client given an authentication token.
    pub fn from_token(token: impl Display) -> Self {
        Self {
            http: HttpClient::new_with_token(token.to_string()),
            gateway: (),
        }
    }
}
