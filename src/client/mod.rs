use crate::Error;

use std::{env, ffi::OsStr, fmt::Display};

/// Represents the builder for `Client` structs.
#[derive(Clone, Debug)]
pub struct ClientBuilder {
    token: Option<String>,
}

impl ClientBuilder {
    fn new() -> Self {
        Self {
            token: None,
        }
    }

    /// Specifies the token the client will use for HTTP requests and the gateway.
    ///
    /// # Parameters
    /// - `token`: Anything that implements `AsRef<str>`, such as a `String` or `&str` representing
    /// your bot's token.
    ///
    /// # Returns
    /// [`Self`]
    pub fn with_token<T: AsRef<str>>(mut self, token: T) -> Self {
        self.token = Some(token);
        self
    }

    /// Specifies the token the client will use from an environment variable.
    ///
    /// # Note
    /// This does panic if the environment variable was not found! If you need to handle
    /// this yourself, it is recommended you use [`with_token`](ClientBuilder::with_token) in
    /// conjunction with [`std::env::var`] instead.
    ///
    /// # Parameters
    ///  - `key`: The name of the environment variable. Must implement both `AsRef<OsStr>` and
    /// `Display`, which means that a `String` or `&str` will suffice.
    ///
    /// # Returns
    /// [`Self`]
    ///
    /// # Panics
    /// - The environment variable was not found.
    pub fn with_env_token<T: AsRef<OsStr> + Display>(mut self, key: T) -> Self {
        self.with_token(env::var(key.as_ref()).expect(&*format!("no environment variable named {}", key)))
    }

    /// Builds the [`Client`].
    ///
    /// # Returns
    /// A result of the built client, [`Result<Client, Error>`].
    ///
    /// # Errors
    /// - No token was specified for the client.
    /// - No intents were specified for the client.
    pub fn build(self) -> Result<Client, Error> {
        if self.token.is_none() {
            return Err(Error::Library("No token specified for the client".to_string()));
        }

        Ok(Client {
            token: token.unwrap(),
        })
    }
}

#[derive(Debug)]
/// The client which manages the HTTP client, websocket messages,
/// and any other requests to and from Discord, along with metadata that
/// may get stored along the way.
///
/// These should not be created manually, instead use the builder API
/// via [`Client::build`].
pub struct Client {
    /// The bot token that this client is using to make requests.
    pub token: String,
}

impl Client {
    #[must_use]
    pub fn build() -> ClientBuilder {
        ClientBuilder::new()
    }
}
