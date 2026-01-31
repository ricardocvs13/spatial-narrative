//! # Geoparsing Module
//!
//! This module provides functionality for extracting geographic locations
//! from unstructured text.
//!
//! ## Overview
//!
//! The parser module can detect and extract:
//! - **Coordinates**: Decimal degrees, degrees with symbols, DMS format
//! - **Place names**: Using a configurable gazetteer for place name resolution
//!
//! ## Key Types
//!
//! - [`GeoParser`]: Main parser for extracting locations from text
//! - [`LocationMention`]: A detected location mention in text
//! - [`MentionType`]: Classification of location mention types
//! - [`LocationPattern`]: Configuration for what patterns to detect
//! - [`Gazetteer`]: Trait for place name resolution
//! - [`BuiltinGazetteer`]: Built-in gazetteer with 200+ major world locations
//!
//! ## Example
//!
//! ```rust
//! use spatial_narrative::parser::{GeoParser, BuiltinGazetteer};
//!
//! // Create a parser with the built-in gazetteer
//! let gazetteer = BuiltinGazetteer::new();
//! let parser = GeoParser::with_gazetteer(Box::new(gazetteer));
//!
//! // Extract locations from text
//! let text = "The conference in Paris started at 48.8566°N, 2.3522°E.";
//! let mentions = parser.extract(text);
//!
//! for mention in mentions {
//!     println!("Found '{}' at position {}-{}", mention.text, mention.start, mention.end);
//!     if let Some(loc) = mention.location {
//!         println!("  Coordinates: {}, {}", loc.lat, loc.lon);
//!     }
//! }
//! ```

mod gazetteer;
mod geoparser;
mod mention;

pub use gazetteer::{BuiltinGazetteer, Gazetteer, GazetteerEntry};
pub use geoparser::GeoParser;
pub use mention::{LocationMention, LocationPattern, MentionType};
