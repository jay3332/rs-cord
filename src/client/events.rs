use crate::ClientState;

use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::future::Future;
use std::pin::Pin;

/// Stores all websocket-related events.
pub struct WsEventStore<Fut>
where
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    pub ready: Option<Box<dyn Fn(ClientState) -> Fut>>,
}

/// The event handler that handles all dispatched events from the gateway.
pub struct EventHandler {
    pub ws: WsEventStore<Pin<Box<dyn Future<Output = ()> + Send + Sync>>>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            ws: WsEventStore::<Pin<Box<dyn Future<Output = ()> + Send + Sync>>> { ready: None },
        }
    }

    pub fn on_ready<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(ClientState) -> Fut + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        self.ws.ready = Some(Box::new(move |a| Box::pin(f(a))));

        self
    }
}

impl Debug for EventHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("EventHandler")
            .field("store:ws", &self.ws)
            .finish()
    }
}

impl<Fut> Debug for WsEventStore<Fut>
where
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("WsEventStore")
            .field(
                "ready",
                &String::from(if self.ready.is_some() { "Some" } else { "None" }),
            )
            .finish()
    }
}
