//! CSV format import/export.

use super::format::Format;
use crate::core::{
    EventBuilder, Location, Narrative, NarrativeBuilder, SourceRef, SourceType, Timestamp,
};
use crate::{Error, Result};
use csv::StringRecord;
use std::io::{Read, Write};

/// CSV format handler.
///
/// This format handler can import and export narratives in CSV format.
/// The CSV must have latitude, longitude, and timestamp columns at minimum.
///
/// # Example
///
/// ```rust
/// use spatial_narrative::io::{CsvFormat, Format};
///
/// let format = CsvFormat::default();
///
/// let csv_data = "lat,lon,timestamp,text\n\
///                 40.7128,-74.006,2024-01-15T14:30:00Z,Something happened\n\
///                 34.0522,-118.2437,2024-01-16T10:00:00Z,Another event";
///
/// let narrative = format.import_str(csv_data).unwrap();
/// assert_eq!(narrative.events().len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct CsvFormat {
    /// Configuration options for CSV import/export
    pub options: CsvOptions,
}

/// Configuration options for CSV import/export.
#[derive(Debug, Clone)]
pub struct CsvOptions {
    /// Column name for latitude (defaults to "lat")
    pub lat_column: String,

    /// Column name for longitude (defaults to "lon")
    pub lon_column: String,

    /// Column name for timestamp (defaults to "timestamp")
    pub timestamp_column: String,

    /// Column name for elevation (optional)
    pub elevation_column: Option<String>,

    /// Column name for text/description (optional)
    pub text_column: Option<String>,

    /// Column name for tags (optional, comma-separated in cell)
    pub tags_column: Option<String>,

    /// Column name for source name (optional)
    pub source_name_column: Option<String>,

    /// Column name for source type (optional)
    pub source_type_column: Option<String>,

    /// Whether to include headers in exported CSV
    pub include_headers: bool,

    /// CSV delimiter character
    pub delimiter: u8,
}

impl Default for CsvOptions {
    fn default() -> Self {
        Self {
            lat_column: "lat".to_string(),
            lon_column: "lon".to_string(),
            timestamp_column: "timestamp".to_string(),
            elevation_column: Some("elevation".to_string()),
            text_column: Some("text".to_string()),
            tags_column: Some("tags".to_string()),
            source_name_column: Some("source".to_string()),
            source_type_column: Some("source_type".to_string()),
            include_headers: true,
            delimiter: b',',
        }
    }
}

impl Default for CsvFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl CsvFormat {
    /// Create a new CSV format handler with default options.
    pub fn new() -> Self {
        Self {
            options: CsvOptions::default(),
        }
    }

    /// Create a new CSV format handler with custom options.
    pub fn with_options(options: CsvOptions) -> Self {
        Self { options }
    }

    /// Helper to find column index by name
    fn find_column(&self, headers: &StringRecord, name: &str) -> Option<usize> {
        headers.iter().position(|h| h.eq_ignore_ascii_case(name))
    }

    /// Helper to get optional string value from record
    fn get_optional(&self, record: &StringRecord, index: Option<usize>) -> Option<String> {
        index.and_then(|i| record.get(i).filter(|s| !s.is_empty()).map(String::from))
    }
}

