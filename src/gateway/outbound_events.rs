use crate::internal::prelude::*;
use crate::constants::OpCodes;
use super::{Gateway, GatewayError};

use std::env::consts;

impl Gateway {
    async fn identify(&mut self) -> Result<()> {
        Ok(self.send_json(
            &json!({
                "op": OpCodes::Identify,
                "d": {
                    "token": self.http.token,
                    "intents": 0_u32,  // self.intents.value,
                    "compress": true,
                    "large_threshold": 250_u8,
                    // shard: ...
                    "presence": null,  // self.presence,
                    "properties": {
                        "$os": consts::OS,
                        "$browser": "rs-cord",
                        "$device": "rs-cord",
                    }
                }
            })
        ).await?)
    }

    async fn resume(&mut self) -> Result<()> {
        Ok(self.send_json(
                &json!({
                    "op": OpCodes::Resume,
                    "d": {
                        "token": self.http.token,
                        "session_id": &self.session_id.ok_or(GatewayError::NoSessionId)?,
                        "seq": &self.seq,
                    }
                })
            ).await?
        )
    }

    async fn heartbeat(&mut self) -> Result<()> {
        Ok(self.send_json(
                &json!({
                    "op": OpCodes::Heartbeat,
                    "d": &self.seq,
                })
            ).await?
        )
    }

    async fn request_guild_members(&mut self, guild_id: u64, query: &str, limit: u64, user_ids: Option<Vec<u64>>, nonce: Option<&str>) -> Result<()> {
        Ok(self.send_json(
                &json!({
                    "op": OpCodes::RequestGuildMembers,
                    "d": {
                        "guild_id": guild_id,
                        "query": query,
                        "limit": limit,
                        "user_ids": user_ids,
                        "nonce": nonce,
                    }
                })
            ).await?
        )
    }

    async fn update_voice_state(&mut self, guild_id: u64, channel_id: Option<u64>, self_mute: bool, self_deaf: bool) -> Result<()> {
        Ok(self.send_json(
                &json!({
                    "op": OpCodes::VoiceStateUpdate,
                    "d": {
                        "guild_id": guild_id,
                        "channel_id": channel_id,
                        "self_mute": self_mute,
                        "self_deaf": self_deaf,
                    }
                })
            ).await?
        )
    }
}
