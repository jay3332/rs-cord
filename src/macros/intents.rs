/// Aids in constructing a set of gateway intent bitflags.
///
/// # See
/// - [`Intents`]
///
/// # Example
///
/// ```rust
/// use rs_cord::{intents, Intents};
///
/// assert_eq!(intents!(GUILDS, MESSAGES), Intents::GUILDS | Intents::MESSAGES);
/// ```
#[macro_export]
macro_rules! intents {
    ($($i:ident),* $(,)*) => {{
        use $crate::Intents;
        Intents::from_bits_truncate(0 $( | Intents::$i.bits )*)
    }};
}
