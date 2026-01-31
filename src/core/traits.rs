//! Core traits for spatial and temporal entities.

use crate::core::{GeoBounds, Location, TimeRange, Timestamp};

/// Trait for entities with a spatial location.
///
/// This trait provides a common interface for anything that has
/// a geographic location, enabling spatial indexing and queries.
pub trait SpatialEntity {
    /// Returns the primary location of this entity.
    fn location(&self) -> &Location;

    /// Returns the geographic bounds of this entity.
    ///
    /// For point entities, this returns a zero-size bounding box
    /// at the location. For entities with extent, this returns
    /// the full bounding box.
    fn bounds(&self) -> GeoBounds {
        let loc = self.location();
        GeoBounds::new(loc.lat, loc.lon, loc.lat, loc.lon)
    }

    /// Returns the location as a geo-types Point.
    fn to_geo_point(&self) -> geo_types::Point<f64> {
        self.location().to_geo_point()
    }

    /// Checks if this entity is within the given bounds.
    fn is_within_bounds(&self, bounds: &GeoBounds) -> bool {
        bounds.contains(self.location())
    }
}

/// Trait for entities with temporal information.
///
/// This trait provides a common interface for anything that has
/// a timestamp, enabling temporal indexing and queries.
pub trait TemporalEntity {
    /// Returns the primary timestamp of this entity.
    fn timestamp(&self) -> &Timestamp;

    /// Returns the time range of this entity.
    ///
    /// For instantaneous events, this returns a range where
    /// start equals end. For entities with duration, this
    /// returns the full time span.
    fn time_range(&self) -> TimeRange {
        let ts = self.timestamp();
        TimeRange::new(ts.clone(), ts.clone())
    }

    /// Checks if this entity falls within the given time range.
    fn is_within_time_range(&self, range: &TimeRange) -> bool {
        range.contains(self.timestamp())
    }
}

/// Trait for entities that can be spatiotemporally indexed.
///
/// This is a convenience trait that combines `SpatialEntity` and
/// `TemporalEntity` for entities that have both space and time.
#[allow(dead_code)]
pub trait SpatiotemporalEntity: SpatialEntity + TemporalEntity {
    /// Checks if this entity is within both spatial bounds and time range.
    fn is_within(&self, bounds: &GeoBounds, range: &TimeRange) -> bool {
        self.is_within_bounds(bounds) && self.is_within_time_range(range)
    }
}

// Blanket implementation for anything that implements both traits
impl<T: SpatialEntity + TemporalEntity> SpatiotemporalEntity for T {}

// Implement traits for Event
use crate::core::Event;

impl SpatialEntity for Event {
    fn location(&self) -> &Location {
        &self.location
    }
}

impl TemporalEntity for Event {
    fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_spatial_entity() {
        let event = Event::builder()
            .location(Location::new(40.7128, -74.0060))
            .timestamp(Timestamp::now())
            .text("Test")
            .build();

        assert_eq!(event.location().lat, 40.7128);
        assert_eq!(event.location().lon, -74.0060);
    }

    #[test]
    fn test_event_is_within_bounds() {
        let event = Event::builder()
            .location(Location::new(40.7128, -74.0060))
            .timestamp(Timestamp::now())
            .text("NYC Event")
            .build();

        let nyc_bounds = GeoBounds::new(40.0, -75.0, 41.0, -73.0);
        let la_bounds = GeoBounds::new(33.0, -119.0, 35.0, -117.0);

        assert!(event.is_within_bounds(&nyc_bounds));
        assert!(!event.is_within_bounds(&la_bounds));
    }

    #[test]
    fn test_event_temporal_entity() {
        let event = Event::builder()
            .location(Location::new(40.0, -74.0))
            .timestamp(Timestamp::parse("2024-03-15T12:00:00Z").unwrap())
            .text("Test")
            .build();

        let march = TimeRange::month(2024, 3);
        let april = TimeRange::month(2024, 4);

        assert!(event.is_within_time_range(&march));
        assert!(!event.is_within_time_range(&april));
    }

    #[test]
    fn test_event_spatiotemporal() {
        let event = Event::builder()
            .location(Location::new(40.7128, -74.0060))
            .timestamp(Timestamp::parse("2024-03-15T12:00:00Z").unwrap())
            .text("NYC in March")
            .build();

        let nyc_bounds = GeoBounds::new(40.0, -75.0, 41.0, -73.0);
        let march = TimeRange::month(2024, 3);
        let april = TimeRange::month(2024, 4);

        assert!(event.is_within(&nyc_bounds, &march));
        assert!(!event.is_within(&nyc_bounds, &april));
    }
}
