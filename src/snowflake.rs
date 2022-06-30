use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    sync::atomic::AtomicU64,
};
use std::sync::atomic::Ordering;

/// Represents a Discord snowflake.
pub trait Snowflake: Copy + Clone + Debug + Display + PartialEq + Eq + PartialOrd + Send + Sync {
    /// Creates a new snowflake from a [`u64`].
    fn from_u64(n: u64) -> Self {
        Self::from_atomic_u64(AtomicU64::new(n))
    }

    /// Creates a new snowflake from an [`AtomicU64`](std::sync::atomic::AtomicU64).
    fn from_atomic_u64(n: AtomicU64) -> Self;

    /// The snowflake ID represented as a [`u64`].
    fn into_u64(self) -> u64 {
        self.as_atomic_u64().into_inner()
    }

    /// The snowflake ID represented as an [`AtomicU64`](std::sync::atomic::AtomicU64).
    fn as_atomic_u64(&self) -> AtomicU64;

    /// Converts this snowflake to another type of snowflake.
    fn convert<S: Snowflake>(&self) -> S {
        S::from_atomic_u64(self.as_atomic_u64())
    }

    /// Creates a blank or placeholder snowflake. Realistically, this has a value of 0.
    fn unset() -> Self {
        Self::from_u64(0)
    }

    fn set(&self, n: u64);
}

macro_rules! impl_snowflake {
    ($($name:ident),+) => {
        $(
            #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
            pub struct $name(AtomicU64);

            impl Snowflake for $name {
                fn from_atomic_u64(n: AtomicU64) -> Self {
                    Self(n)
                }

                fn as_atomic_u64(&self) -> AtomicU64 {
                    *self.0
                }

                fn set(&self, n: u64) {
                    self.0.store(n, Ordering::Relaxed);
                }
            }

            impl Display for $name {
                fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                    write!(f, "{}", self.as_u64())
                }
            }

            impl Eq for $name {}

            impl From<u64> for $name {
                fn from(n: u64) -> Self {
                    $name::from_u64(n)
                }
            }

            impl From<AtomicU64> for $name {
                fn from(n: AtomicU64) -> Self {
                    $name::from_atomic_u64(n)
                }
            }

            impl From<$name> for u64 {
                fn from(n: $name) -> Self {
                    n.as_u64()
                }
            }

            impl From<$name> for AtomicU64 {
                fn from(n: $name) -> Self {
                    n.as_atomic_u64()
                }
            }
        ),+
    };
}

impl_snowflake!(
    AnySnowflake,
    ChannelSnowflake,
    MessageSnowflake,
    UserSnowflake,
    GuildSnowflake,
);
