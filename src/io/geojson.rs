//! GeoJSON format import/export.

use super::format::Format;
use crate::core::{
    EventBuilder, Location, Narrative, NarrativeBuilder, SourceRef, SourceType, Timestamp,
};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::io::{Read, Write};

/// GeoJSON format handler.
///
/// This format handler can import and export narratives in GeoJSON format,
/// storing events as Features with Point geometries. Temporal and metadata
/// information is stored in feature properties.
///
/// # Example
///
/// ```rust
/// use spatial_narrative::io::{GeoJsonFormat, Format};
/// use spatial_narrative::prelude::*;
///
/// let format = GeoJsonFormat::default();
///
/// // Import from GeoJSON
/// let geojson = r#"{
///   "type": "FeatureCollection",
///   "features": [
///     {
///       "type": "Feature",
///       "geometry": {
///         "type": "Point",
///         "coordinates": [-74.006, 40.7128]
///       },
///       "properties": {
///         "text": "Something happened",
///         "timestamp": "2024-01-15T14:30:00Z"
///       }
///     }
///   ]
/// }"#;
///
/// let narrative = format.import_str(geojson).unwrap();
/// assert_eq!(narrative.events().len(), 1);
/// ```
#[derive(Debug, Clone, Default)]
pub struct GeoJsonFormat {
    /// Options for import/export behavior
    pub options: GeoJsonOptions,
}

/// Configuration options for GeoJSON import/export.
#[derive(Debug, Clone)]
pub struct GeoJsonOptions {
    /// Whether to include event IDs in exported GeoJSON
    pub include_ids: bool,

    /// Whether to include tags in exported GeoJSON
    pub include_tags: bool,

    /// Whether to include source references in exported GeoJSON
    pub include_sources: bool,

    /// Property name for timestamp field
    pub timestamp_property: String,

    /// Property name for text/description field
    pub text_property: String,
}

impl Default for GeoJsonOptions {
    fn default() -> Self {
        Self {
            include_ids: true,
            include_tags: true,
            include_sources: true,
            timestamp_property: "timestamp".to_string(),
            text_property: "text".to_string(),
        }
    }
}

impl GeoJsonFormat {
    /// Create a new GeoJSON format handler with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new GeoJSON format handler with custom options.
    pub fn with_options(options: GeoJsonOptions) -> Self {
        Self { options }
    }
}

/// Internal structure for GeoJSON FeatureCollection
#[derive(Debug, Serialize, Deserialize)]
struct FeatureCollection {
    #[serde(rename = "type")]
    type_: String,
    features: Vec<Feature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Map<String, Value>>,
}

/// Internal structure for GeoJSON Feature
#[derive(Debug, Serialize, Deserialize)]
struct Feature {
    #[serde(rename = "type")]
    type_: String,
    geometry: Geometry,
    properties: Map<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Value>,
}

/// Internal structure for GeoJSON Geometry
#[derive(Debug, Serialize, Deserialize)]
struct Geometry {
    #[serde(rename = "type")]
    type_: String,
    coordinates: Vec<f64>,
}

impl Format for GeoJsonFormat {
    fn import<R: Read>(&self, reader: R) -> Result<Narrative> {
        let fc: FeatureCollection = serde_json::from_reader(reader)?;

        if fc.type_ != "FeatureCollection" {
            return Err(Error::InvalidFormat(
                "expected GeoJSON FeatureCollection".to_string(),
            ));
        }

        let mut builder = NarrativeBuilder::new();

        // Extract narrative-level metadata from FeatureCollection properties
        if let Some(props) = fc.properties {
            if let Some(title) = props.get("title").and_then(|v| v.as_str()) {
                builder = builder.title(title);
            }
            if let Some(desc) = props.get("description").and_then(|v| v.as_str()) {
                builder = builder.description(desc);
            }
        }

        // Convert each feature to an event
        for feature in fc.features {
            if feature.geometry.type_ != "Point" {
                continue; // Skip non-point geometries
            }

            let coords = &feature.geometry.coordinates;
            if coords.len() < 2 {
                continue; // Invalid coordinates
            }

            let lon = coords[0];
            let lat = coords[1];
            let mut location = Location::new(lat, lon);
            if let Some(elev) = coords.get(2).copied() {
                location.elevation = Some(elev);
            }

            let props = &feature.properties;

            // Extract timestamp
            let timestamp = if let Some(ts_str) = props
                .get(&self.options.timestamp_property)
                .and_then(|v| v.as_str())
            {
                Timestamp::parse(ts_str)
                    .map_err(|e| Error::InvalidFormat(format!("invalid timestamp: {}", e)))?
            } else {
                Timestamp::now() // Default to current time if not specified
            };

            // Build the event
            let mut event_builder = EventBuilder::new().location(location).timestamp(timestamp);

            // Extract text/description
            if let Some(text) = props
                .get(&self.options.text_property)
                .and_then(|v| v.as_str())
            {
                event_builder = event_builder.text(text);
            }

            // Extract tags
            if let Some(tags) = props.get("tags").and_then(|v| v.as_array()) {
                for tag in tags {
                    if let Some(tag_str) = tag.as_str() {
                        event_builder = event_builder.tag(tag_str);
                    }
                }
            }

            // Extract source
            if let Some(source_obj) = props.get("source").and_then(|v| v.as_object()) {
                let source_type = source_obj
                    .get("type")
                    .and_then(|v| v.as_str())
                    .and_then(|s| match s.to_lowercase().as_str() {
                        "article" => Some(SourceType::Article),
                        "report" => Some(SourceType::Report),
                        "witness" => Some(SourceType::Witness),
                        "sensor" => Some(SourceType::Sensor),
                        _ => None,
                    })
                    .unwrap_or(SourceType::Article);

                let mut source = SourceRef::new(source_type);
                if let Some(url) = source_obj.get("url").and_then(|v| v.as_str()) {
                    source.url = Some(url.to_string());
                }
                if let Some(title) = source_obj.get("title").and_then(|v| v.as_str()) {
                    source.title = Some(title.to_string());
                }
                event_builder = event_builder.source(source);
            }

            let event = event_builder.build();
            builder = builder.event(event);
        }

        Ok(builder.build())
    }

