//! Narrative - a collection of related events forming a coherent story.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::core::{Event, EventId, GeoBounds, TimeRange, Timestamp};
use crate::error::{Error, Result};

/// Unique identifier for a narrative.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NarrativeId(pub Uuid);

impl NarrativeId {
    /// Creates a new random NarrativeId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a NarrativeId from a UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Parses a NarrativeId from a string.
    pub fn parse(s: &str) -> Result<Self> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| Error::ParseError(format!("invalid narrative ID: {}", s)))
    }

    /// Returns the inner UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for NarrativeId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for NarrativeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Metadata associated with a narrative.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct NarrativeMetadata {
    /// When the narrative was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,
    /// When the narrative was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<Timestamp>,
    /// Author or creator of the narrative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Description of the narrative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Category or type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Additional key-value metadata.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, String>,
}

impl NarrativeMetadata {
    /// Creates empty metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates metadata with creation timestamp set to now.
    pub fn with_created_now() -> Self {
        Self {
            created: Some(Timestamp::now()),
            ..Default::default()
        }
    }
}

/// A collection of related events forming a coherent story.
///
/// Narratives are the primary container for organizing events
/// into meaningful stories with geographic and temporal structure.
///
/// # Examples
///
/// ```
/// use spatial_narrative::core::{Narrative, Event, Location, Timestamp};
///
/// let mut narrative = Narrative::builder()
///     .title("Hurricane Timeline")
///     .description("Tracking the hurricane's path")
///     .category("disaster")
///     .build();
///
/// narrative.add_event(Event::builder()
///     .location(Location::new(25.0, -80.0))
///     .timestamp(Timestamp::now())
///     .text("Hurricane makes landfall")
///     .tag("landfall")
///     .build());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Narrative {
    /// Unique identifier.
    pub id: NarrativeId,
    /// Title of the narrative.
    pub title: String,
    /// Events in this narrative.
    #[serde(default)]
    pub events: Vec<Event>,
    /// Narrative metadata.
    #[serde(default)]
    pub metadata: NarrativeMetadata,
    /// Categorical tags for the narrative.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl Narrative {
    /// Creates a new narrative with the given title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: NarrativeId::new(),
            title: title.into(),
            events: Vec::new(),
            metadata: NarrativeMetadata::with_created_now(),
            tags: Vec::new(),
        }
    }

    /// Creates a builder for constructing a Narrative.
    pub fn builder() -> NarrativeBuilder {
        NarrativeBuilder::new()
    }

    /// Returns the events in this narrative.
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    /// Returns a mutable reference to the events.
    pub fn events_mut(&mut self) -> &mut Vec<Event> {
        &mut self.events
    }

    /// Returns the number of events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns true if the narrative has no events.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Adds an event to the narrative.
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
        self.metadata.modified = Some(Timestamp::now());
    }

    /// Removes an event by ID.
    pub fn remove_event(&mut self, id: &EventId) -> Option<Event> {
        if let Some(pos) = self.events.iter().position(|e| &e.id == id) {
            self.metadata.modified = Some(Timestamp::now());
            Some(self.events.remove(pos))
        } else {
            None
        }
    }

    /// Finds an event by ID.
    pub fn get_event(&self, id: &EventId) -> Option<&Event> {
        self.events.iter().find(|e| &e.id == id)
    }

    /// Finds an event by ID (mutable).
    pub fn get_event_mut(&mut self, id: &EventId) -> Option<&mut Event> {
        self.events.iter_mut().find(|e| &e.id == id)
    }

    /// Returns events sorted by timestamp.
    pub fn events_chronological(&self) -> Vec<&Event> {
        let mut events: Vec<_> = self.events.iter().collect();
        events.sort_by_key(|e| &e.timestamp);
        events
    }

    /// Filters events by spatial bounds.
    pub fn filter_spatial(&self, bounds: &GeoBounds) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| bounds.contains(&e.location))
            .collect()
    }

    /// Filters events by time range.
    pub fn filter_temporal(&self, range: &TimeRange) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| range.contains(&e.timestamp))
            .collect()
    }

    /// Filters events by tag.
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&Event> {
        self.events.iter().filter(|e| e.has_tag(tag)).collect()
    }

    /// Returns the geographic bounds of all events.
    pub fn bounds(&self) -> Option<GeoBounds> {
        let locations: Vec<_> = self.events.iter().map(|e| &e.location).collect();
        GeoBounds::from_locations(locations)
    }

    /// Returns the time range spanning all events.
    pub fn time_range(&self) -> Option<TimeRange> {
        if self.events.is_empty() {
            return None;
        }

        let mut min_ts = &self.events[0].timestamp;
        let mut max_ts = &self.events[0].timestamp;

        for event in &self.events {
            if event.timestamp < *min_ts {
                min_ts = &event.timestamp;
            }
            if event.timestamp > *max_ts {
                max_ts = &event.timestamp;
            }
        }

        Some(TimeRange::new(min_ts.clone(), max_ts.clone()))
    }

    /// Returns all unique tags used by events in this narrative.
    pub fn all_tags(&self) -> Vec<&str> {
        let mut tags: Vec<_> = self
            .events
            .iter()
            .flat_map(|e| e.tags.iter().map(|s| s.as_str()))
            .collect();
        tags.sort();
        tags.dedup();
        tags
    }

    /// Adds a tag to the narrative.
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        let tag = tag.into();
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Creates a new narrative containing only events that match the predicate.
    pub fn filter<F>(&self, predicate: F) -> Narrative
    where
        F: Fn(&Event) -> bool,
    {
        let events = self
            .events
            .iter()
            .filter(|e| predicate(e))
            .cloned()
            .collect();
        Narrative {
            id: NarrativeId::new(),
            title: format!("{} (filtered)", self.title),
            events,
            metadata: NarrativeMetadata::with_created_now(),
            tags: self.tags.clone(),
        }
    }

    /// Merges another narrative into this one.
    pub fn merge(&mut self, other: Narrative) {
        self.events.extend(other.events);
        self.metadata.modified = Some(Timestamp::now());
    }
}

