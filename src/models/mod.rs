pub mod asset;
pub mod color;
pub mod gateway;
pub mod message;
pub mod timestamp;
pub mod user;

pub use asset::{Asset, AssetQuality};
pub use color::{Color, Colour};
pub use gateway::Intents;
pub use message::Message;
pub use timestamp::{RelativeTime, Timestamp};
pub use user::User;

macro_rules! impl_created_at {
    ($i:ident) => {
        impl $i {
            /// Returns a [`Timestamp`] of when this object was created.
            ///
            /// This is based of the snowflake of this object.
            #[must_use]
            pub fn created_at(&self) -> $crate::Timestamp {
                $crate::Timestamp::from_snowflake(self.id)
            }
        }
    };
}

pub(crate) use impl_created_at;
