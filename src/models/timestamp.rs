use std::time::Duration;

#[cfg(feature = "chrono")]
extern crate chrono;

#[cfg(feature = "chrono")]
use chrono::{DateTime, NaiveDateTime, Utc};

/// An enum to represent relative time.
///
/// # See
/// [`Timestamp::as_relative_time`]
#[derive(Clone, Debug)]
pub enum RelativeTime {
    /// This timestamp is in the past.
    Past(Duration),

    /// This timestamp is in the future.
    Future(Duration),
}

/// Represents a timestamp which contains date and time.
#[derive(Clone, Debug)]
pub struct Timestamp {
    /// Unix timestamp in milliseconds
    timestamp: u64,
}

impl Timestamp {
    /// Creates a new timestamp from a Unix timestamp in milliseconds.
    #[must_use]
    pub fn from_unix(timestamp: u64) -> Self {
        Self { timestamp }
    }

    /// Creates a new timestamp from a Discord snowflake.
    #[must_use]
    pub fn from_snowflake(snowflake: u64) -> Self {
        Self::from_unix(snowflake >> 22)
    }

    /// Creates a new timestamp from a [`chrono::DateTime`].
    ///
    /// # Panics
    /// - Timestamp is before the Unix epoch
    #[cfg(feature = "chrono")]
    #[must_use]
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        Self::from_unix(
            dt.timestamp_millis()
                .try_into()
                .expect("Timestamp is before the Unix epoch."),
        )
    }

    /// Creates a new timestamp from an ISO 8601 timestamp.
    ///
    /// # Panics
    /// - Timestamp is not a valid timestamp.
    #[cfg(feature = "chrono")]
    #[must_use]
    pub fn from_iso(iso: String) -> Self {
        Self::from_datetime(
            DateTime::parse_from_rfc3339(&iso)
                .expect("Failed to parse ISO 8601 timestamp. (Note: this is confined to the RFC 3339 standard.)")
                .with_timezone(&Utc)
        )
    }

    /// The amount of milliseconds since the Unix epoch.
    #[must_use]
    pub fn timestamp_millis(&self) -> u64 {
        self.timestamp
    }

    /// The amount of seconds since the Unix epoch, as a whole number.
    #[must_use]
    pub fn timestamp_secs(&self) -> u64 {
        self.timestamp / 1000_u64
    }

    /// Converts this timestamp into a tuple (seconds, nonoseconds).
    #[must_use]
    pub fn to_secs_nanos(&self) -> (u64, u32) {
        let secs = self.timestamp_secs();
        let nanos = (self.timestamp - secs * 1000_u64) * 1_000_000_u64;

        #[allow(clippy::cast_possible_truncation)]
        (secs, nanos as u32)
    }

    /// The timestamp as a [`chrono::DateTime`].
    #[cfg(feature = "chrono")]
    #[must_use]
    pub fn as_datetime(&self) -> DateTime<Utc> {
        let (secs, nanos) = self.to_secs_nanos();
        #[allow(clippy::cast_possible_wrap)]
        DateTime::from_utc(NaiveDateTime::from_timestamp(secs as i64, nanos), Utc)
    }

    /// Returns a [`Duration`][`std::time::Duration`] that represents how much time has elapsed from this timestamp.
    ///
    /// Note that this method will panic if this timestamp is in the future.
    /// If you would like to convert this into relative time, use [`as_relative_time()`][`Timestamp::as_relative_time`].
    ///
    /// # Panics
    /// - Timestamp is in the future
    #[cfg(feature = "chrono")]
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        Duration::from_millis(
            (chrono::Utc::now().timestamp_millis() - self.timestamp_millis() as i64)
                .try_into()
                .expect("Timestamp is in the future."),
        )
    }

    /// Returns a [`RelativeTime`] that represents how much time has elapsed from this timestamp.
    ///
    /// If the timestamp is exactly now, the returned value is [`RelativeTime::Future`].
    ///
    /// # Example
    /// ```rust
    /// use rs_cord::{Timestamp, RelativeTime};
    /// use RelativeTime::{Past, Future};
    ///
    /// match Timestamp::from_unix(1234).relative_time() {
    ///     Past(dur) => println!("{} seconds ago", dur.as_secs()),
    ///     Future(dur) => println!("{} seconds from now", dur.as_secs()),
    /// }
    /// ```
    #[cfg(feature = "chrono")]
    #[must_use]
    pub fn as_relative_time(&self) -> RelativeTime {
        let delta = chrono::Utc::now().timestamp_millis() - self.timestamp_millis() as i64;

        if delta <= 0 {
            RelativeTime::Future(Duration::from_millis(delta.abs() as u64))
        } else {
            RelativeTime::Past(Duration::from_millis(delta as u64))
        }
    }
}

#[cfg(feature = "chrono")]
impl From<Timestamp> for DateTime<Utc> {
    fn from(ts: Timestamp) -> Self {
        ts.as_datetime()
    }
}

#[cfg(feature = "chrono")]
impl From<DateTime<Utc>> for Timestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Self::from_datetime(dt)
    }
}
