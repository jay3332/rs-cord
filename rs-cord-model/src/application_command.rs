use crate::Snowflake;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CommandPermissionType {
    Role = 1,
    User = 2,
    Channel = 3,
    Unknown = !0,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandPermission {
    /// The id of the command.
    pub id: Snowflake,
    /// The id of the application the command belongs to.
    pub application_id: Snowflake,
    /// The id of the guild.
    pub guild_id: Snowflake,
    /// The permissions for the command in the guild.
    pub permissions: Vec<CommandPermissionData>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandPermissionData {
    /// The id of the role, user, or channel, depends on `type`
    pub id: Snowflake,
    /// The type of data this permissions applies to.
    pub r#type: CommandPermissionType,
    /// Whether or not the provided data can use the command or not.
    pub permission: bool,
}
