//! Timestamp representation with precision awareness.

use crate::error::{Error, Result};
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// Precision level for timestamps.
///
/// Real-world data often has varying levels of temporal precision.
/// This enum captures how precise a timestamp is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TemporalPrecision {
    /// Year only (e.g., "2024")
    Year,
    /// Year and month (e.g., "2024-03")
    Month,
    /// Year, month, and day (e.g., "2024-03-15")
    Day,
    /// Down to the hour
    Hour,
    /// Down to the minute
    Minute,
    /// Down to the second
    #[default]
    Second,
    /// Sub-second precision (milliseconds)
    Millisecond,
}

/// A timestamp with timezone awareness and precision tracking.
///
/// Timestamps in spatial narratives often come from sources with varying
/// precision (e.g., "sometime in March 2024" vs "2024-03-15T14:30:00Z").
///
/// # Examples
///
/// ```
/// use spatial_narrative::core::{Timestamp, TemporalPrecision};
///
/// // Current time
/// let now = Timestamp::now();
///
/// // Parse from ISO 8601
/// let ts = Timestamp::parse("2024-03-15T14:30:00Z").unwrap();
///
/// // With explicit precision
/// let approximate = Timestamp::with_precision(
///     Timestamp::parse("2024-03-01T00:00:00Z").unwrap().datetime,
///     TemporalPrecision::Month
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamp {
    /// The datetime in UTC.
    pub datetime: DateTime<Utc>,
    /// The precision of this timestamp.
    #[serde(default)]
    pub precision: TemporalPrecision,
}

impl Timestamp {
    /// Creates a new timestamp with the given datetime and default (Second) precision.
    pub fn new(datetime: DateTime<Utc>) -> Self {
        Self {
            datetime,
            precision: TemporalPrecision::Second,
        }
    }

    /// Creates a new timestamp with explicit precision.
    pub fn with_precision(datetime: DateTime<Utc>, precision: TemporalPrecision) -> Self {
        Self {
            datetime,
            precision,
        }
    }

    /// Creates a timestamp for the current moment.
    pub fn now() -> Self {
        Self::new(Utc::now())
    }

    /// Parses a timestamp from an ISO 8601 string.
    ///
    /// Supported formats:
    /// - `2024-03-15T14:30:00Z` (full precision)
    /// - `2024-03-15T14:30:00+00:00` (with timezone offset)
    /// - `2024-03-15` (date only, day precision)
    /// - `2024-03` (year-month, month precision)
    /// - `2024` (year only, year precision)
    pub fn parse(s: &str) -> Result<Self> {
        // Try full ISO 8601 with timezone
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            return Ok(Self::new(dt.with_timezone(&Utc)));
        }

        // Try full ISO 8601 with Z suffix
        if let Ok(dt) = s.parse::<DateTime<Utc>>() {
            return Ok(Self::new(dt));
        }

        // Try date only (YYYY-MM-DD)
        if s.len() == 10 {
            if let Ok(naive) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
                let dt = naive
                    .and_hms_opt(0, 0, 0)
                    .map(|ndt| Utc.from_utc_datetime(&ndt))
                    .ok_or_else(|| Error::InvalidTimestamp(s.to_string()))?;
                return Ok(Self::with_precision(dt, TemporalPrecision::Day));
            }
        }

        // Try year-month (YYYY-MM)
        if s.len() == 7 && s.chars().nth(4) == Some('-') {
            let year: i32 = s[0..4]
                .parse()
                .map_err(|_| Error::InvalidTimestamp(s.to_string()))?;
            let month: u32 = s[5..7]
                .parse()
                .map_err(|_| Error::InvalidTimestamp(s.to_string()))?;

            if let Some(naive) = chrono::NaiveDate::from_ymd_opt(year, month, 1) {
                let dt = naive
                    .and_hms_opt(0, 0, 0)
                    .map(|ndt| Utc.from_utc_datetime(&ndt))
                    .ok_or_else(|| Error::InvalidTimestamp(s.to_string()))?;
                return Ok(Self::with_precision(dt, TemporalPrecision::Month));
            }
        }

        // Try year only (YYYY)
        if s.len() == 4 {
            let year: i32 = s
                .parse()
                .map_err(|_| Error::InvalidTimestamp(s.to_string()))?;

            if let Some(naive) = chrono::NaiveDate::from_ymd_opt(year, 1, 1) {
                let dt = naive
                    .and_hms_opt(0, 0, 0)
                    .map(|ndt| Utc.from_utc_datetime(&ndt))
                    .ok_or_else(|| Error::InvalidTimestamp(s.to_string()))?;
                return Ok(Self::with_precision(dt, TemporalPrecision::Year));
            }
        }

        Err(Error::InvalidTimestamp(s.to_string()))
    }

    /// Creates a timestamp from Unix epoch seconds.
    pub fn from_unix(secs: i64) -> Option<Self> {
        DateTime::from_timestamp(secs, 0).map(Self::new)
    }

    /// Creates a timestamp from Unix epoch milliseconds.
    pub fn from_unix_millis(millis: i64) -> Option<Self> {
        DateTime::from_timestamp_millis(millis)
            .map(|dt| Self::with_precision(dt, TemporalPrecision::Millisecond))
    }

    /// Returns the Unix timestamp in seconds.
    pub fn unix_timestamp(&self) -> i64 {
        self.datetime.timestamp()
    }

    /// Returns the Unix timestamp in milliseconds.
    pub fn unix_timestamp_millis(&self) -> i64 {
        self.datetime.timestamp_millis()
    }

    /// Returns the Unix timestamp in milliseconds (alias for `unix_timestamp_millis`).
    pub fn to_unix_millis(&self) -> i64 {
        self.datetime.timestamp_millis()
    }

    /// Formats the timestamp as an ISO 8601 string.
    pub fn to_rfc3339(&self) -> String {
        self.datetime.to_rfc3339()
    }

    /// Formats the timestamp according to precision.
    pub fn format_with_precision(&self) -> String {
        match self.precision {
            TemporalPrecision::Year => self.datetime.format("%Y").to_string(),
            TemporalPrecision::Month => self.datetime.format("%Y-%m").to_string(),
            TemporalPrecision::Day => self.datetime.format("%Y-%m-%d").to_string(),
            TemporalPrecision::Hour => self.datetime.format("%Y-%m-%dT%H:00:00Z").to_string(),
            TemporalPrecision::Minute => self.datetime.format("%Y-%m-%dT%H:%M:00Z").to_string(),
            TemporalPrecision::Second | TemporalPrecision::Millisecond => {
                self.datetime.to_rfc3339()
            },
        }
    }

    /// Checks if this timestamp is before another.
    pub fn is_before(&self, other: &Timestamp) -> bool {
        self.datetime < other.datetime
    }

    /// Checks if this timestamp is after another.
    pub fn is_after(&self, other: &Timestamp) -> bool {
        self.datetime > other.datetime
    }

    /// Returns the duration between this timestamp and another.
    pub fn duration_since(&self, earlier: &Timestamp) -> chrono::Duration {
        self.datetime.signed_duration_since(earlier.datetime)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.datetime.cmp(&other.datetime)
    }
}

