//! Core types and traits for spatial narratives.
//!
//! This module provides the fundamental building blocks:
//! - [`Location`] - Geographic coordinates (WGS84)
//! - [`Timestamp`] - Temporal information with precision
//! - [`Event`] - Something that happened at a place and time
//! - [`Narrative`] - A collection of related events
//! - [`SourceRef`] - Reference to source material

mod bounds;
mod event;
mod location;
mod narrative;
mod source;
mod timestamp;
mod traits;

pub use bounds::{GeoBounds, TimeRange};
pub use event::{Event, EventBuilder, EventId};
pub use location::{Location, LocationBuilder};
pub use narrative::{Narrative, NarrativeBuilder, NarrativeId, NarrativeMetadata};
pub use source::{SourceRef, SourceType};
pub use timestamp::{TemporalPrecision, Timestamp};
pub use traits::{SpatialEntity, TemporalEntity};
