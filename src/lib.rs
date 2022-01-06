#![feature(never_type)]

mod client;
pub mod constants;
pub mod error;
pub(crate) mod http;
pub(crate) mod internal;
pub mod types;
pub(crate) mod ws;

#[macro_use]
mod macros;

pub(crate) use error::ThreadSafeError;

pub use client::Client;
pub use internal::prelude;
