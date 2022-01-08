use super::Snowflake;

use serde::{Deserialize, Deserializer, Serialize, de::Error as DeserializeError};
use serde_json::Map;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoleData {
    pub id: Snowflake,
    pub name: String,
    pub color: u64,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: u64,
    pub permissions: u64,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTagsData>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RoleTagsData {
    pub bot_id: Option<Snowflake>,
    pub integration_id: Option<Snowflake>,
    pub premium_subscriber: bool,
}

impl<'de> Deserialize<'de> for RoleTagsData {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut json = Map::deserialize(deserializer)?;

        let bot_id = if json.contains_key("bot_id") {
            Some(
                json.remove("bot_id")
                    .ok_or_else(|| panic!())  // Impossible to get here
                    .and_then(String::deserialize)
                    .map_err(DeserializeError::custom)?
            )
        } else {
            None
        };

        let integration_id = if json.contains_key("integration_id") {
            Some(
                json.remove("integration_id")
                    .ok_or_else(|| panic!())  // Impossible to get here
                    .and_then(String::deserialize)
                    .map_err(DeserializeError::custom)?
            )
        } else {
            None
        };

        Ok(Self {
            bot_id,
            integration_id,
            premium_subscriber: json.contains_key("premium_subscriber"),
        })
    }
}
