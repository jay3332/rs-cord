use std::sync::Arc;
use std::collections::HashMap;

use super::Route;

use tokio::sync::{RwLock, Mutex};

pub struct Ratelimiter {
    pub global_ratelimited: Arc<Mutex<()>>,

    pub routes_ratelimit: Arc<RwLock<HashMap<Route, Arc<RouteBucket>>>>
}

pub struct RouteBucket {
    limit: u64,
    remaining: u64,
    reset_at: Option<SystemTime>,
    reset_after: Option<Duration>,
}