impl std::hash::Hash for Timestamp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.datetime.hash(state);
        self.precision.hash(state);
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_with_precision())
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(datetime: DateTime<Utc>) -> Self {
        Self::new(datetime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_now() {
        let ts = Timestamp::now();
        assert_eq!(ts.precision, TemporalPrecision::Second);
    }

    #[test]
    fn test_timestamp_parse_rfc3339() {
        let ts = Timestamp::parse("2024-03-15T14:30:00Z").unwrap();
        assert_eq!(ts.datetime.year(), 2024);
        assert_eq!(ts.datetime.month(), 3);
        assert_eq!(ts.datetime.day(), 15);
    }

    #[test]
    fn test_timestamp_parse_date_only() {
        let ts = Timestamp::parse("2024-03-15").unwrap();
        assert_eq!(ts.precision, TemporalPrecision::Day);
        assert_eq!(ts.datetime.year(), 2024);
        assert_eq!(ts.datetime.month(), 3);
        assert_eq!(ts.datetime.day(), 15);
    }

    #[test]
    fn test_timestamp_parse_year_month() {
        let ts = Timestamp::parse("2024-03").unwrap();
        assert_eq!(ts.precision, TemporalPrecision::Month);
        assert_eq!(ts.datetime.year(), 2024);
        assert_eq!(ts.datetime.month(), 3);
    }

    #[test]
    fn test_timestamp_parse_year() {
        let ts = Timestamp::parse("2024").unwrap();
        assert_eq!(ts.precision, TemporalPrecision::Year);
        assert_eq!(ts.datetime.year(), 2024);
    }

    #[test]
    fn test_timestamp_parse_invalid() {
        assert!(Timestamp::parse("not a timestamp").is_err());
        assert!(Timestamp::parse("").is_err());
    }

    #[test]
    fn test_timestamp_from_unix() {
        let ts = Timestamp::from_unix(1710510600).unwrap(); // 2024-03-15T14:30:00Z
        assert_eq!(ts.datetime.year(), 2024);
    }

    #[test]
    fn test_timestamp_format_with_precision() {
        let ts = Timestamp::parse("2024-03").unwrap();
        assert_eq!(ts.format_with_precision(), "2024-03");
    }

    #[test]
    fn test_timestamp_ordering() {
        let ts1 = Timestamp::parse("2024-01-01T00:00:00Z").unwrap();
        let ts2 = Timestamp::parse("2024-06-01T00:00:00Z").unwrap();
        assert!(ts1 < ts2);
        assert!(ts1.is_before(&ts2));
        assert!(ts2.is_after(&ts1));
    }

    #[test]
    fn test_timestamp_duration() {
        let ts1 = Timestamp::parse("2024-01-01T00:00:00Z").unwrap();
        let ts2 = Timestamp::parse("2024-01-02T00:00:00Z").unwrap();
        let duration = ts2.duration_since(&ts1);
        assert_eq!(duration.num_days(), 1);
    }

    #[test]
    fn test_timestamp_serialization() {
        let ts = Timestamp::parse("2024-03-15T14:30:00Z").unwrap();
        let json = serde_json::to_string(&ts).unwrap();
        let parsed: Timestamp = serde_json::from_str(&json).unwrap();
        assert_eq!(ts.datetime, parsed.datetime);
    }

    use chrono::Datelike;
}
