//! Combined spatiotemporal indexing.
//!
//! This module provides indexing that combines both spatial and temporal
//! dimensions for efficient queries on events that have both location
//! and time components.
//!
//! # Example
//!
//! ```rust
//! use spatial_narrative::index::SpatiotemporalIndex;
//! use spatial_narrative::core::{Event, Location, Timestamp, GeoBounds, TimeRange};
//!
//! // Create an index for events
//! let mut index = SpatiotemporalIndex::new();
//!
//! // Add events
//! index.insert(
//!     "Event 1",
//!     &Location::new(40.7128, -74.0060),
//!     &Timestamp::now(),
//! );
//!
//! // Query by both space and time
//! let bounds = GeoBounds::new(39.0, -75.0, 42.0, -73.0);
//! let range = TimeRange::year(2024);
//! let results = index.query(&bounds, &range);
//! ```

use super::{SpatialIndex, TemporalIndex};
use crate::core::{GeoBounds, Location, TimeRange, Timestamp};

/// Combined spatiotemporal index for efficient space-time queries.
///
/// This index maintains both a spatial R-tree and a temporal B-tree,
/// enabling queries that filter by both dimensions efficiently.
#[derive(Debug)]
pub struct SpatiotemporalIndex<T> {
    spatial: SpatialIndex<usize>,
    temporal: TemporalIndex<usize>,
    items: Vec<T>,
    locations: Vec<Location>,
    timestamps: Vec<Timestamp>,
}

impl<T: Clone> SpatiotemporalIndex<T> {
    /// Create an empty spatiotemporal index.
    pub fn new() -> Self {
        Self {
            spatial: SpatialIndex::new(),
            temporal: TemporalIndex::new(),
            items: Vec::new(),
            locations: Vec::new(),
            timestamps: Vec::new(),
        }
    }

    /// Create an index from items with location and timestamp extractors.
    pub fn from_iter<I, L, Ts>(iter: I, location_fn: L, timestamp_fn: Ts) -> Self
    where
        I: IntoIterator<Item = T>,
        L: Fn(&T) -> &Location,
        Ts: Fn(&T) -> &Timestamp,
    {
        let mut index = Self::new();
        for item in iter {
            let loc = location_fn(&item).clone();
            let ts = timestamp_fn(&item).clone();
            index.insert(item, &loc, &ts);
        }
        index
    }

    /// Insert an item with its location and timestamp.
    pub fn insert(&mut self, item: T, location: &Location, timestamp: &Timestamp) {
        let idx = self.items.len();

        self.items.push(item);
        self.locations.push(location.clone());
        self.timestamps.push(timestamp.clone());

        self.spatial.insert(idx, location);
        self.temporal.insert(idx, timestamp);
    }

    /// Query items within both spatial bounds and time range.
    pub fn query(&self, bounds: &GeoBounds, range: &TimeRange) -> Vec<&T> {
        // Get spatial candidates
        let spatial_indices: std::collections::HashSet<usize> = self
            .spatial
            .query_bounds(bounds)
            .into_iter()
            .copied()
            .collect();

        // Get temporal candidates
        let temporal_indices: std::collections::HashSet<usize> = self
            .temporal
            .query_range(range)
            .into_iter()
            .copied()
            .collect();

        // Intersect the results
        spatial_indices
            .intersection(&temporal_indices)
            .map(|&i| &self.items[i])
            .collect()
    }

    /// Query items within spatial bounds only.
    pub fn query_spatial(&self, bounds: &GeoBounds) -> Vec<&T> {
        self.spatial
            .query_bounds(bounds)
            .into_iter()
            .map(|&i| &self.items[i])
            .collect()
    }

    /// Query items within time range only.
    pub fn query_temporal(&self, range: &TimeRange) -> Vec<&T> {
        self.temporal
            .query_range(range)
            .into_iter()
            .map(|&i| &self.items[i])
            .collect()
    }

    /// Find k nearest items to a point within a time range.
    pub fn nearest_in_range(&self, lat: f64, lon: f64, k: usize, range: &TimeRange) -> Vec<&T> {
        // Get temporal candidates first
        let temporal_indices: std::collections::HashSet<usize> = self
            .temporal
            .query_range(range)
            .into_iter()
            .copied()
            .collect();

        // Get spatial nearest and filter by temporal
        self.spatial
            .nearest(lat, lon, k * 2) // Get more candidates to account for filtering
            .into_iter()
            .filter(|&i| temporal_indices.contains(i))
            .take(k)
            .map(|&i| &self.items[i])
            .collect()
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

    /// Get the geographic bounds of all indexed items.
    pub fn bounds(&self) -> Option<GeoBounds> {
        if self.locations.is_empty() {
            return None;
        }
        GeoBounds::from_locations(&self.locations)
    }

    /// Get the time range of all indexed items.
    pub fn time_range(&self) -> Option<TimeRange> {
        self.temporal.time_range()
    }
}

impl<T: Clone> Default for SpatiotemporalIndex<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Specification for generating a heatmap grid.
#[derive(Debug, Clone)]
pub struct GridSpec {
    /// Number of cells in latitude direction
    pub lat_cells: usize,
    /// Number of cells in longitude direction
    pub lon_cells: usize,
    /// Bounds of the grid
    pub bounds: GeoBounds,
}

impl GridSpec {
    /// Create a new grid specification.
    pub fn new(bounds: GeoBounds, lat_cells: usize, lon_cells: usize) -> Self {
        Self {
            bounds,
            lat_cells,
            lon_cells,
        }
    }

