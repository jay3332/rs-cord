use super::Snowflake;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPayload {
    id: Snowflake,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: bool,
    mfa_enabled: bool,
    locale: Option<String>,
    verified: bool,
    email: Option<String>,
    flags: u64,
    premium_type: Option<u8>,
    public_flags: u64,
}
