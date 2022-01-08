use super::Snowflake;
use super::user::UserData;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmojiData {
    pub id: Snowflake,
    pub name: String,
    pub roles: Option<Vec<Snowflake>>,
    pub user: Option<UserData>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialEmojiData {
    pub id: Option<Snowflake>,
    pub name: Option<String>,
    pub roles: Option<Vec<Snowflake>>,
    pub user: Option<UserData>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityEmojiData {
    pub id: Option<Snowflake>,
    pub name: String,
    pub animated: Option<bool>,
}
