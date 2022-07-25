use crate::UserSnowflake;

use reqwest::{Client, Method};

pub type RequestMethod = Method;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApiVersion {
    V9 = 9,
    V10 = 10,
}

impl From<u8> for ApiVersion {
    fn from(v: u8) -> Self {
        match v {
            9 => Self::V9,
            10 => Self::V10,
            _ => panic!("unsupported api version v{}", v),
        }
    }
}

/// Represents an HTTP Client which makes requests to Discord's REST API.
pub struct Http {
    pub(crate) client: Client,

    /// The bot token this client is using to make requests.
    pub token: String,

    /// The Discord application ID. If this is a bot application, this will likely be
    /// the user ID of your bot.
    pub application_id: Option<UserSnowflake>,
}

impl Http {
    pub const USER_AGENT: &'static str = concat!(
        "DiscordBot (https://github.com/jay3332/rs-cord), rs-cord ",
        env!("CARGO_PKG_VERSION"),
    );

    #[must_use]
    pub fn new<T: AsRef<str>>(token: T) -> Self {
        Self {
            client: Client::new(),
            token: token.as_ref().to_string(),
            application_id: None,
        }
    }
}
