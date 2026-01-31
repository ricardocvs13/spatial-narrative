# Bounds

The `core` module provides types for representing geographic and temporal bounds.

## GeoBounds

Represents a rectangular geographic region.

### Creating GeoBounds

```rust
use spatial_narrative::core::GeoBounds;

// Explicit bounds
let bounds = GeoBounds::new(
    40.4,   // min_lat (south)
    -74.3,  // min_lon (west)
    41.0,   // max_lat (north)
    -73.7   // max_lon (east)
);

// From locations
let bounds = GeoBounds::from_locations(&[
    Location::new(40.7128, -74.0060),  // NYC
    Location::new(42.3601, -71.0589),  // Boston
]);
```

### Methods

```rust
// Check if a location is within bounds
if bounds.contains(40.75, -73.99) {
    println!("Location is within bounds");
}

// Get center point
let (lat, lon) = bounds.center();

// Check intersection
if bounds1.intersects(&bounds2) {
    println!("Regions overlap");
}
```

## TimeRange

Represents a period between two timestamps.

### Creating TimeRange

```rust
use spatial_narrative::core::{TimeRange, Timestamp};

// From timestamps
let range = TimeRange::new(
    Timestamp::parse("2024-01-01T00:00:00Z")?,
    Timestamp::parse("2024-12-31T23:59:59Z")?
);

// Helper methods
let year_2024 = TimeRange::year(2024);
let june_2024 = TimeRange::month(2024, 6);
```

### Methods

```rust
// Get duration
let days = range.duration().num_days();

// Check if timestamp is within range
if range.contains(&timestamp) {
    println!("Time is within range");
}

// Check overlap
if range1.overlaps(&range2) {
    println!("Ranges overlap");
}
```

## Examples

### Regional Analysis

```rust
// Define regions
let east_coast = GeoBounds::new(25.0, -82.0, 45.0, -66.0);
let west_coast = GeoBounds::new(32.0, -125.0, 49.0, -114.0);

// Filter events by region
let east_events = narrative.filter_spatial(&east_coast);
let west_events = narrative.filter_spatial(&west_coast);
```

### Temporal Analysis

```rust
// Define periods
let q1_2024 = TimeRange::new(
    Timestamp::parse("2024-01-01")?,
    Timestamp::parse("2024-03-31")?
);

// Filter events by period
let q1_events = narrative.filter_temporal(&q1_2024);
```
