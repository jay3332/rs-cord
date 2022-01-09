use std::sync::Arc;

use super::Client;
use crate::http::HttpClient;
use crate::User;

/// Represents the state of the client.
#[derive(Clone, Debug)]
pub struct ClientState {
    /// An [`Arc`][`std::sync::Arc`] reference to the client serving this state.
    pub client: Arc<Client>,

    /// The HTTP Client being used.
    pub http: Arc<HttpClient>,

    /// The current authentication token being used by the client.
    /// This should not be exposed to anyone.
    pub token: String,
}

impl ClientState {
    /// The user the client is logged in to.
    ///
    /// # Panics
    /// - The client is not logged in.
    pub fn user(&self) -> &User {
        self.client.user.as_ref().expect("Client is not logged in.")
    }
}
