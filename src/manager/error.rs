use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result};

/// An item in the cache does not exist.
#[derive(Clone, Debug)]
pub enum NotFoundError {
    /// A user was not found in the cache.
    UserNotFound(
        /// The given ID that the user should have.
        u64,
    ),
}

impl Display for NotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::UserNotFound(_) => f.write_str("User could not be resolved from the cache."),
        }
    }
}

impl StdError for NotFoundError {}
