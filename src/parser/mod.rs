//! # Geoparsing Module
//!
//! This module provides functionality for extracting geographic locations
//! from unstructured text.
//!
//! ## Overview
//!
//! The parser module can detect and extract:
//! - **Coordinates**: Decimal degrees, degrees with symbols, DMS format
//! - **Place names**: Using configurable gazetteers for place name resolution
//!
//! ## Key Types
//!
//! - [`GeoParser`]: Main parser for extracting locations from text
//! - [`LocationMention`]: A detected location mention in text
//! - [`MentionType`]: Classification of location mention types
//! - [`LocationPattern`]: Configuration for what patterns to detect
//! - [`Gazetteer`]: Trait for place name resolution
//! - [`BuiltinGazetteer`]: Built-in gazetteer with 200+ major world locations
//! - [`MultiGazetteer`]: Combines multiple gazetteers with fallback
//!
//! ### API Gazetteers (requires `geocoding` feature)
//!
//! - `GazetteerNominatim`: OpenStreetMap Nominatim API
//! - `GazetteerWikidata`: Wikidata SPARQL query service
//! - `GazetteerGeoNames`: GeoNames web service (requires username)
//!
//! ## Examples
//!
//! ### Basic Usage with Built-in Gazetteer
//!
//! ```rust,no_run
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
//!
//! ### Using Multiple Gazetteers with Fallback
//!
//! ```rust,no_run
//! use spatial_narrative::parser::{GeoParser, BuiltinGazetteer, MultiGazetteer};
//!
//! // Combine built-in with API fallbacks
//! let mut multi = MultiGazetteer::new();
//! multi.add_source(Box::new(BuiltinGazetteer::new()));
//!
//! // Built-in will be tried first, then APIs if enabled
//! # #[cfg(feature = "geocoding")]
//! # {
//! use spatial_narrative::parser::GazetteerNominatim;
//! multi.add_source(Box::new(GazetteerNominatim::new()));
//! # }
//!
//! let parser = GeoParser::with_gazetteer(Box::new(multi));
//! let mentions = parser.extract("Meeting in Seattle tomorrow");
//! ```
//!
//! ### Using API Gazetteers (requires `geocoding` feature)
//!
//! ```rust,no_run
//! # #[cfg(feature = "geocoding")]
//! # {
//! use spatial_narrative::parser::{GazetteerNominatim, Gazetteer};
//!
//! let gaz = GazetteerNominatim::new();
//! if let Some(loc) = gaz.lookup("Berlin") {
//!     println!("Berlin: {}, {}", loc.lat, loc.lon);
//! }
//! # }
//! ```

mod gazetteer;
mod geoparser;
mod mention;

pub use gazetteer::{BuiltinGazetteer, Gazetteer, GazetteerEntry, MultiGazetteer};

#[cfg(feature = "geocoding")]
pub use gazetteer::{GazetteerGeoNames, GazetteerNominatim, GazetteerWikidata};

pub use geoparser::GeoParser;
pub use mention::{LocationMention, LocationPattern, MentionType};
