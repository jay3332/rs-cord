pub mod state;

use std::fmt::Display;
use std::sync::Arc;

use crate::gateway::Gateway;
use crate::http::HttpClient;
use crate::{Intents, User};
pub use state::ClientState;

use tokio::sync::Mutex;

/// An authenticated client which will be able to interact with the Discord API,
/// through both the REST and Gateway (websocket) APIs.
pub struct Client {
    /// The client for Discord's RESTful API.  
    pub http: Option<Arc<HttpClient>>,

    /// The client for Discord's Gateway, or websocket API.
    pub gateway: Option<Arc<Mutex<Gateway>>>,

    /// The stored intent flags to use when connecting to the gateway.
    ///
    /// # See
    /// - [`intents!`]
    /// - [`Intents`]
    pub intents: Intents,

    /// The authentication token to be used.
    pub(crate) token: Option<String>,

    /// The current user that this client represents. Only applicable if the client has logged in.
    pub(crate) user: Option<User>,
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
            user: None,
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

    /// Retrieves the current client state.
    ///
    /// # Panics
    /// - The HTTP client is not initialized.
    /// - No token has been provided yet.
    pub fn state(&self) -> ClientState {
        ClientState {
            client: self,
            http: self.http.clone().expect("HTTP client is not initialized."),
            token: self.token.clone().expect("No token has been provided yet."),
        }
    }

    /// Starts the client by authenticating via HTTP and connecting via the gateway.
    ///
    /// # Panics
    /// - No authentication token has been provided yet.
    pub async fn start(&mut self) -> crate::error::Result<()> {
        self.init_http();
        self.init_gateway().await?;

        self.user = Some(User::from_user_data(
            self.http.as_ref().unwrap().get_self().await?,
        ));

        self.gateway
            .as_ref()
            .unwrap()
            .lock()
            .await
            .connect(false)
            .await
    }

    fn init_http(&mut self) {
        if self.http.is_some() {
            return;
        }

        self.http = Some(Arc::new(HttpClient::new_with_token(
            self.token
                .clone()
                .expect("An authentication token must be provided."),
        )));
    }

    async fn init_gateway(&mut self) -> crate::error::Result<()> {
        if self.gateway.is_some() {
            return Ok(());
        }

        self.gateway = Some(Arc::new(Mutex::new(
            Gateway::new(self.http.clone().unwrap(), self.intents).await?,
        )));

        Ok(())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}