#![allow(dead_code)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

mod client;
pub mod constants;
pub mod error;
pub(crate) mod gateway;
pub(crate) mod http;
pub(crate) mod internal;
pub mod macros;
pub mod models;
pub mod types;

pub use client::{Client, ClientState};
pub use error::{Error as RsCordError, ThreadSafeError, ThreadSafeResult};
pub use internal::prelude;
pub use models::*;

#[macro_use]
extern crate tracing;
