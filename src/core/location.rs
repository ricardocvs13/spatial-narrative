//! Geographic location representation.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// A geographic location using WGS84 coordinates.
///
/// Locations are the fundamental spatial unit in spatial narratives.
/// They support optional elevation and uncertainty for real-world data.
///
/// # Examples
///
/// ```
/// use spatial_narrative::core::Location;
///
/// // Create a location (New York City)
/// let nyc = Location::new(40.7128, -74.0060);
/// assert!(nyc.is_valid());
///
/// // With elevation (Mount Everest)
/// let peak = Location::with_elevation(27.9881, 86.9250, 8848.86);
///
/// // Using the builder for more options
/// let approximate = Location::builder()
///     .coordinates(40.7, -74.0)
///     .uncertainty_meters(1000.0)
///     .name("Approximate NYC")
///     .build()
///     .unwrap();
/// ```
///
/// # Coordinate System
///
/// Locations use WGS84 (EPSG:4326):
/// - Latitude: -90° to +90° (negative = South)
/// - Longitude: -180° to +180° (negative = West)
/// - Elevation: meters above sea level (optional)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// Latitude in decimal degrees (-90 to 90).
    pub lat: f64,
    /// Longitude in decimal degrees (-180 to 180).
    pub lon: f64,
    /// Elevation in meters above sea level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
    /// Uncertainty radius in meters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncertainty_meters: Option<f64>,
    /// Human-readable place name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Location {
    /// Creates a new location with the given latitude and longitude.
    ///
    /// # Arguments
    ///
    /// * `lat` - Latitude in decimal degrees (-90 to 90)
    /// * `lon` - Longitude in decimal degrees (-180 to 180)
    ///
    /// # Examples
    ///
    /// ```
    /// use spatial_narrative::core::Location;
    ///
    /// let loc = Location::new(51.5074, -0.1278); // London
    /// ```
    pub fn new(lat: f64, lon: f64) -> Self {
        Self {
            lat,
            lon,
            elevation: None,
            uncertainty_meters: None,
            name: None,
        }
    }

    /// Creates a new location with elevation.
    ///
    /// # Arguments
    ///
    /// * `lat` - Latitude in decimal degrees
    /// * `lon` - Longitude in decimal degrees
    /// * `elevation` - Elevation in meters above sea level
    pub fn with_elevation(lat: f64, lon: f64, elevation: f64) -> Self {
        Self {
            lat,
            lon,
            elevation: Some(elevation),
            uncertainty_meters: None,
            name: None,
        }
    }

    /// Creates a new builder for constructing a Location.
    pub fn builder() -> LocationBuilder {
        LocationBuilder::new()
    }

    /// Checks if the coordinates are valid WGS84 values.
    ///
    /// Returns `true` if latitude is between -90 and 90,
    /// and longitude is between -180 and 180.
    pub fn is_valid(&self) -> bool {
        self.lat >= -90.0 && self.lat <= 90.0 && self.lon >= -180.0 && self.lon <= 180.0
    }

    /// Validates the location, returning an error if invalid.
    pub fn validate(&self) -> Result<()> {
        if self.lat < -90.0 || self.lat > 90.0 {
            return Err(Error::InvalidLatitude(self.lat));
        }
        if self.lon < -180.0 || self.lon > 180.0 {
            return Err(Error::InvalidLongitude(self.lon));
        }
        Ok(())
    }

    /// Returns the coordinates as a tuple (lat, lon).
    pub fn as_tuple(&self) -> (f64, f64) {
        (self.lat, self.lon)
    }

    /// Returns the coordinates as a geo-types Point.
    pub fn to_geo_point(&self) -> geo_types::Point<f64> {
        geo_types::Point::new(self.lon, self.lat)
    }

    /// Creates a Location from a geo-types Point.
    pub fn from_geo_point(point: geo_types::Point<f64>) -> Self {
        Self::new(point.y(), point.x())
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl From<(f64, f64)> for Location {
    fn from((lat, lon): (f64, f64)) -> Self {
        Self::new(lat, lon)
    }
}

impl From<geo_types::Point<f64>> for Location {
    fn from(point: geo_types::Point<f64>) -> Self {
        Self::from_geo_point(point)
    }
}

/// Builder for constructing [`Location`] instances.
#[derive(Debug, Default)]
pub struct LocationBuilder {
    lat: Option<f64>,
    lon: Option<f64>,
    elevation: Option<f64>,
    uncertainty_meters: Option<f64>,
    name: Option<String>,
}

impl LocationBuilder {
    /// Creates a new LocationBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the latitude.
    pub fn lat(mut self, lat: f64) -> Self {
        self.lat = Some(lat);
        self
    }

    /// Sets the longitude.
    pub fn lon(mut self, lon: f64) -> Self {
        self.lon = Some(lon);
        self
    }

    /// Sets both latitude and longitude.
    pub fn coordinates(mut self, lat: f64, lon: f64) -> Self {
        self.lat = Some(lat);
        self.lon = Some(lon);
        self
    }

    /// Sets the elevation in meters.
    pub fn elevation(mut self, elevation: f64) -> Self {
        self.elevation = Some(elevation);
        self
    }

    /// Sets the uncertainty radius in meters.
    pub fn uncertainty_meters(mut self, uncertainty: f64) -> Self {
        self.uncertainty_meters = Some(uncertainty);
        self
    }

    /// Sets the place name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Builds the Location, returning an error if required fields are missing.
    pub fn build(self) -> Result<Location> {
        let lat = self.lat.ok_or(Error::MissingField("lat"))?;
        let lon = self.lon.ok_or(Error::MissingField("lon"))?;

        let location = Location {
            lat,
            lon,
            elevation: self.elevation,
            uncertainty_meters: self.uncertainty_meters,
            name: self.name,
        };

        location.validate()?;
        Ok(location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_new() {
        let loc = Location::new(40.7128, -74.0060);
        assert_eq!(loc.lat, 40.7128);
        assert_eq!(loc.lon, -74.0060);
        assert!(loc.elevation.is_none());
        assert!(loc.is_valid());
    }

    #[test]
    fn test_location_with_elevation() {
        let loc = Location::with_elevation(27.9881, 86.9250, 8848.86);
        assert_eq!(loc.elevation, Some(8848.86));
    }

    #[test]
    fn test_location_validation() {
        let valid = Location::new(45.0, 90.0);
        assert!(valid.is_valid());
        assert!(valid.validate().is_ok());

        let invalid_lat = Location::new(91.0, 0.0);
        assert!(!invalid_lat.is_valid());
        assert!(invalid_lat.validate().is_err());

        let invalid_lon = Location::new(0.0, 181.0);
        assert!(!invalid_lon.is_valid());
        assert!(invalid_lon.validate().is_err());
    }

    #[test]
    fn test_location_builder() {
        let loc = Location::builder()
            .coordinates(51.5074, -0.1278)
            .elevation(11.0)
            .uncertainty_meters(10.0)
            .name("London")
            .build()
            .unwrap();

        assert_eq!(loc.lat, 51.5074);
        assert_eq!(loc.lon, -0.1278);
        assert_eq!(loc.elevation, Some(11.0));
        assert_eq!(loc.uncertainty_meters, Some(10.0));
        assert_eq!(loc.name, Some("London".to_string()));
    }

    #[test]
    fn test_location_builder_missing_fields() {
        let result = Location::builder().lat(40.0).build();
        assert!(result.is_err());
    }

    #[test]
    fn test_location_from_tuple() {
        let loc: Location = (40.7128, -74.0060).into();
        assert_eq!(loc.lat, 40.7128);
        assert_eq!(loc.lon, -74.0060);
    }

    #[test]
    fn test_location_to_geo_point() {
        let loc = Location::new(40.7128, -74.0060);
        let point = loc.to_geo_point();
        assert_eq!(point.x(), -74.0060);
        assert_eq!(point.y(), 40.7128);
    }

    #[test]
    fn test_location_serialization() {
        let loc = Location::new(40.7128, -74.0060);
        let json = serde_json::to_string(&loc).unwrap();
        let parsed: Location = serde_json::from_str(&json).unwrap();
        assert_eq!(loc, parsed);
    }
}
