#![feature(never_type)]

mod client;
pub mod constants;
pub mod error;
mod http;
mod internal;
#[macro_use]
mod macros;

pub(crate) use error::ThreadSafeError;

pub use client::Client;
