use crate::{ClientState, ThreadSafeError, ThreadSafeResult};

type Result = ThreadSafeResult<()>;

#[crate::async_trait]
#[allow(dead_code)]
pub trait WsEventHandler {
    async fn on_ready(state: &ClientState) -> Result {}
    async fn on_message_create(state: &ClientState) -> Result {}
    async fn on_message_update(state: &ClientState) -> Result {}
    async fn on_message_delete(state: &ClientState) -> Result {}
}
