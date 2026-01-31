//! # spatial-narrative
//!
//! A Rust library for representing, analyzing, and working with narratives
//! that unfold across real-world geographic space.
//!
//! ## Overview
//!
//! `spatial-narrative` provides tools for:
//! - Representing events with geographic coordinates and timestamps
//! - Organizing events into coherent narratives
//! - Efficient spatial and temporal indexing
//! - Graph-based analysis of event relationships
//! - Clustering and pattern detection
//! - Import/export in standard formats (GeoJSON, CSV, GPX)
//! - Text processing and geoparsing
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use spatial_narrative::prelude::*;
//!
//! // Create a location
//! let location = Location::new(40.7128, -74.0060);
//!
//! // Create an event
//! let event = Event::builder()
//!     .location(location)
//!     .timestamp(Timestamp::now())
//!     .text("Something happened here")
//!     .tag("example")
//!     .build();
//!
//! // Create a narrative
//! let narrative = Narrative::builder()
//!     .title("My Narrative")
//!     .event(event)
//!     .build();
//! ```
//!
//! ## Modules
//!
//! - [`core`] - Fundamental types: `Location`, `Timestamp`, `Event`, `Narrative`
//! - [`index`] - Spatial and temporal indexing for efficient queries
//! - [`graph`] - Graph representation of narratives
//! - [`analysis`] - Metrics, clustering, and movement analysis
//! - [`io`] - Import/export in various formats
//! - [`transform`] - Coordinate transformations and projections
//! - [`parser`] - Extract locations from unstructured text (geoparsing)
//! - [`text`] - Natural language processing utilities

pub mod analysis;
pub mod core;
pub mod graph;
pub mod index;
pub mod io;
pub mod parser;
pub mod text;
pub mod transform;

/// Error types for the library
pub mod error;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::core::{
        Event, EventBuilder, EventId, GeoBounds, Location, LocationBuilder, Narrative,
        NarrativeBuilder, NarrativeId, NarrativeMetadata, SourceRef, SourceType, SpatialEntity,
        TemporalEntity, TemporalPrecision, TimeRange, Timestamp,
    };
    pub use crate::error::{Error, Result};

    // Re-export commonly used parser types
    pub use crate::parser::{BuiltinGazetteer, Gazetteer, GeoParser, LocationMention, MentionType};

    // Re-export commonly used text types
    pub use crate::text::{Entity, EntityType, Keyword, KeywordExtractor, TextAnalyzer};
}

pub use error::{Error, Result};
