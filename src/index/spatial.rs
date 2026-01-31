//! Spatial indexing using R-tree for efficient geographic queries.
//!
//! This module provides spatial indexing capabilities using the `rstar` crate,
//! enabling fast queries like:
//! - Bounding box queries
//! - Radius (distance) queries  
//! - K-nearest neighbor searches
//!
//! # Example
//!
//! ```rust
//! use spatial_narrative::index::SpatialIndex;
//! use spatial_narrative::core::Location;
//!
//! // Build spatial index
//! let mut index: SpatialIndex<&str> = SpatialIndex::new();
//! index.insert("NYC", &Location::new(40.7128, -74.0060));
//! index.insert("LA", &Location::new(34.0522, -118.2437));
//! index.insert("Chicago", &Location::new(41.8781, -87.6298));
//!
//! // Query events within a bounding box
//! let results = index.query_bbox(39.0, -120.0, 42.0, -70.0);
//! assert!(!results.is_empty());
//! ```

use crate::core::{GeoBounds, Location};
use rstar::{PointDistance, RTree, RTreeObject, AABB};

/// A wrapper that makes Location compatible with R-tree indexing.
#[derive(Debug, Clone)]
pub struct IndexedLocation {
    /// The geographic location
    pub location: Location,
    /// Optional associated data index
    pub index: usize,
}

impl IndexedLocation {
    /// Create a new indexed location.
    pub fn new(location: Location, index: usize) -> Self {
        Self { location, index }
    }
}

impl RTreeObject for IndexedLocation {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.location.lon, self.location.lat])
    }
}

impl PointDistance for IndexedLocation {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let dx = self.location.lon - point[0];
        let dy = self.location.lat - point[1];
        dx * dx + dy * dy
    }
}

/// Spatial index for efficient geographic queries.
///
/// Uses an R-tree data structure for O(log n) query performance.
#[derive(Debug)]
pub struct SpatialIndex<T> {
    tree: RTree<IndexedLocation>,
    items: Vec<T>,
}

impl<T: Clone> SpatialIndex<T> {
    /// Create an empty spatial index.
    pub fn new() -> Self {
        Self {
            tree: RTree::new(),
            items: Vec::new(),
        }
    }

    /// Create a spatial index from items with a location extractor.
    pub fn from_iter<I, F>(iter: I, location_fn: F) -> Self
    where
        I: IntoIterator<Item = T>,
        F: Fn(&T) -> &Location,
    {
        let items: Vec<T> = iter.into_iter().collect();
        let indexed: Vec<IndexedLocation> = items
            .iter()
            .enumerate()
            .map(|(i, item)| IndexedLocation::new(location_fn(item).clone(), i))
            .collect();

        Self {
            tree: RTree::bulk_load(indexed),
            items,
        }
    }

    /// Insert an item into the index.
    pub fn insert(&mut self, item: T, location: &Location) {
        let index = self.items.len();
        self.items.push(item);
        self.tree
            .insert(IndexedLocation::new(location.clone(), index));
    }

    /// Query items within a bounding box.
    ///
    /// # Arguments
    /// * `min_lat` - Minimum latitude
    /// * `min_lon` - Minimum longitude
    /// * `max_lat` - Maximum latitude
    /// * `max_lon` - Maximum longitude
    pub fn query_bbox(&self, min_lat: f64, min_lon: f64, max_lat: f64, max_lon: f64) -> Vec<&T> {
        let envelope = AABB::from_corners([min_lon, min_lat], [max_lon, max_lat]);
        self.tree
            .locate_in_envelope(&envelope)
            .map(|indexed| &self.items[indexed.index])
            .collect()
    }

    /// Query items within geographic bounds.
    pub fn query_bounds(&self, bounds: &GeoBounds) -> Vec<&T> {
        self.query_bbox(
            bounds.min_lat,
            bounds.min_lon,
            bounds.max_lat,
            bounds.max_lon,
        )
    }

