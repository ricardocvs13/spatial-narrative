# Event

The `Event` type represents something that happened at a specific place and time.

## Creating Events

### Simple Constructor

```rust
use spatial_narrative::core::{Event, Location, Timestamp};

let event = Event::new(
    Location::new(40.7128, -74.0060),
    Timestamp::parse("2024-01-15T10:00:00Z").unwrap(),
    "Conference begins"
);
```

### Builder Pattern

```rust
use spatial_narrative::core::{EventBuilder, Location, Timestamp, SourceRef};

let event = EventBuilder::new()
    .location(Location::new(40.7128, -74.0060))
    .timestamp(Timestamp::parse("2024-01-15T10:00:00Z").unwrap())
    .text("Conference begins in Manhattan")
    .tag("conference")
    .tag("technology")
    .source(SourceRef::builder()
        .title("Event Calendar")
        .url("https://example.com/events")
        .build())
    .build()
    .unwrap();
```

## Properties

| Property | Type | Description |
|----------|------|-------------|
| `id` | `EventId` | Unique identifier (UUID) |
| `location` | `Location` | Where it happened |
| `timestamp` | `Timestamp` | When it happened |
| `text` | `String` | Description |
| `tags` | `HashSet<String>` | Categories/labels |
| `source` | `Option<SourceRef>` | Source attribution |
| `metadata` | `HashMap<String, Value>` | Custom metadata |

## Methods

### Tags

```rust
// Add tags
event.add_tag("important");
event.add_tag("verified");

// Check tags
if event.has_tag("important") {
    println!("This is important!");
}

// Get all tags
for tag in &event.tags {
    println!("Tag: {}", tag);
}
```

### Metadata

```rust
use serde_json::json;

// Add metadata
event.set_metadata("priority", json!(1));
event.set_metadata("verified", json!(true));

// Get metadata
if let Some(priority) = event.get_metadata("priority") {
    println!("Priority: {}", priority);
}
```

## Traits

Events implement spatial and temporal traits:

```rust
use spatial_narrative::core::traits::{SpatialEntity, TemporalEntity};

// Spatial trait
let coords = event.coordinates();  // (lat, lon)
let bounds = event.bounds();       // GeoBounds

// Temporal trait  
let time = event.time();           // Timestamp
let range = event.time_range();    // TimeRange
```

## Examples

### News Event

```rust
let event = EventBuilder::new()
    .location(Location::builder()
        .lat(48.8566).lon(2.3522)
        .name("Paris, France")
        .build()?)
    .timestamp(Timestamp::parse("2024-07-14T10:00:00Z")?)
    .text("Bastille Day celebrations commence")
    .tag("celebration")
    .tag("national-holiday")
    .source(SourceRef::builder()
        .title("Le Monde")
        .source_type(SourceType::Article)
        .url("https://lemonde.fr/article/123")
        .build())
    .build()?;
```

### Sensor Reading

```rust
let reading = EventBuilder::new()
    .location(Location::builder()
        .lat(37.7749).lon(-122.4194)
        .uncertainty_meters(1.0)
        .build()?)
    .timestamp(Timestamp::now())
    .text("Temperature reading")
    .tag("sensor")
    .tag("temperature")
    .build()?;

reading.set_metadata("temperature_c", json!(22.5));
reading.set_metadata("humidity_pct", json!(65));
```
