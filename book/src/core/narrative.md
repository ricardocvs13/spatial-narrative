# Narrative

The `Narrative` type represents a collection of related events that form a coherent story.

## Creating Narratives

### Builder Pattern

```rust
use spatial_narrative::core::{NarrativeBuilder, Event};

let narrative = NarrativeBuilder::new()
    .title("Road Trip to Boston")
    .description("A day trip from NYC to Boston with stops along the way")
    .tag("travel")
    .tag("road-trip")
    .events(vec![event1, event2, event3])
    .build();
```

### Empty Narrative

```rust
let mut narrative = NarrativeBuilder::new()
    .title("My Story")
    .build();

// Add events later
narrative.add_event(event1);
narrative.add_event(event2);
```

## Properties

| Property | Type | Description |
|----------|------|-------------|
| `id` | `NarrativeId` | Unique identifier |
| `title` | `Option<String>` | Narrative title |
| `description` | `Option<String>` | Description |
| `events` | `Vec<Event>` | Collection of events |
| `tags` | `HashSet<String>` | Categories/labels |
| `metadata` | `NarrativeMetadata` | Additional metadata |

## Methods

### Events

```rust
// Add events
narrative.add_event(event);

// Get event count
println!("Events: {}", narrative.events.len());

// Iterate events
for event in &narrative.events {
    println!("{}", event.text);
}
```

### Chronological Order

```rust
// Get events sorted by time
let ordered = narrative.events_chronological();

for event in ordered {
    println!("{}: {}", event.timestamp.to_rfc3339(), event.text);
}
```

### Time Range

```rust
// Get overall time span
if let Some(range) = narrative.time_range() {
    println!("Start: {}", range.start.to_rfc3339());
    println!("End: {}", range.end.to_rfc3339());
    println!("Duration: {} days", range.duration().num_days());
}
```

### Geographic Bounds

```rust
// Get bounding box of all events
if let Some(bounds) = narrative.bounds() {
    println!("Lat: {} to {}", bounds.min_lat, bounds.max_lat);
    println!("Lon: {} to {}", bounds.min_lon, bounds.max_lon);
    
    let (center_lat, center_lon) = bounds.center();
    println!("Center: ({}, {})", center_lat, center_lon);
}
```

### Filtering

```rust
// Filter by geographic bounds
let paris_events = narrative.filter_spatial(&paris_bounds);

// Filter by time range
let june_events = narrative.filter_temporal(&june_2024);
```

## Examples

### Historical Timeline

```rust
let ww1 = NarrativeBuilder::new()
    .title("World War I Timeline")
    .description("Key events of the Great War")
    .tag("history")
    .tag("world-war")
    .events(vec![
        Event::new(
            Location::new(43.8563, 18.4131),  // Sarajevo
            Timestamp::parse("1914-06-28")?,
            "Assassination of Archduke Franz Ferdinand"
        ),
        Event::new(
            Location::new(48.8566, 2.3522),   // Paris
            Timestamp::parse("1919-06-28")?,
            "Treaty of Versailles signed"
        ),
    ])
    .build();

println!("Timeline: {}", ww1.title.as_deref().unwrap_or("Untitled"));
println!("Duration: {} years", ww1.time_range().unwrap().duration().num_days() / 365);
```

### Travel Journal

```rust
let trip = NarrativeBuilder::new()
    .title("European Adventure 2024")
    .tag("travel")
    .tag("europe")
    .events(cities_visited)
    .build();

// Get geographic extent
if let Some(bounds) = trip.bounds() {
    println!("Trip covered {} degrees of latitude", 
        bounds.max_lat - bounds.min_lat);
}
```