impl Default for Narrative {
    fn default() -> Self {
        Self::new("Untitled")
    }
}

/// Builder for constructing [`Narrative`] instances.
#[derive(Debug, Default)]
pub struct NarrativeBuilder {
    id: Option<NarrativeId>,
    title: Option<String>,
    events: Vec<Event>,
    metadata: NarrativeMetadata,
    tags: Vec<String>,
}

impl NarrativeBuilder {
    /// Creates a new NarrativeBuilder.
    pub fn new() -> Self {
        Self {
            metadata: NarrativeMetadata::with_created_now(),
            ..Default::default()
        }
    }

    /// Sets the narrative ID.
    pub fn id(mut self, id: NarrativeId) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Adds an event.
    pub fn event(mut self, event: Event) -> Self {
        self.events.push(event);
        self
    }

    /// Adds multiple events.
    pub fn events(mut self, events: impl IntoIterator<Item = Event>) -> Self {
        self.events.extend(events);
        self
    }

    /// Sets the author.
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.metadata.author = Some(author.into());
        self
    }

    /// Sets the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.metadata.description = Some(description.into());
        self
    }

    /// Sets the category.
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.metadata.category = Some(category.into());
        self
    }

    /// Adds a metadata key-value pair.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.extra.insert(key.into(), value.into());
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

    /// Builds the Narrative.
    pub fn build(self) -> Narrative {
        Narrative {
            id: self.id.unwrap_or_default(),
            title: self.title.unwrap_or_else(|| "Untitled".to_string()),
            events: self.events,
            metadata: self.metadata,
            tags: self.tags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Location;

    fn make_event(lat: f64, lon: f64, time: &str, text: &str) -> Event {
        Event::builder()
            .location(Location::new(lat, lon))
            .timestamp(Timestamp::parse(time).unwrap())
            .text(text)
            .build()
    }

    #[test]
    fn test_narrative_new() {
        let narrative = Narrative::new("Test Narrative");
        assert_eq!(narrative.title, "Test Narrative");
        assert!(narrative.is_empty());
    }

    #[test]
    fn test_narrative_builder() {
        let narrative = Narrative::builder()
            .title("Hurricane Timeline")
            .description("Tracking the storm")
            .author("Weather Service")
            .category("disaster")
            .tag("weather")
            .build();

        assert_eq!(narrative.title, "Hurricane Timeline");
        assert_eq!(
            narrative.metadata.description,
            Some("Tracking the storm".to_string())
        );
        assert!(narrative.tags.contains(&"weather".to_string()));
    }

    #[test]
    fn test_narrative_add_events() {
        let mut narrative = Narrative::new("Test");

        narrative.add_event(make_event(40.0, -74.0, "2024-03-15T10:00:00Z", "Event 1"));
        narrative.add_event(make_event(41.0, -73.0, "2024-03-15T11:00:00Z", "Event 2"));

        assert_eq!(narrative.len(), 2);
    }

    #[test]
    fn test_narrative_filter_spatial() {
        let mut narrative = Narrative::new("Test");
        narrative.add_event(make_event(40.0, -74.0, "2024-03-15T10:00:00Z", "NYC"));
        narrative.add_event(make_event(34.0, -118.0, "2024-03-15T11:00:00Z", "LA"));

        let nyc_bounds = GeoBounds::new(39.0, -75.0, 41.0, -73.0);
        let filtered = narrative.filter_spatial(&nyc_bounds);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].text, "NYC");
    }

    #[test]
    fn test_narrative_filter_temporal() {
        let mut narrative = Narrative::new("Test");
        narrative.add_event(make_event(40.0, -74.0, "2024-03-15T10:00:00Z", "March"));
        narrative.add_event(make_event(40.0, -74.0, "2024-04-15T10:00:00Z", "April"));

        let march = TimeRange::month(2024, 3);
        let filtered = narrative.filter_temporal(&march);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].text, "March");
    }

    #[test]
    fn test_narrative_bounds() {
        let mut narrative = Narrative::new("Test");
        narrative.add_event(make_event(40.0, -74.0, "2024-03-15T10:00:00Z", "NYC"));
        narrative.add_event(make_event(34.0, -118.0, "2024-03-15T11:00:00Z", "LA"));

        let bounds = narrative.bounds().unwrap();
        assert_eq!(bounds.min_lat, 34.0);
        assert_eq!(bounds.max_lat, 40.0);
        assert_eq!(bounds.min_lon, -118.0);
        assert_eq!(bounds.max_lon, -74.0);
    }

    #[test]
    fn test_narrative_time_range() {
        let mut narrative = Narrative::new("Test");
        narrative.add_event(make_event(40.0, -74.0, "2024-03-15T10:00:00Z", "First"));
        narrative.add_event(make_event(40.0, -74.0, "2024-03-20T10:00:00Z", "Last"));

        let range = narrative.time_range().unwrap();
        let duration = range.duration();
        assert_eq!(duration.num_days(), 5);
    }

    #[test]
    fn test_narrative_events_chronological() {
        let mut narrative = Narrative::new("Test");
        narrative.add_event(make_event(40.0, -74.0, "2024-03-20T10:00:00Z", "Third"));
        narrative.add_event(make_event(40.0, -74.0, "2024-03-10T10:00:00Z", "First"));
        narrative.add_event(make_event(40.0, -74.0, "2024-03-15T10:00:00Z", "Second"));

        let sorted = narrative.events_chronological();
        assert_eq!(sorted[0].text, "First");
        assert_eq!(sorted[1].text, "Second");
        assert_eq!(sorted[2].text, "Third");
    }

    #[test]
    fn test_narrative_serialization() {
        let narrative = Narrative::builder()
            .title("Test")
            .event(make_event(40.0, -74.0, "2024-03-15T10:00:00Z", "Event"))
            .build();

        let json = serde_json::to_string(&narrative).unwrap();
        let parsed: Narrative = serde_json::from_str(&json).unwrap();

        assert_eq!(narrative.title, parsed.title);
        assert_eq!(narrative.events.len(), parsed.events.len());
    }
}
