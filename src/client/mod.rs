use std::fmt::Display;
use std::sync::Arc;

use crate::Intents;
use crate::http::HttpClient;
use crate::ws::Gateway;

/// An authenticated client which will be able to interact with the Discord API,
/// through both the REST and Gateway (websocket) APIs.
pub struct Client {
    /// The client for Discord's RESTful API.  
    pub http: Option<Arc<HttpClient>>,

    /// The client for Discord's Gateway, or websocket API.
    pub gateway: Option<Gateway>,

    /// The stored intent flags to use when connecting to the gateway.
    /// 
    /// # See
    /// - [`intents!`]
    /// - [`Intents`]
    pub intents: Intents,

    /// The authentication token to be used.
    pub(crate) token: Option<String>,
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
            gateway: None,
            intents: Intents::non_privileged(),
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
    /// 
    /// This method is chainable and modifies in place.
    pub fn with_token(mut self, token: impl Display) -> Self {
        self.token = Some(token.to_string());

        self
    }

    /// Sets the gateway intent flags to be used.
    /// 
    /// This method is chainable and modifies in place.
    pub fn with_intents(mut self, intents: Intents) -> Self {
        self.intents = intents;
        
        self
    }

    fn init_http(&mut self) {
        if self.http.is_some() {
            return;
        }

        self.http = Some(
            Arc::new(HttpClient::new_with_token(self.token.clone().expect("An authentication token must be provided.")))
        );
    }

    async fn init_gateway(&mut self) -> crate::error::Result<()> {
        if self.gateway.is_some() {
            return Ok(());
        }

        self.gateway = Some(Gateway::new(self.http.clone().unwrap(), self.intents).await?);

        Ok(())
    }
}
