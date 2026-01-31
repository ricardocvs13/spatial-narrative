//! Custom JSON format for narratives.
//!
//! This format is optimized for storing and exchanging complete narratives
//! with all metadata preserved.

use super::format::Format;
use crate::core::{
    Event, Location, Narrative, NarrativeMetadata, SourceRef, SourceType, Timestamp,
};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// Custom JSON format handler.
///
/// This format preserves all narrative structure and metadata, making it
/// ideal for storing narratives in databases or exchanging between systems.
///
/// # Example
///
/// ```rust
/// use spatial_narrative::io::{JsonFormat, Format};
/// use spatial_narrative::prelude::*;
///
/// let format = JsonFormat::new();
///
/// let event = Event::builder()
///     .location(Location::new(40.7128, -74.006))
///     .timestamp(Timestamp::now())
///     .text("Something happened")
///     .build();
///
/// let narrative = Narrative::builder()
///     .title("My Story")
///     .event(event)
///     .build();
///
/// let json = format.export_str(&narrative).unwrap();
/// let restored = format.import_str(&json).unwrap();
///
/// assert_eq!(restored.events().len(), 1);
/// ```
#[derive(Debug, Clone, Default)]
pub struct JsonFormat {
    /// Whether to pretty-print the JSON output
    pub pretty: bool,
}

impl JsonFormat {
    /// Create a new JSON format handler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new JSON format handler with pretty printing enabled.
    pub fn pretty() -> Self {
        Self { pretty: true }
    }
}

/// JSON representation of a narrative with version info.
#[derive(Debug, Serialize, Deserialize)]
struct NarrativeJson {
    /// Format version for future compatibility
    version: String,

    /// Narrative metadata
    metadata: NarrativeMetadataJson,