    /// Create a grid with approximately square cells.
    pub fn square_cells(bounds: GeoBounds, target_cells: usize) -> Self {
        let lat_range = bounds.max_lat - bounds.min_lat;
        let lon_range = bounds.max_lon - bounds.min_lon;
        let aspect = lon_range / lat_range;

        let lat_cells = ((target_cells as f64 / aspect).sqrt() as usize).max(1);
        let lon_cells = ((target_cells as f64 * aspect).sqrt() as usize).max(1);

        Self {
            bounds,
            lat_cells,
            lon_cells,
        }
    }

    /// Get the cell size in degrees.
    pub fn cell_size(&self) -> (f64, f64) {
        let lat_size = (self.bounds.max_lat - self.bounds.min_lat) / self.lat_cells as f64;
        let lon_size = (self.bounds.max_lon - self.bounds.min_lon) / self.lon_cells as f64;
        (lat_size, lon_size)
    }
}

/// Result of a heatmap computation.
#[derive(Debug, Clone)]
pub struct Heatmap {
    /// The grid specification used
    pub grid: GridSpec,
    /// Count of items in each cell (row-major order)
    pub counts: Vec<usize>,
    /// Maximum count in any cell
    pub max_count: usize,
}

impl Heatmap {
    /// Get the count at a specific cell.
    pub fn get(&self, lat_idx: usize, lon_idx: usize) -> usize {
        if lat_idx >= self.grid.lat_cells || lon_idx >= self.grid.lon_cells {
            return 0;
        }
        self.counts[lat_idx * self.grid.lon_cells + lon_idx]
    }

    /// Get the normalized value (0.0 to 1.0) at a cell.
    pub fn get_normalized(&self, lat_idx: usize, lon_idx: usize) -> f64 {
        if self.max_count == 0 {
            return 0.0;
        }
        self.get(lat_idx, lon_idx) as f64 / self.max_count as f64
    }

    /// Convert to a 2D vector for easier manipulation.
    pub fn to_grid(&self) -> Vec<Vec<usize>> {
        self.counts
            .chunks(self.grid.lon_cells)
            .map(|row| row.to_vec())
            .collect()
    }
}

impl<T: Clone> SpatiotemporalIndex<T> {
    /// Generate a heatmap from the indexed items.
    pub fn heatmap(&self, grid: GridSpec) -> Heatmap {
        let mut counts = vec![0usize; grid.lat_cells * grid.lon_cells];
        let (lat_size, lon_size) = grid.cell_size();

        for loc in &self.locations {
            if !grid.bounds.contains(loc) {
                continue;
            }

            let lat_idx = ((loc.lat - grid.bounds.min_lat) / lat_size) as usize;
            let lon_idx = ((loc.lon - grid.bounds.min_lon) / lon_size) as usize;

            let lat_idx = lat_idx.min(grid.lat_cells - 1);
            let lon_idx = lon_idx.min(grid.lon_cells - 1);

            counts[lat_idx * grid.lon_cells + lon_idx] += 1;
        }

        let max_count = counts.iter().copied().max().unwrap_or(0);

        Heatmap {
            grid,
            counts,
            max_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_timestamp(day: u32) -> Timestamp {
        Timestamp::parse(&format!("2024-01-{:02}T12:00:00Z", day)).unwrap()
    }

    #[test]
    fn test_spatiotemporal_index_new() {
        let index: SpatiotemporalIndex<String> = SpatiotemporalIndex::new();
        assert!(index.is_empty());
    }

    #[test]
    fn test_spatiotemporal_index_insert() {
        let mut index = SpatiotemporalIndex::new();
        index.insert("NYC", &Location::new(40.7128, -74.0060), &make_timestamp(1));
        index.insert("LA", &Location::new(34.0522, -118.2437), &make_timestamp(5));

        assert_eq!(index.len(), 2);
    }

    #[test]
    fn test_spatiotemporal_query() {
        let mut index = SpatiotemporalIndex::new();
        index.insert(
            "NYC Jan 1",
            &Location::new(40.7128, -74.0060),
            &make_timestamp(1),
        );
        index.insert(
            "NYC Jan 15",
            &Location::new(40.7128, -74.0060),
            &make_timestamp(15),
        );
        index.insert(
            "LA Jan 1",
            &Location::new(34.0522, -118.2437),
            &make_timestamp(1),
        );
        index.insert(
            "LA Jan 15",
            &Location::new(34.0522, -118.2437),
            &make_timestamp(15),
        );

        // Query: East Coast in first week
        let bounds = GeoBounds::new(35.0, -80.0, 45.0, -70.0);
        let range = TimeRange::new(make_timestamp(1), make_timestamp(7));

        let results = index.query(&bounds, &range);
        assert_eq!(results.len(), 1);
        assert_eq!(*results[0], "NYC Jan 1");
    }

    #[test]
    fn test_heatmap_generation() {
        let mut index: SpatiotemporalIndex<&str> = SpatiotemporalIndex::new();

        // Add clustered events
        for i in 0..10 {
            index.insert(
                "NYC area",
                &Location::new(40.7 + (i as f64 * 0.01), -74.0 + (i as f64 * 0.01)),
                &make_timestamp(1),
            );
        }
        for i in 0..5 {
            index.insert(
                "LA area",
                &Location::new(34.0 + (i as f64 * 0.01), -118.2 + (i as f64 * 0.01)),
                &make_timestamp(1),
            );
        }

        let bounds = GeoBounds::new(30.0, -125.0, 45.0, -70.0);
        let grid = GridSpec::new(bounds, 10, 10);
        let heatmap = index.heatmap(grid);

        assert!(heatmap.max_count > 0);
    }
}
