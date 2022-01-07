#[macro_export]
macro_rules! intents {
    ($($i:ident),* $(,)*) => {{
        use crate::Intents;
        Intents::from_bits_truncate(0 $( | Intents::$i.bits )*)
    }};
}