    /// Events in the narrative
    events: Vec<EventJson>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NarrativeMetadataJson {
    created: Option<String>,
    modified: Option<String>,
    author: Option<String>,
    description: Option<String>,
    category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EventJson {
    id: String,
    location: LocationJson,
    timestamp: String,
    text: String,
    tags: Vec<String>,
    #[serde(default)]
    sources: Vec<SourceRefJson>,
    metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct LocationJson {
    lat: f64,
    lon: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    elevation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uncertainty_meters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SourceRefJson {
    source_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
}

impl Format for JsonFormat {
    fn import<R: Read>(&self, reader: R) -> Result<Narrative> {
        let json: NarrativeJson = serde_json::from_reader(reader)?;

        // Check version compatibility (for now, we only support 1.0)
        if !json.version.starts_with("1.") {
            return Err(Error::InvalidFormat(format!(
                "unsupported format version: {}",
                json.version
            )));
        }

        // Convert from JSON representation to internal types
        let metadata = NarrativeMetadata {
            created: json
                .metadata
                .created
                .as_ref()
                .map(|s| Timestamp::parse(s))
                .transpose()?,
            modified: json
                .metadata
                .modified
                .as_ref()
                .map(|s| Timestamp::parse(s))
                .transpose()?,
            author: json.metadata.author,
            description: json.metadata.description,
            category: json.metadata.category,
            extra: std::collections::HashMap::new(),
        };

        let mut events = Vec::new();

        for event_json in json.events {
            let location = Location {
                lat: event_json.location.lat,
                lon: event_json.location.lon,
                elevation: event_json.location.elevation,
                uncertainty_meters: event_json.location.uncertainty_meters,
                name: event_json.location.name,
            };

            // Validate location
            location.validate()?;

            let timestamp = Timestamp::parse(&event_json.timestamp)?;

            let sources: Vec<SourceRef> = event_json
                .sources
                .into_iter()
                .map(|s| {
                    let source_type = match s.source_type.as_str() {
                        "article" => SourceType::Article,
                        "report" => SourceType::Report,
                        "witness" => SourceType::Witness,
                        "sensor" => SourceType::Sensor,
                        _ => SourceType::Other,
                    };

                    SourceRef {
                        source_type,
                        url: s.url,
                        title: s.title,
                        author: s.author,
                        date: s.date.and_then(|d| Timestamp::parse(&d).ok()),
                        notes: None,
                    }
                })
                .collect();

            let event = Event {
                id: crate::core::EventId::parse(&event_json.id)?,
                location,
                timestamp,
                text: event_json.text,
                tags: event_json.tags,
                sources,
                metadata: serde_json::from_value(event_json.metadata).unwrap_or_default(),
            };
            events.push(event);
        }

        Ok(Narrative {
            id: crate::core::NarrativeId::new(),
            title: "Imported Narrative".to_string(),
            events,
            metadata,
            tags: Vec::new(),
        })
    }

    fn export<W: Write>(&self, narrative: &Narrative, writer: W) -> Result<()> {
        let metadata = NarrativeMetadataJson {
            created: narrative.metadata.created.as_ref().map(|t| t.to_rfc3339()),
            modified: narrative.metadata.modified.as_ref().map(|t| t.to_rfc3339()),
            author: narrative.metadata.author.clone(),
            description: narrative.metadata.description.clone(),
            category: narrative.metadata.category.clone(),
        };

        let events: Vec<EventJson> = narrative
            .events
            .iter()
            .map(|event| {
                let location = LocationJson {
                    lat: event.location.lat,
                    lon: event.location.lon,
                    elevation: event.location.elevation,
                    uncertainty_meters: event.location.uncertainty_meters,
                    name: event.location.name.clone(),
                };

                EventJson {
                    id: event.id.to_string(),
                    location,
                    timestamp: event.timestamp.to_rfc3339(),
                    text: event.text.clone(),
                    tags: event.tags.clone(),
                    sources: event
                        .sources
                        .iter()
                        .map(|s| SourceRefJson {
                            source_type: s.source_type.to_string(),
                            title: s.title.clone(),
                            author: s.author.clone(),
                            url: s.url.clone(),
                            date: s.date.as_ref().map(|ts| ts.to_rfc3339()),
                        })
                        .collect(),
                    metadata: serde_json::to_value(&event.metadata)
                        .unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
                }
            })
            .collect();

        let json = NarrativeJson {
            version: "1.0".to_string(),
            metadata,
            events,
        };

        if self.pretty {
            serde_json::to_writer_pretty(writer, &json)?;
        } else {
            serde_json::to_writer(writer, &json)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_roundtrip() {
        let event = Event::builder()
            .location(Location::new(40.7128, -74.006))
            .timestamp(Timestamp::parse("2024-01-15T14:30:00Z").unwrap())
            .text("Test event")
            .tag("tag1")
            .build();

        let narrative = Narrative::builder()
            .title("Test Narrative")
            .description("A test narrative")
            .event(event)
            .build();

        let format = JsonFormat::pretty();
        let json = format.export_str(&narrative).unwrap();
        let restored = format.import_str(&json).unwrap();

        assert_eq!(restored.events().len(), 1);
        assert_eq!(restored.events()[0].text, "Test event");
        assert_eq!(restored.events()[0].tags, vec!["tag1"]);
    }

    #[test]
    fn test_json_version_check() {
        let json = r#"{
            "version": "2.0",
            "metadata": {
                "id": "00000000-0000-0000-0000-000000000000",
                "title": null,
                "description": null,
                "created_at": "2024-01-15T14:30:00Z",
                "updated_at": "2024-01-15T14:30:00Z",
                "tags": []
            },
            "events": []
        }"#;

        let format = JsonFormat::new();
        let result = format.import_str(json);

        assert!(result.is_err());
    }

    #[test]
    fn test_json_with_source() {
        let mut source = SourceRef::new(SourceType::Article);
        source.title = Some("Test Source".to_string());
        source.url = Some("https://example.com".to_string());

        let event = Event::builder()
            .location(Location::new(40.7128, -74.006))
            .timestamp(Timestamp::parse("2024-01-15T14:30:00Z").unwrap())
            .source(source)
            .build();

        let narrative = Narrative::builder().event(event).build();

        let format = JsonFormat::new();
        let json = format.export_str(&narrative).unwrap();
        let restored = format.import_str(&json).unwrap();

        let restored_event = &restored.events()[0];
        assert!(!restored_event.sources.is_empty());
        assert_eq!(
            restored_event.sources[0].title,
            Some("Test Source".to_string())
        );
        assert_eq!(
            restored_event.sources[0].url,
            Some("https://example.com".to_string())
        );
    }
}
