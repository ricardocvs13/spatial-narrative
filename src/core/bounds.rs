//! Geographic and temporal bounds for filtering and queries.

use chrono::{Datelike, Duration, TimeZone};
use serde::{Deserialize, Serialize};

use crate::core::{Location, Timestamp};

/// Geographic bounding box.
///
/// Represents a rectangular region defined by minimum and maximum
/// latitude and longitude values.
///
/// # Examples
///
/// ```
/// use spatial_narrative::core::{GeoBounds, Location};
///
/// // Create bounds for the San Francisco Bay Area
/// let bay_area = GeoBounds::new(37.0, -123.0, 38.5, -121.5);
///
/// // Check if a location is within bounds
/// let sf = Location::new(37.7749, -122.4194);
/// assert!(bay_area.contains(&sf));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GeoBounds {
    /// Minimum latitude (south).
    pub min_lat: f64,
    /// Minimum longitude (west).
    pub min_lon: f64,
    /// Maximum latitude (north).
    pub max_lat: f64,
    /// Maximum longitude (east).
    pub max_lon: f64,
}

impl GeoBounds {
    /// Creates a new bounding box.
    ///
    /// # Arguments
    ///
    /// * `min_lat` - Southern boundary
    /// * `min_lon` - Western boundary
    /// * `max_lat` - Northern boundary
    /// * `max_lon` - Eastern boundary
    pub fn new(min_lat: f64, min_lon: f64, max_lat: f64, max_lon: f64) -> Self {
        Self {
            min_lat,
            min_lon,
            max_lat,
            max_lon,
        }
    }

    /// Creates bounds from two corner locations.
    pub fn from_corners(sw: &Location, ne: &Location) -> Self {
        Self::new(sw.lat, sw.lon, ne.lat, ne.lon)
    }

    /// Creates bounds that contain all given locations.
    pub fn from_locations<'a>(locations: impl IntoIterator<Item = &'a Location>) -> Option<Self> {
        let mut iter = locations.into_iter();
        let first = iter.next()?;

        let mut bounds = Self::new(first.lat, first.lon, first.lat, first.lon);

        for loc in iter {
            bounds.expand_to_include(loc);
        }

        Some(bounds)
    }

    /// Creates bounds centered on a point with given radius in degrees.
    pub fn from_center_degrees(center: &Location, lat_radius: f64, lon_radius: f64) -> Self {
        Self::new(
            center.lat - lat_radius,
            center.lon - lon_radius,
            center.lat + lat_radius,
            center.lon + lon_radius,
        )
    }

    /// Checks if a location is within these bounds.
    pub fn contains(&self, location: &Location) -> bool {
        location.lat >= self.min_lat
            && location.lat <= self.max_lat
            && location.lon >= self.min_lon
            && location.lon <= self.max_lon
    }

    /// Checks if these bounds intersect with other bounds.
    pub fn intersects(&self, other: &GeoBounds) -> bool {
        self.min_lat <= other.max_lat
            && self.max_lat >= other.min_lat
            && self.min_lon <= other.max_lon
            && self.max_lon >= other.min_lon
    }

    /// Returns the intersection of two bounds, if any.
    pub fn intersection(&self, other: &GeoBounds) -> Option<GeoBounds> {
        if !self.intersects(other) {
            return None;
        }

        Some(GeoBounds::new(
            self.min_lat.max(other.min_lat),
            self.min_lon.max(other.min_lon),
            self.max_lat.min(other.max_lat),
            self.max_lon.min(other.max_lon),
        ))
    }

    /// Returns bounds that contain both this and other bounds.
    pub fn union(&self, other: &GeoBounds) -> GeoBounds {
        GeoBounds::new(
            self.min_lat.min(other.min_lat),
            self.min_lon.min(other.min_lon),
            self.max_lat.max(other.max_lat),
            self.max_lon.max(other.max_lon),
        )
    }

    /// Expands bounds to include the given location.
    pub fn expand_to_include(&mut self, location: &Location) {
        self.min_lat = self.min_lat.min(location.lat);
        self.max_lat = self.max_lat.max(location.lat);
        self.min_lon = self.min_lon.min(location.lon);
        self.max_lon = self.max_lon.max(location.lon);
    }

    /// Returns the center of the bounds.
    pub fn center(&self) -> Location {
        Location::new(
            (self.min_lat + self.max_lat) / 2.0,
            (self.min_lon + self.max_lon) / 2.0,
        )
    }

    /// Returns the width in degrees (longitude span).
    pub fn width(&self) -> f64 {
        self.max_lon - self.min_lon
    }

    /// Returns the height in degrees (latitude span).
    pub fn height(&self) -> f64 {
        self.max_lat - self.min_lat
    }

    /// Returns the southwest corner.
    pub fn southwest(&self) -> Location {
        Location::new(self.min_lat, self.min_lon)
    }

    /// Returns the northeast corner.
    pub fn northeast(&self) -> Location {
        Location::new(self.max_lat, self.max_lon)
    }

    /// Returns the northwest corner.
    pub fn northwest(&self) -> Location {
        Location::new(self.max_lat, self.min_lon)
    }

    /// Returns the southeast corner.
    pub fn southeast(&self) -> Location {
        Location::new(self.min_lat, self.max_lon)
    }

    /// Converts to a geo-types Rect.
    pub fn to_geo_rect(&self) -> geo_types::Rect<f64> {
        geo_types::Rect::new(
            geo_types::coord! { x: self.min_lon, y: self.min_lat },
            geo_types::coord! { x: self.max_lon, y: self.max_lat },
        )
    }
}

