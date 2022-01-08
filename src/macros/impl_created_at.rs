#[macro_export]
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
