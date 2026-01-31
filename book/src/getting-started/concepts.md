# Core Concepts

Understanding these concepts will help you use `spatial-narrative` effectively.

## The Data Model

```
┌─────────────────────────────────────────────────────────────────┐
│                         NARRATIVE                                │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ Title, Description, Tags, Metadata                           ││
│  └─────────────────────────────────────────────────────────────┘│
│                              │                                   │
│                              ▼                                   │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐               │
│  │ Event 1 │ │ Event 2 │ │ Event 3 │ │ Event N │ ...           │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘               │
│       │           │           │           │                      │
│       ▼           ▼           ▼           ▼                      │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  Location    │  Timestamp   │  Text      │  Metadata        ││
│  │  (lat, lon)  │  (datetime)  │  (string)  │  (tags, source)  ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

## Core Types

### Event

The **Event** is the atomic unit. It represents something that happened:
- **Where**: A `Location` (latitude, longitude, optional elevation)
- **When**: A `Timestamp` (timezone-aware datetime with precision)
- **What**: Text description
- **Context**: Tags, source references, custom metadata

```rust
// Simple event
let event = Event::new(location, timestamp, "Something happened");

// Rich event with builder
let event = EventBuilder::new()
    .location(location)
    .timestamp(timestamp)
    .text("Something happened")
    .tag("important")
    .source(source_ref)
    .build()
    .unwrap();
```

### Location

A **Location** represents a point on Earth:
- Required: `lat` (-90 to 90), `lon` (-180 to 180)
- Optional: `elevation` (meters), `uncertainty` (meters), `name`

```rust
// Minimal
let loc = Location::new(40.7128, -74.0060);

// With details
let loc = Location::builder()
    .lat(40.7128)
    .lon(-74.0060)
    .elevation(10.5)
    .uncertainty_meters(5.0)
    .name("Empire State Building")
    .build()
    .unwrap();
```

### Timestamp

A **Timestamp** represents a moment in time:
- Timezone-aware (stored as UTC)
- Precision levels: Year, Month, Day, Hour, Minute, Second
- Flexible parsing (ISO 8601, dates, partial dates)

```rust
// Parse various formats
Timestamp::parse("2024-01-15T10:30:00Z")?;     // Full ISO 8601
Timestamp::parse("2024-01-15")?;                // Date only
Timestamp::parse("2024-01")?;                   // Month only
Timestamp::parse("2024")?;                      // Year only

// Current time
Timestamp::now();
```

### Narrative

A **Narrative** is a collection of events that form a coherent story:
- Has a title, description, and tags
- Contains zero or more events
- Provides aggregate operations (bounds, time range, filtering)

```rust
let narrative = NarrativeBuilder::new()
    .title("My Story")
    .description("A series of events")
    .tag("category")
    .events(vec![event1, event2, event3])
    .build();
```

## Spatial Concepts

### Coordinates

We use **WGS84** (standard GPS) coordinates:
- Latitude: -90 (South Pole) to +90 (North Pole)
- Longitude: -180 to +180 (Prime Meridian at 0)

```
                    +90° (North Pole)
                         │
    -180° ───────────────┼───────────────► +180°
         (International  │  (Date Line)
          Date Line)     │
                         │
                    -90° (South Pole)
```

### GeoBounds

A **GeoBounds** represents a rectangular region:
- Defined by min/max latitude and longitude
- Used for spatial queries and filtering

```rust
// NYC metropolitan area
let bounds = GeoBounds::new(
    40.4,   // min_lat (south)
    -74.3,  // min_lon (west)
    41.0,   // max_lat (north)
    -73.7   // max_lon (east)
);

// From a collection of locations
let bounds = GeoBounds::from_locations(&locations);
```

## Temporal Concepts

### TimeRange

A **TimeRange** represents a period between two timestamps:
- Has a start and end timestamp
- Supports duration calculations and overlap checks

```rust
// Explicit range
let range = TimeRange::new(start_timestamp, end_timestamp);

// From year/month
let year_2024 = TimeRange::year(2024);
let june_2024 = TimeRange::month(2024, 6);
```

### Temporal Precision

Events can have different temporal precision:

| Precision | Example | Use Case |
|-----------|---------|----------|
| `Year` | "2024" | Historical events |
| `Month` | "2024-01" | Approximate dates |
| `Day` | "2024-01-15" | Calendar events |
| `Hour` | "2024-01-15T10:00" | Scheduled events |
| `Minute` | "2024-01-15T10:30" | Meetings |
| `Second` | "2024-01-15T10:30:45" | Precise logging |

## Graph Concepts

### NarrativeGraph

Events can be connected into a directed graph:
- **Nodes** = Events
- **Edges** = Relationships between events

### Edge Types

| Type | Meaning | Auto-Connect |
|------|---------|--------------|
| `Temporal` | A happens before B | `connect_temporal()` |
| `Spatial` | A is near B | `connect_spatial(km)` |
| `Thematic` | A and B share tags | `connect_thematic()` |
| `Causal` | A causes B | Manual |
| `Reference` | A references B | Manual |

```rust
let mut graph = NarrativeGraph::from_events(events);

// Automatic connections
graph.connect_temporal();     // Time sequence
graph.connect_spatial(10.0);  // Within 10km
graph.connect_thematic();     // Shared tags

// Manual connections
graph.connect(node1, node2, EdgeType::Causal);
```

## Indexing Concepts

### Spatial Index (R-tree)

For efficient geographic queries:
- Bounding box queries: "Find all events in this region"
- Radius queries: "Find all events within X km of this point"
- Nearest neighbor: "Find the closest N events to this location"

### Temporal Index (B-tree)

For efficient time-based queries:
- Range queries: "Find all events between date A and date B"
- Before/After: "Find all events before/after this time"
- Ordering: "Iterate events in chronological order"

### Spatiotemporal Index

Combines both for powerful queries:
- "Find events in NYC during June 2024"
- "Find nearest events to this location within the last hour"

## Next Steps

- [Location](../core/location.md) - Deep dive into Location type
- [Timestamp](../core/timestamp.md) - Deep dive into Timestamp type
- [Event](../core/event.md) - Deep dive into Event type