impl Format for CsvFormat {
    fn import<R: Read>(&self, reader: R) -> Result<Narrative> {
        let mut csv_reader = csv::ReaderBuilder::new()
            .delimiter(self.options.delimiter)
            .from_reader(reader);

        let headers = csv_reader.headers()?.clone();

        // Find required columns
        let lat_idx = self
            .find_column(&headers, &self.options.lat_column)
            .ok_or_else(|| {
                Error::InvalidFormat(format!(
                    "missing required column: {}",
                    self.options.lat_column
                ))
            })?;

        let lon_idx = self
            .find_column(&headers, &self.options.lon_column)
            .ok_or_else(|| {
                Error::InvalidFormat(format!(
                    "missing required column: {}",
                    self.options.lon_column
                ))
            })?;

        let ts_idx = self
            .find_column(&headers, &self.options.timestamp_column)
            .ok_or_else(|| {
                Error::InvalidFormat(format!(
                    "missing required column: {}",
                    self.options.timestamp_column
                ))
            })?;

        // Find optional columns
        let elev_idx = self
            .options
            .elevation_column
            .as_ref()
            .and_then(|col| self.find_column(&headers, col));

        let text_idx = self
            .options
            .text_column
            .as_ref()
            .and_then(|col| self.find_column(&headers, col));

        let tags_idx = self
            .options
            .tags_column
            .as_ref()
            .and_then(|col| self.find_column(&headers, col));

        let source_name_idx = self
            .options
            .source_name_column
            .as_ref()
            .and_then(|col| self.find_column(&headers, col));

        let source_type_idx = self
            .options
            .source_type_column
            .as_ref()
            .and_then(|col| self.find_column(&headers, col));

        let mut builder = NarrativeBuilder::new();

        // Process each record
        for (row_num, result) in csv_reader.records().enumerate() {
            let record = result?;

            // Parse required fields
            let lat: f64 = record
                .get(lat_idx)
                .ok_or_else(|| Error::InvalidFormat(format!("missing lat at row {}", row_num)))?
                .parse()
                .map_err(|_| Error::InvalidFormat(format!("invalid lat at row {}", row_num)))?;

            let lon: f64 = record
                .get(lon_idx)
                .ok_or_else(|| Error::InvalidFormat(format!("missing lon at row {}", row_num)))?
                .parse()
                .map_err(|_| Error::InvalidFormat(format!("invalid lon at row {}", row_num)))?;

            let ts_str = record.get(ts_idx).ok_or_else(|| {
                Error::InvalidFormat(format!("missing timestamp at row {}", row_num))
            })?;

            let timestamp = Timestamp::parse(ts_str).map_err(|_| {
                Error::InvalidFormat(format!("invalid timestamp at row {}", row_num))
            })?;

            // Build location
            let mut location = Location::new(lat, lon);
            if let Some(elev_str) = self.get_optional(&record, elev_idx) {
                if let Ok(elev) = elev_str.parse::<f64>() {
                    location.elevation = Some(elev);
                }
            }

            // Build event
            let mut event_builder = EventBuilder::new().location(location).timestamp(timestamp);

            // Add optional fields
            if let Some(text) = self.get_optional(&record, text_idx) {
                event_builder = event_builder.text(text);
            }

            if let Some(tags_str) = self.get_optional(&record, tags_idx) {
                for tag in tags_str.split(',') {
                    let trimmed = tag.trim();
                    if !trimmed.is_empty() {
                        event_builder = event_builder.tag(trimmed);
                    }
                }
            }

            if let Some(source_name) = self.get_optional(&record, source_name_idx) {
                let source_type = self
                    .get_optional(&record, source_type_idx)
                    .and_then(|s| match s.to_lowercase().as_str() {
                        "article" => Some(SourceType::Article),
                        "report" => Some(SourceType::Report),
                        "witness" => Some(SourceType::Witness),
                        "sensor" => Some(SourceType::Sensor),
                        _ => None,
                    })
                    .unwrap_or(SourceType::Article);

                let mut source = SourceRef::new(source_type);
                source.title = Some(source_name);
                event_builder = event_builder.source(source);
            }

            let event = event_builder.build();
            builder = builder.event(event);
        }

        Ok(builder.build())
    }

