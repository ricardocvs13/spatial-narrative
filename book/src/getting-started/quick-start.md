# Quick Start

This guide will get you up and running with `spatial-narrative` in under 5 minutes.

## Create Your First Narrative

### Step 1: Create Events

Events are the fundamental building blocks. Each event has a location, timestamp, and text description:

```rust
use spatial_narrative::core::{Event, Location, Timestamp};

// Create locations
let nyc = Location::new(40.7128, -74.0060);
let boston = Location::new(42.3601, -71.0589);

// Create timestamps
let morning = Timestamp::parse("2024-06-15T09:00:00Z").unwrap();
let afternoon = Timestamp::parse("2024-06-15T14:00:00Z").unwrap();

// Create events
let event1 = Event::new(nyc, morning, "Departure from New York City");
let event2 = Event::new(boston, afternoon, "Arrival in Boston");

println!("Event 1: {} at {}", event1.text, event1.timestamp.to_rfc3339());
println!("Event 2: {} at {}", event2.text, event2.timestamp.to_rfc3339());
```

### Step 2: Build a Narrative

A Narrative collects events into a coherent story:

```rust
use spatial_narrative::core::{Event, Location, Timestamp, NarrativeBuilder};

let events = vec![
    Event::new(
        Location::new(40.7128, -74.0060),
        Timestamp::parse("2024-06-15T09:00:00Z").unwrap(),
        "Departure from NYC"
    ),
    Event::new(
        Location::new(42.3601, -71.0589),
        Timestamp::parse("2024-06-15T14:00:00Z").unwrap(),
        "Arrival in Boston"
    ),
];

let narrative = NarrativeBuilder::new()
    .title("Road Trip to Boston")
    .description("A day trip from NYC to Boston")
    .tag("travel")
    .tag("road-trip")
    .events(events)
    .build();

println!("Narrative: {}", narrative.title.as_deref().unwrap_or("Untitled"));
println!("Events: {}", narrative.events.len());
```

### Step 3: Query Events

Filter events spatially and temporally:

```rust
use spatial_narrative::core::{GeoBounds, TimeRange, Timestamp};

// Get events in chronological order
let ordered = narrative.events_chronological();
for event in &ordered {
    println!("{}: {}", event.timestamp.to_rfc3339(), event.text);
}

// Get time range
if let Some(range) = narrative.time_range() {
    println!("Duration: {} hours", range.duration().num_hours());
}

// Get geographic bounds
if let Some(bounds) = narrative.bounds() {
    println!("Area: ({:.2}, {:.2}) to ({:.2}, {:.2})",
        bounds.min_lat, bounds.min_lon,
        bounds.max_lat, bounds.max_lon
    );
}
```

### Step 4: Index for Fast Queries

For large datasets, use indexes:

```rust
use spatial_narrative::index::SpatiotemporalIndex;
use spatial_narrative::core::{GeoBounds, TimeRange, Timestamp};

// Create an index
let mut index = SpatiotemporalIndex::new();

// Insert events
for event in &narrative.events {
    index.insert(event.clone(), &event.location, &event.timestamp);
}

// Query by location and time
let bounds = GeoBounds::new(40.0, -75.0, 43.0, -70.0);
let time_range = TimeRange::new(
    Timestamp::parse("2024-06-15T00:00:00Z").unwrap(),
    Timestamp::parse("2024-06-15T23:59:59Z").unwrap(),
);

let results = index.query(&bounds, &time_range);
println!("Found {} events in region during time range", results.len());
```

### Step 5: Build a Graph

Connect events into a relationship graph:

```rust
use spatial_narrative::graph::{NarrativeGraph, EdgeType};

// Create graph from events
let mut graph = NarrativeGraph::from_events(narrative.events.clone());

// Auto-connect by temporal sequence
graph.connect_temporal();

// Connect spatially close events (within 50km)
graph.connect_spatial(50.0);

println!("Graph: {} nodes, {} edges", graph.node_count(), graph.edge_count());

// Export to DOT format for visualization
let dot = graph.to_dot();
println!("DOT output:\n{}", dot);
```

### Step 6: Export to GeoJSON

Export for use in mapping tools:

```rust
use spatial_narrative::io::{Format, GeoJsonFormat};

let geojson_format = GeoJsonFormat::default();
let geojson_string = geojson_format.export(&narrative.events).unwrap();

// Save to file
std::fs::write("narrative.geojson", &geojson_string).unwrap();

println!("Exported to narrative.geojson");
```

## Complete Example

Here's everything together:

```rust
use spatial_narrative::core::{Event, Location, Timestamp, NarrativeBuilder};
use spatial_narrative::index::SpatiotemporalIndex;
use spatial_narrative::graph::{NarrativeGraph, EdgeType};
use spatial_narrative::io::{Format, GeoJsonFormat};

fn main() {
    // Create events
    let events = vec![
        Event::new(
            Location::builder().lat(40.7128).lon(-74.0060).name("NYC").build().unwrap(),
            Timestamp::parse("2024-06-15T09:00:00Z").unwrap(),
            "Departure from New York City"
        ),
        Event::new(
            Location::builder().lat(41.2033).lon(-73.1975).name("New Haven").build().unwrap(),
            Timestamp::parse("2024-06-15T11:00:00Z").unwrap(),
            "Quick stop in New Haven"
        ),
        Event::new(
            Location::builder().lat(42.3601).lon(-71.0589).name("Boston").build().unwrap(),
            Timestamp::parse("2024-06-15T14:00:00Z").unwrap(),
            "Arrival in Boston"
        ),
    ];

    // Build narrative
    let narrative = NarrativeBuilder::new()
        .title("Road Trip to Boston")
        .events(events)
        .build();

    // Index events
    let mut index = SpatiotemporalIndex::new();
    for event in &narrative.events {
        index.insert(event.clone(), &event.location, &event.timestamp);
    }

    // Build graph
    let mut graph = NarrativeGraph::from_events(narrative.events.clone());
    graph.connect_temporal();

    // Export
    let geojson = GeoJsonFormat::default().export(&narrative.events).unwrap();

    println!("Created narrative with {} events", narrative.events.len());
    println!("Graph has {} connections", graph.edge_count());
    println!("GeoJSON: {} bytes", geojson.len());
}
```

## Next Steps

- [Concepts](./concepts.md) - Understand the architecture
- [Core Types](../core/overview.md) - Deep dive into types
- [Cookbook](../cookbook/patterns.md) - Common patterns and recipes
