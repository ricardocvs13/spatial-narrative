//! Event representation - something that happened at a place and time.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::core::{Location, SourceRef, Timestamp};
use crate::error::{Error, Result};

/// Unique identifier for an event.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventId(pub Uuid);

impl EventId {
    /// Creates a new random EventId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates an EventId from a UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Parses an EventId from a string.
    pub fn parse(s: &str) -> Result<Self> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| Error::ParseError(format!("invalid event ID: {}", s)))
    }

    /// Returns the inner UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for EventId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

/// An event in a spatial narrative.
///
/// Events are the fundamental unit of spatial narratives. Each event
/// represents something that happened at a specific place and time.
///
/// # Examples
///
/// ```
/// use spatial_narrative::core::{Event, Location, Timestamp, SourceRef};
///
/// let event = Event::builder()
///     .location(Location::new(40.7128, -74.0060))
///     .timestamp(Timestamp::now())
///     .text("Protest began at City Hall")
///     .tag("protest")
///     .tag("politics")
///     .source(SourceRef::article("https://news.example.com/protest"))
///     .metadata("participants", "5000")
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// Unique identifier.
    pub id: EventId,
    /// Geographic location.
    pub location: Location,
    /// When the event occurred.
    pub timestamp: Timestamp,
    /// Description of the event.
    pub text: String,
    /// Key-value metadata.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    /// References to source material.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<SourceRef>,
    /// Categorical tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl Event {
    /// Creates a new event with required fields.
    pub fn new(location: Location, timestamp: Timestamp, text: impl Into<String>) -> Self {
        Self {
            id: EventId::new(),
            location,
            timestamp,
            text: text.into(),
            metadata: HashMap::new(),
            sources: Vec::new(),
            tags: Vec::new(),
        }
    }

    /// Creates a builder for constructing an Event.
    pub fn builder() -> EventBuilder {
        EventBuilder::new()
    }

    /// Returns true if the event has the given tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// Adds a tag to the event.
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        let tag = tag.into();
        if !self.has_tag(&tag) {
            self.tags.push(tag);
        }
    }

    /// Removes a tag from the event.
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    /// Gets a metadata value.
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }

    /// Sets a metadata value.
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Adds a source reference.
    pub fn add_source(&mut self, source: SourceRef) {
        self.sources.push(source);
    }

    /// Returns the location as a geo-types Point.
    pub fn to_geo_point(&self) -> geo_types::Point<f64> {
        self.location.to_geo_point()
    }
}

/// Builder for constructing [`Event`] instances.
#[derive(Debug, Default)]
pub struct EventBuilder {
    id: Option<EventId>,
    location: Option<Location>,
    timestamp: Option<Timestamp>,
    text: Option<String>,
    metadata: HashMap<String, String>,
    sources: Vec<SourceRef>,
    tags: Vec<String>,
}

impl EventBuilder {
    /// Creates a new EventBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the event ID.
    pub fn id(mut self, id: EventId) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the location.
    pub fn location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    /// Sets the location from coordinates.
    pub fn coordinates(mut self, lat: f64, lon: f64) -> Self {
        self.location = Some(Location::new(lat, lon));
        self
    }

    /// Sets the timestamp.
    pub fn timestamp(mut self, timestamp: Timestamp) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// Sets the timestamp from a parseable string.
    pub fn timestamp_str(mut self, s: &str) -> Result<Self> {
        self.timestamp = Some(Timestamp::parse(s)?);
        Ok(self)
    }

    /// Sets the event text.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Adds a tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Adds multiple tags.
    pub fn tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tags.extend(tags.into_iter().map(Into::into));
        self
    }

    /// Adds a source reference.
    pub fn source(mut self, source: SourceRef) -> Self {
        self.sources.push(source);
        self
    }

    /// Adds multiple sources.
    pub fn sources(mut self, sources: impl IntoIterator<Item = SourceRef>) -> Self {
        self.sources.extend(sources);
        self
    }

    /// Sets a metadata value.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Builds the Event.
    ///
    /// Uses current time if timestamp is not set.
    /// Uses empty string if text is not set.
    pub fn build(self) -> Event {
        Event {
            id: self.id.unwrap_or_default(),
            location: self.location.unwrap_or_default(),
            timestamp: self.timestamp.unwrap_or_else(Timestamp::now),
            text: self.text.unwrap_or_default(),
            metadata: self.metadata,
            sources: self.sources,
            tags: self.tags,
        }
    }

    /// Builds the Event, returning an error if required fields are missing.
    pub fn try_build(self) -> Result<Event> {
        let location = self.location.ok_or(Error::MissingField("location"))?;
        let timestamp = self.timestamp.ok_or(Error::MissingField("timestamp"))?;
        let text = self.text.ok_or(Error::MissingField("text"))?;

        Ok(Event {
            id: self.id.unwrap_or_default(),
            location,
            timestamp,
            text,
            metadata: self.metadata,
            sources: self.sources,
            tags: self.tags,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_id_new() {
        let id1 = EventId::new();
        let id2 = EventId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_event_id_parse() {
        let id = EventId::new();
        let parsed = EventId::parse(&id.to_string()).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_event_new() {
        let event = Event::new(
            Location::new(40.7128, -74.0060),
            Timestamp::now(),
            "Test event",
        );
        assert_eq!(event.text, "Test event");
        assert!(event.tags.is_empty());
    }

    #[test]
    fn test_event_builder() {
        let event = Event::builder()
            .location(Location::new(40.7128, -74.0060))
            .timestamp(Timestamp::now())
            .text("Protest at City Hall")
            .tag("protest")
            .tag("politics")
            .metadata("participants", "1000")
            .source(SourceRef::article("https://example.com"))
            .build();

        assert_eq!(event.text, "Protest at City Hall");
        assert!(event.has_tag("protest"));
        assert!(event.has_tag("politics"));
        assert_eq!(event.get_metadata("participants"), Some("1000"));
        assert_eq!(event.sources.len(), 1);
    }

    #[test]
    fn test_event_try_build_missing_fields() {
        let result = Event::builder().try_build();
        assert!(result.is_err());
    }

    #[test]
    fn test_event_tags() {
        let mut event = Event::new(Location::new(0.0, 0.0), Timestamp::now(), "Test");

        event.add_tag("tag1");
        event.add_tag("tag2");
        event.add_tag("tag1"); // Duplicate should be ignored

        assert_eq!(event.tags.len(), 2);
        assert!(event.has_tag("tag1"));
        assert!(event.has_tag("tag2"));

        event.remove_tag("tag1");
        assert!(!event.has_tag("tag1"));
    }

    #[test]
    fn test_event_metadata() {
        let mut event = Event::builder()
            .location(Location::new(0.0, 0.0))
            .timestamp(Timestamp::now())
            .text("Test")
            .metadata("key1", "value1")
            .build();

        assert_eq!(event.get_metadata("key1"), Some("value1"));
        assert_eq!(event.get_metadata("key2"), None);

        event.set_metadata("key2", "value2");
        assert_eq!(event.get_metadata("key2"), Some("value2"));
    }

    #[test]
    fn test_event_serialization() {
        let event = Event::builder()
            .location(Location::new(40.7128, -74.0060))
            .timestamp(Timestamp::parse("2024-03-15T14:30:00Z").unwrap())
            .text("Test event")
            .tag("test")
            .build();

        let json = serde_json::to_string(&event).unwrap();
        let parsed: Event = serde_json::from_str(&json).unwrap();

        assert_eq!(event.text, parsed.text);
        assert_eq!(event.location.lat, parsed.location.lat);
    }
}
