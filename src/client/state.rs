use std::sync::Arc;

use super::Client;
use crate::http::HttpClient;
use crate::User;

/// Represents the state of the client.
#[derive(Clone, Debug)]
pub struct ClientState {
    /// An reference to the client serving this state.
    pub client: Arc<Client>,

    /// The HTTP Client being used.
    pub http: Arc<HttpClient>,

    /// The current authentication token being used by the client.
    /// This should not be exposed to anyone.
    pub token: String,
}

impl ClientState {
    /// A clone of the user the client is logged in to.
    ///
    /// This method is asynchronous due to that fact that it will non-blockingly wait for
    /// the write lock to be released before it acquires the user.
    /// 
    /// # Panics
    /// - The client is not logged in.
    pub async fn user(&self) -> User {
        let cache = self.client.cache.read().await;
        
        cache.client_user.clone().expect("Client is not logged in.")
    }
}

// For some reason these did not get automatically implemented.
unsafe impl Send for ClientState {}
unsafe impl Sync for ClientState {}
