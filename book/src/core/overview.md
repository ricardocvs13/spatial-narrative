# Core Types Overview

The `core` module provides the fundamental types for spatial narratives.

## Type Hierarchy

```
spatial_narrative::core
├── Location         # Geographic point
├── Timestamp        # Point in time
├── Event            # Something that happened somewhere, somewhen
├── Narrative        # Collection of events
├── GeoBounds        # Geographic bounding box
├── TimeRange        # Temporal range
├── SourceRef        # Source attribution
└── EventId          # Unique event identifier
```

## Quick Reference

### Location

```rust
use spatial_narrative::core::Location;

// Simple
let loc = Location::new(40.7128, -74.0060);

// With builder
let loc = Location::builder()
    .lat(40.7128)
    .lon(-74.0060)
    .name("New York City")
    .build()
    .unwrap();
```

### Timestamp

```rust
use spatial_narrative::core::Timestamp;

// Parse from string
let ts = Timestamp::parse("2024-01-15T10:30:00Z").unwrap();

// Current time
let now = Timestamp::now();

// From Unix timestamp
let ts = Timestamp::from_unix(1705315800).unwrap();
```

### Event

```rust
use spatial_narrative::core::{Event, Location, Timestamp};

// Simple
let event = Event::new(
    Location::new(40.7128, -74.0060),
    Timestamp::now(),
    "Event description"
);

// With builder
let event = EventBuilder::new()
    .location(location)
    .timestamp(timestamp)
    .text("Description")
    .tag("category")
    .build()
    .unwrap();
```

### Narrative

```rust
use spatial_narrative::core::NarrativeBuilder;

let narrative = NarrativeBuilder::new()
    .title("My Narrative")
    .description("A collection of events")
    .events(vec![event1, event2])
    .build();
```

## Common Operations

### Serialization

All core types implement `Serialize` and `Deserialize`:

```rust
use serde_json;

// Serialize
let json = serde_json::to_string(&event)?;

// Deserialize
let event: Event = serde_json::from_str(&json)?;
```

### Comparison

Events, Timestamps, and Locations implement comparison traits:

```rust
// Timestamps are ordered
if timestamp1 < timestamp2 {
    println!("Event 1 happened first");
}

// Events can be compared by time
let events = events.into_iter()
    .sorted_by_key(|e| e.timestamp)
    .collect();
```

### Validation

Types validate on construction:

```rust
// This will fail - invalid latitude
let result = Location::new(200.0, 0.0);  // Returns error

// Use builders for detailed error handling
match Location::builder().lat(200.0).lon(0.0).build() {
    Ok(loc) => println!("Valid"),
    Err(e) => println!("Error: {}", e),
}
```

## Module Links

- [Location](./location.md) - Geographic points
- [Timestamp](./timestamp.md) - Points in time
- [Event](./event.md) - Events with location and time
- [Narrative](./narrative.md) - Event collections
- [Bounds](./bounds.md) - Geographic and temporal bounds
- [Sources](./sources.md) - Source attribution
