#![allow(dead_code)]
#![feature(never_type)]

mod client;
pub mod constants;
pub mod error;
pub(crate) mod http;
pub(crate) mod internal;
pub mod macros;
pub mod types;
pub(crate) mod ws;

pub use client::Client;
pub use error::{Error as RsCordError, ThreadSafeError, ThreadSafeResult};
pub use internal::prelude;