impl Default for GeoBounds {
    fn default() -> Self {
        // World bounds
        Self::new(-90.0, -180.0, 90.0, 180.0)
    }
}

/// Time range for temporal queries.
///
/// Represents a span of time with start and end timestamps.
///
/// # Examples
///
/// ```
/// use spatial_narrative::core::{TimeRange, Timestamp};
///
/// // Create a range for March 2024
/// let march = TimeRange::month(2024, 3);
///
/// // Check if a timestamp is within range
/// let ts = Timestamp::parse("2024-03-15T12:00:00Z").unwrap();
/// assert!(march.contains(&ts));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeRange {
    /// Start of the range (inclusive).
    pub start: Timestamp,
    /// End of the range (inclusive).
    pub end: Timestamp,
}

impl TimeRange {
    /// Creates a new time range.
    pub fn new(start: Timestamp, end: Timestamp) -> Self {
        Self { start, end }
    }

    /// Creates a time range for a specific year.
    pub fn year(year: i32) -> Self {
        let start = Timestamp::parse(&format!("{}", year)).unwrap();
        let end_dt = chrono::NaiveDate::from_ymd_opt(year, 12, 31)
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap();
        let end = Timestamp::new(chrono::Utc.from_utc_datetime(&end_dt));
        Self::new(start, end)
    }

