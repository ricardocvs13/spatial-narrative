//! Format trait for import/export operations.

use crate::core::Narrative;
use crate::Result;
use std::io::{Read, Write};

/// Trait for formats that can import and export narratives.
///
/// This trait defines a common interface for reading and writing
/// narratives in various formats (GeoJSON, CSV, etc.).
pub trait Format {
    /// Import a narrative from a reader.
    ///
    /// # Errors
    ///
    /// Returns an error if the data is malformed or doesn't match
    /// the expected format.
    fn import<R: Read>(&self, reader: R) -> Result<Narrative>;

    /// Import a narrative from a string.
    ///
    /// This is a convenience method that wraps the string in a reader.
    fn import_str(&self, data: &str) -> Result<Narrative> {
        self.import(data.as_bytes())
    }

    /// Export a narrative to a writer.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails.
    fn export<W: Write>(&self, narrative: &Narrative, writer: W) -> Result<()>;

    /// Export a narrative to a string.
    ///
    /// This is a convenience method that collects output into a String.
    fn export_str(&self, narrative: &Narrative) -> Result<String> {
        let mut buffer = Vec::new();
        self.export(narrative, &mut buffer)?;
        Ok(String::from_utf8(buffer).expect("format produced invalid UTF-8"))
    }
}
