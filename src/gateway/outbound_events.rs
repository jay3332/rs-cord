use super::{Gateway, GatewayError};
use crate::internal::prelude::*;
use crate::models::gateway::OpCode;

use serde_json::json;

use std::env::consts;

impl Gateway {
    pub(crate) async fn identify(&mut self) -> Result<()> {
        self.send_json(&json!({
            "op": 2_u8,
            "d": {
                "token": self.http.token,
                "intents": self.intents.bits(),
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
        }))
        .await
    }

    pub(crate) async fn resume(&mut self) -> Result<()> {
        self.send_json(&json!({
            "op": OpCode::Resume,
            "d": {
                "token": self.http.token,
                "session_id": self.session_id.as_ref().ok_or(GatewayError::NoSessionId)?,
                "seq": self.seq.as_ref(),
            }
        }))
        .await
    }

    pub(crate) async fn heartbeat(&mut self) -> Result<()> {
        self.send_json(&json!({
            "op": OpCode::Heartbeat,
            "d": &self.seq,
        }))
        .await
    }

    pub(crate) async fn request_guild_members(
        &mut self,
        guild_id: u64,
        query: &str,
        limit: u64,
        user_ids: Option<Vec<u64>>,
        nonce: Option<&str>,
    ) -> Result<()> {
        self.send_json(&json!({
            "op": OpCode::RequestGuildMembers,
            "d": {
                "guild_id": guild_id,
                "query": query,
                "limit": limit,
                "user_ids": user_ids,
                "nonce": nonce,
            }
        }))
        .await
    }

    pub(crate) async fn update_voice_state(
        &mut self,
        guild_id: u64,
        channel_id: Option<u64>,
        self_mute: bool,
        self_deaf: bool,
    ) -> Result<()> {
        self.send_json(&json!({
            "op": OpCode::VoiceStateUpdate,
            "d": {
                "guild_id": guild_id,
                "channel_id": channel_id,
                "self_mute": self_mute,
                "self_deaf": self_deaf,
            }
        }))
        .await
    }
}