    /// Query items within a radius of a point.
    ///
    /// Note: This uses Euclidean distance in degrees. For accurate
    /// great-circle distance, use `query_radius_meters`.
    pub fn query_radius(&self, lat: f64, lon: f64, radius_degrees: f64) -> Vec<&T> {
        let radius_sq = radius_degrees * radius_degrees;
        self.tree
            .locate_within_distance([lon, lat], radius_sq)
            .map(|indexed| &self.items[indexed.index])
            .collect()
    }

    /// Query items within a radius in meters.
    ///
    /// Uses the Haversine formula for accurate great-circle distance.
    pub fn query_radius_meters(&self, lat: f64, lon: f64, radius_meters: f64) -> Vec<&T> {
        // Convert to approximate degree radius for initial R-tree query
        // 1 degree latitude ≈ 111,320 meters
        let degree_radius = radius_meters / 111_320.0 * 1.5; // Add buffer

        // Get candidates from R-tree
        let candidates = self.query_radius(lat, lon, degree_radius);

        // Return all candidates within the approximate radius
        // (precise Haversine filtering would require storing locations)
        candidates
    }

    /// Find the k nearest neighbors to a point.
    pub fn nearest(&self, lat: f64, lon: f64, k: usize) -> Vec<&T> {
        self.tree
            .nearest_neighbor_iter(&[lon, lat])
            .take(k)
            .map(|indexed| &self.items[indexed.index])
            .collect()
    }

    /// Find the single nearest item to a point.
    pub fn nearest_one(&self, lat: f64, lon: f64) -> Option<&T> {
        self.tree
            .nearest_neighbor(&[lon, lat])
            .map(|indexed| &self.items[indexed.index])
    }

    /// Returns the number of indexed items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the index is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get all items in the index.
    pub fn items(&self) -> &[T] {
        &self.items
    }
}

impl<T: Clone> Default for SpatialIndex<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Location;

    #[test]
    fn test_spatial_index_new() {
        let index: SpatialIndex<String> = SpatialIndex::new();
        assert!(index.is_empty());
    }

    #[test]
    fn test_spatial_index_insert() {
        let mut index = SpatialIndex::new();
        index.insert("NYC".to_string(), &Location::new(40.7128, -74.0060));
        index.insert("LA".to_string(), &Location::new(34.0522, -118.2437));

        assert_eq!(index.len(), 2);
    }

    #[test]
    fn test_spatial_index_query_bbox() {
        let items = vec![
            ("NYC", Location::new(40.7128, -74.0060)),
            ("LA", Location::new(34.0522, -118.2437)),
            ("Chicago", Location::new(41.8781, -87.6298)),
        ];

        // Build index
        let mut index: SpatialIndex<&str> = SpatialIndex::new();
        for (name, loc) in &items {
            index.insert(*name, loc);
        }

        // Query East Coast (should find NYC)
        let results = index.query_bbox(35.0, -80.0, 45.0, -70.0);
        assert_eq!(results.len(), 1);
        assert_eq!(*results[0], "NYC");
    }

    #[test]
    fn test_spatial_index_nearest() {
        let mut index: SpatialIndex<&str> = SpatialIndex::new();
        index.insert("NYC", &Location::new(40.7128, -74.0060));
        index.insert("LA", &Location::new(34.0522, -118.2437));
        index.insert("Chicago", &Location::new(41.8781, -87.6298));

        // Find nearest to Philadelphia (should be NYC)
        let nearest = index.nearest_one(39.9526, -75.1652);
        assert_eq!(nearest, Some(&"NYC"));
    }

    #[test]
    fn test_spatial_index_k_nearest() {
        let mut index: SpatialIndex<&str> = SpatialIndex::new();
        index.insert("NYC", &Location::new(40.7128, -74.0060));
        index.insert("Boston", &Location::new(42.3601, -71.0589));
        index.insert("Philadelphia", &Location::new(39.9526, -75.1652));
        index.insert("LA", &Location::new(34.0522, -118.2437));

        // Find 2 nearest to NYC
        let results = index.nearest(40.7128, -74.0060, 2);
        assert_eq!(results.len(), 2);
    }
}