    /// Creates a time range for a specific month.
    pub fn month(year: i32, month: u32) -> Self {
        let start = Timestamp::parse(&format!("{}-{:02}", year, month)).unwrap();

        // Calculate last day of month
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        let last_day = chrono::NaiveDate::from_ymd_opt(next_year, next_month, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day();

        let end_dt = chrono::NaiveDate::from_ymd_opt(year, month, last_day)
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap();
        let end = Timestamp::new(chrono::Utc.from_utc_datetime(&end_dt));

        Self::new(start, end)
    }

    /// Creates a time range for a specific day.
    pub fn day(year: i32, month: u32, day: u32) -> Self {
        let start = Timestamp::parse(&format!("{}-{:02}-{:02}", year, month, day)).unwrap();
        let end_dt = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap();
        let end = Timestamp::new(chrono::Utc.from_utc_datetime(&end_dt));
        Self::new(start, end)
    }

    /// Creates a time range from now going back by the given duration.
    pub fn last(duration: Duration) -> Self {
        let end = Timestamp::now();
        let start = Timestamp::new(end.datetime - duration);
        Self::new(start, end)
    }

    /// Creates a time range from now going forward by the given duration.
    pub fn next(duration: Duration) -> Self {
        let start = Timestamp::now();
        let end = Timestamp::new(start.datetime + duration);
        Self::new(start, end)
    }

    /// Checks if a timestamp is within this range.
    pub fn contains(&self, timestamp: &Timestamp) -> bool {
        timestamp >= &self.start && timestamp <= &self.end
    }

    /// Checks if this range overlaps with another.
    pub fn overlaps(&self, other: &TimeRange) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    /// Returns the intersection of two ranges, if any.
    pub fn intersection(&self, other: &TimeRange) -> Option<TimeRange> {
        if !self.overlaps(other) {
            return None;
        }

        let start = if self.start > other.start {
            self.start.clone()
        } else {
            other.start.clone()
        };

        let end = if self.end < other.end {
            self.end.clone()
        } else {
            other.end.clone()
        };

        Some(TimeRange::new(start, end))
    }

    /// Returns a range that spans both this and another range.
    pub fn union(&self, other: &TimeRange) -> TimeRange {
        let start = if self.start < other.start {
            self.start.clone()
        } else {
            other.start.clone()
        };

        let end = if self.end > other.end {
            self.end.clone()
        } else {
            other.end.clone()
        };

        TimeRange::new(start, end)
    }

    /// Returns the duration of this range.
    pub fn duration(&self) -> Duration {
        self.end.duration_since(&self.start)
    }

    /// Splits the range into smaller ranges of the given duration.
    pub fn split(&self, chunk_duration: Duration) -> Vec<TimeRange> {
        let mut ranges = Vec::new();
        let mut current_start = self.start.clone();

        while current_start < self.end {
            let chunk_end = Timestamp::new(current_start.datetime + chunk_duration);
            let actual_end = if chunk_end > self.end {
                self.end.clone()
            } else {
                chunk_end
            };

            ranges.push(TimeRange::new(current_start.clone(), actual_end.clone()));
            current_start = Timestamp::new(actual_end.datetime + Duration::seconds(1));
        }

        ranges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geobounds_contains() {
        let bounds = GeoBounds::new(37.0, -123.0, 38.5, -121.5);

        let inside = Location::new(37.7749, -122.4194);
        let outside = Location::new(35.0, -120.0);

        assert!(bounds.contains(&inside));
        assert!(!bounds.contains(&outside));
    }

    #[test]
    fn test_geobounds_intersects() {
        let bounds1 = GeoBounds::new(0.0, 0.0, 10.0, 10.0);
        let bounds2 = GeoBounds::new(5.0, 5.0, 15.0, 15.0);
        let bounds3 = GeoBounds::new(20.0, 20.0, 30.0, 30.0);

        assert!(bounds1.intersects(&bounds2));
        assert!(!bounds1.intersects(&bounds3));
    }

    #[test]
    fn test_geobounds_from_locations() {
        let locations = vec![
            Location::new(10.0, 20.0),
            Location::new(30.0, 40.0),
            Location::new(20.0, 30.0),
        ];

        let bounds = GeoBounds::from_locations(&locations).unwrap();
        assert_eq!(bounds.min_lat, 10.0);
        assert_eq!(bounds.max_lat, 30.0);
        assert_eq!(bounds.min_lon, 20.0);
        assert_eq!(bounds.max_lon, 40.0);
    }

    #[test]
    fn test_geobounds_center() {
        let bounds = GeoBounds::new(0.0, 0.0, 10.0, 10.0);
        let center = bounds.center();
        assert_eq!(center.lat, 5.0);
        assert_eq!(center.lon, 5.0);
    }

    #[test]
    fn test_timerange_year() {
        let range = TimeRange::year(2024);
        let inside = Timestamp::parse("2024-06-15T12:00:00Z").unwrap();
        let outside = Timestamp::parse("2023-06-15T12:00:00Z").unwrap();

        assert!(range.contains(&inside));
        assert!(!range.contains(&outside));
    }

    #[test]
    fn test_timerange_month() {
        let range = TimeRange::month(2024, 3);

        let inside = Timestamp::parse("2024-03-15T12:00:00Z").unwrap();
        let outside = Timestamp::parse("2024-04-15T12:00:00Z").unwrap();

        assert!(range.contains(&inside));
        assert!(!range.contains(&outside));
    }

    #[test]
    fn test_timerange_overlaps() {
        let range1 = TimeRange::month(2024, 3);
        let range2 = TimeRange::new(
            Timestamp::parse("2024-03-15T00:00:00Z").unwrap(),
            Timestamp::parse("2024-04-15T00:00:00Z").unwrap(),
        );
        let range3 = TimeRange::month(2024, 5);

        assert!(range1.overlaps(&range2));
        assert!(!range1.overlaps(&range3));
    }

    #[test]
    fn test_timerange_duration() {
        let range = TimeRange::day(2024, 3, 15);
        let duration = range.duration();
        // Should be approximately 24 hours minus 1 second
        assert!(duration.num_hours() >= 23);
    }
}
