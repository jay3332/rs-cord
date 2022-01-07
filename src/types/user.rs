use super::Snowflake;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub bot: bool,
    pub mfa_enabled: bool,
    pub locale: Option<String>,
    pub verified: bool,
    pub email: Option<String>,
    pub flags: u64,
    pub premium_type: Option<u8>,
    pub public_flags: u64,
}