    fn export<W: Write>(&self, narrative: &Narrative, writer: W) -> Result<()> {
        let mut csv_writer = csv::WriterBuilder::new()
            .delimiter(self.options.delimiter)
            .from_writer(writer);

        // Write headers if enabled
        if self.options.include_headers {
            let mut headers = vec![
                self.options.lat_column.as_str(),
                self.options.lon_column.as_str(),
                self.options.timestamp_column.as_str(),
            ];

            if let Some(ref col) = self.options.elevation_column {
                headers.push(col);
            }
            if let Some(ref col) = self.options.text_column {
                headers.push(col);
            }
            if let Some(ref col) = self.options.tags_column {
                headers.push(col);
            }
            if let Some(ref col) = self.options.source_name_column {
                headers.push(col);
            }
            if let Some(ref col) = self.options.source_type_column {
                headers.push(col);
            }

            csv_writer.write_record(&headers)?;
        }

        // Write events
        for event in narrative.events() {
            let loc = &event.location;
            let mut record = vec![
                loc.lat.to_string(),
                loc.lon.to_string(),
                event.timestamp.to_rfc3339(),
            ];

            if self.options.elevation_column.is_some() {
                record.push(loc.elevation.map(|e| e.to_string()).unwrap_or_default());
            }

            if self.options.text_column.is_some() {
                record.push(event.text.clone());
            }

            if self.options.tags_column.is_some() {
                record.push(event.tags.join(", "));
            }

            if self.options.source_name_column.is_some() {
                record.push(
                    event
                        .sources
                        .first()
                        .and_then(|s| s.title.clone())
                        .unwrap_or_default(),
                );
            }

            if self.options.source_type_column.is_some() {
                let type_str = event
                    .sources
                    .first()
                    .map(|s| s.source_type.to_string())
                    .unwrap_or_default();
                record.push(type_str.to_string());
            }

            csv_writer.write_record(&record)?;
        }

        csv_writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Event;

    #[test]
    fn test_csv_import_basic() {
        let csv_data = "lat,lon,timestamp\n\
                       40.7128,-74.006,2024-01-15T14:30:00Z\n\
                       34.0522,-118.2437,2024-01-16T10:00:00Z";

        let format = CsvFormat::new();
        let narrative = format.import_str(csv_data).unwrap();

        assert_eq!(narrative.events().len(), 2);
        assert_eq!(narrative.events()[0].location.lat, 40.7128);
        assert_eq!(narrative.events()[1].location.lat, 34.0522);
    }

    #[test]
    fn test_csv_import_with_text() {
        let csv_data = "lat,lon,timestamp,text\n\
                       40.7128,-74.006,2024-01-15T14:30:00Z,Event in NYC\n\
                       34.0522,-118.2437,2024-01-16T10:00:00Z,Event in LA";

        let format = CsvFormat::new();
        let narrative = format.import_str(csv_data).unwrap();

        assert_eq!(narrative.events()[0].text, "Event in NYC");
        assert_eq!(narrative.events()[1].text, "Event in LA");
    }

    #[test]
    fn test_csv_roundtrip() {
        let event = Event::builder()
            .location(Location::new(40.7128, -74.006))
            .timestamp(Timestamp::parse("2024-01-15T14:30:00Z").unwrap())
            .text("Test event")
            .tag("tag1")
            .tag("tag2")
            .build();

        let narrative = Narrative::builder().event(event).build();

        let format = CsvFormat::new();
        let exported = format.export_str(&narrative).unwrap();
        let imported = format.import_str(&exported).unwrap();

        assert_eq!(imported.events().len(), 1);
        assert_eq!(imported.events()[0].text, "Test event");
        assert_eq!(imported.events()[0].tags.len(), 2);
    }

    #[test]
    fn test_csv_missing_required_column() {
        let csv_data = "latitude,longitude\n40.7128,-74.006";

        let format = CsvFormat::new();
        let result = format.import_str(csv_data);

        assert!(result.is_err());
    }

    #[test]
    fn test_csv_custom_delimiter() {
        let tsv_data = "lat\tlon\ttimestamp\n40.7128\t-74.006\t2024-01-15T14:30:00Z";

        let options = CsvOptions {
            delimiter: b'\t',
            ..Default::default()
        };
        let format = CsvFormat::with_options(options);
        let narrative = format.import_str(tsv_data).unwrap();

        assert_eq!(narrative.events().len(), 1);
    }
}
