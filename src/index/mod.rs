//! Spatial and temporal indexing for efficient queries.
//!
//! This module provides data structures for fast spatial and temporal
//! lookups of events and other entities.
//!
//! # Overview
//!
//! - [`SpatialIndex`] - R-tree based spatial indexing for geographic queries
//! - [`TemporalIndex`] - B-tree based temporal indexing for time-range queries
//! - [`SpatiotemporalIndex`] - Combined space-time indexing
//!
//! # Example
//!
//! ```rust
//! use spatial_narrative::index::{SpatialIndex, TemporalIndex, SpatiotemporalIndex};
//! use spatial_narrative::core::{Location, Timestamp, GeoBounds, TimeRange};
//!
//! // Create a spatiotemporal index
//! let mut index = SpatiotemporalIndex::new();
//!
//! index.insert("Event 1",
//!     &Location::new(40.7128, -74.0060),
//!     &Timestamp::now());
//!
//! // Query by location
//! let bounds = GeoBounds::new(39.0, -75.0, 42.0, -73.0);
//! let spatial_results = index.query_spatial(&bounds);
//!
//! // Query by time
//! let range = TimeRange::year(2024);
//! let temporal_results = index.query_temporal(&range);
//!
//! // Query by both
//! let combined_results = index.query(&bounds, &range);
//! ```

mod spatial;
mod spatiotemporal;
mod temporal;

pub use spatial::{IndexedLocation, SpatialIndex};
pub use spatiotemporal::{GridSpec, Heatmap, SpatiotemporalIndex};
pub use temporal::{SlidingWindowIter, TemporalIndex};
