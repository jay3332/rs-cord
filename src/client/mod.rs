use std::fmt::Display;

use crate::http::HttpClient;

/// An authenticated client which will be able to interact with the Discord API,
/// through both the REST and Gateway (websocket) APIs.
pub struct Client {
    /// The client for Discord's RESTful API.  
    pub http: Option<HttpClient>,

    /// The client for Discord's Gateway, or websocket API.
    pub gateway: (),

    /// The authentication token to be used.
    token: Option<String>,
}

impl Client {
    /// Create a client with no configurations.
    /// You must eventually provide a token in order to start the client.
    ///
    /// # See
    /// - [`Client::new_with_token`]
    /// - [`Client::with_token`]
    pub fn new() -> Self {
        Self {
            http: None,
            gateway: (),
            token: None,
        }
    }

    /// Create a new client given an authentication token.
    pub fn new_with_token(token: impl Display) -> Self {
        let mut client = Self::new().with_token(token);
        client.init_http();

        client
    }

    /// Set the authentication token to be used.
    pub fn with_token(mut self, token: impl Display) -> Self {
        self.token = Some(token.to_string());

        self
    }

    fn init_http(&mut self) {
        if self.http.is_some() {
            return;
        }

        self.http = Some(HttpClient::new_with_token(
            self.token
                .clone()
                .expect("An authentication token must be provided."),
        ));
    }
}