    fn export<W: Write>(&self, narrative: &Narrative, mut writer: W) -> Result<()> {
        let mut features = Vec::new();

        for event in narrative.events() {
            let loc = &event.location;
            let coords = if let Some(elev) = loc.elevation {
                vec![loc.lon, loc.lat, elev]
            } else {
                vec![loc.lon, loc.lat]
            };

            let geometry = Geometry {
                type_: "Point".to_string(),
                coordinates: coords,
            };

            let mut properties = Map::new();

            // Add timestamp
            properties.insert(
                self.options.timestamp_property.clone(),
                Value::String(event.timestamp.to_rfc3339()),
            );

            // Add text if present
            properties.insert(
                self.options.text_property.clone(),
                Value::String(event.text.clone()),
            );

            // Add tags if enabled and present
            if self.options.include_tags && !event.tags.is_empty() {
                let tags: Vec<Value> = event
                    .tags
                    .iter()
                    .map(|t| Value::String(t.clone()))
                    .collect();
                properties.insert("tags".to_string(), Value::Array(tags));
            }

            // Add source if enabled and present
            if self.options.include_sources && !event.sources.is_empty() {
                let source = &event.sources[0]; // Use first source
                let mut source_obj = Map::new();
                source_obj.insert(
                    "type".to_string(),
                    Value::String(source.source_type.to_string()),
                );
                if let Some(url) = &source.url {
                    source_obj.insert("url".to_string(), Value::String(url.clone()));
                }
                if let Some(title) = &source.title {
                    source_obj.insert("title".to_string(), Value::String(title.clone()));
                }
                properties.insert("source".to_string(), Value::Object(source_obj));
            }

            let feature = Feature {
                type_: "Feature".to_string(),
                geometry,
                properties,
                id: if self.options.include_ids {
                    Some(Value::String(event.id.to_string()))
                } else {
                    None
                },
            };

            features.push(feature);
        }

        // Add narrative-level metadata
        let mut fc_properties = Map::new();
        fc_properties.insert("title".to_string(), Value::String(narrative.title.clone()));
        if let Some(desc) = &narrative.metadata.description {
            fc_properties.insert("description".to_string(), Value::String(desc.clone()));
        }

        let fc = FeatureCollection {
            type_: "FeatureCollection".to_string(),
            features,
            properties: if fc_properties.is_empty() {
                None
            } else {
                Some(fc_properties)
            },
        };

        serde_json::to_writer_pretty(&mut writer, &fc)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Event;

    #[test]
    fn test_geojson_import_basic() {
        let geojson = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "geometry": {
                        "type": "Point",
                        "coordinates": [-74.006, 40.7128]
                    },
                    "properties": {
                        "text": "Event at NYC",
                        "timestamp": "2024-01-15T14:30:00Z"
                    }
                }
            ]
        }"#;

        let format = GeoJsonFormat::new();
        let narrative = format.import_str(geojson).unwrap();

        assert_eq!(narrative.events().len(), 1);
        let event = &narrative.events()[0];
        assert_eq!(event.location.lat, 40.7128);
        assert_eq!(event.location.lon, -74.006);
        assert_eq!(event.text.as_str(), "Event at NYC");
    }

    #[test]
    fn test_geojson_roundtrip() {
        let event = Event::builder()
            .location(Location::new(40.7128, -74.006))
            .timestamp(Timestamp::parse("2024-01-15T14:30:00Z").unwrap())
            .text("Test event")
            .tag("test")
            .build();

        let narrative = Narrative::builder()
            .title("Test Narrative")
            .event(event)
            .build();

        let format = GeoJsonFormat::new();
        let exported = format.export_str(&narrative).unwrap();
        let imported = format.import_str(&exported).unwrap();

        assert_eq!(imported.events().len(), 1);
        assert_eq!(imported.title, "Test Narrative");
    }

    #[test]
    fn test_geojson_with_elevation() {
        let geojson = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "geometry": {
                        "type": "Point",
                        "coordinates": [-122.4194, 37.7749, 100.5]
                    },
                    "properties": {
                        "timestamp": "2024-01-15T14:30:00Z"
                    }
                }
            ]
        }"#;

        let format = GeoJsonFormat::new();
        let narrative = format.import_str(geojson).unwrap();

        let event = &narrative.events()[0];
        assert_eq!(event.location.elevation, Some(100.5));
    }
}
