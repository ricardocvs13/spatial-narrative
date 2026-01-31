//! Import and export of narratives in various formats.
//!
//! This module provides converters for reading and writing
//! narratives in standard geographic data formats.
//!
//! # Supported Formats
//!
//! - [`GeoJsonFormat`] - Standard geographic data format
//! - [`CsvFormat`] - Tabular data with configurable columns
//! - [`JsonFormat`] - Custom JSON format optimized for narratives
//! - GPX - GPS exchange format (optional feature, TODO)
//!
//! # Example
//!
//! ```rust
//! use spatial_narrative::io::{GeoJsonFormat, CsvFormat, JsonFormat, Format};
//! use spatial_narrative::prelude::*;
//!
//! let narrative = Narrative::builder()
//!     .title("My Story")
//!     .event(Event::builder()
//!         .location(Location::new(40.7128, -74.006))
//!         .timestamp(Timestamp::now())
//!         .text("Something happened")
//!         .build())
//!     .build();
//!
//! // Export to GeoJSON
//! let geojson_format = GeoJsonFormat::new();
//! let geojson = geojson_format.export_str(&narrative).unwrap();
//!
//! // Export to CSV
//! let csv_format = CsvFormat::new();
//! let csv = csv_format.export_str(&narrative).unwrap();
//!
//! // Export to custom JSON
//! let json_format = JsonFormat::pretty();
//! let json = json_format.export_str(&narrative).unwrap();
//! ```

mod csv_format;
mod format;
mod geojson;
mod json_format;

pub use csv_format::{CsvFormat, CsvOptions};
pub use format::Format;
pub use geojson::{GeoJsonFormat, GeoJsonOptions};
pub use json_format::JsonFormat;
