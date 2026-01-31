# Spatial Narrative

<div class="warning">

📚 **Welcome to the Spatial Narrative documentation!**

This book provides comprehensive documentation for the `spatial-narrative` Rust library.

</div>

## What is Spatial Narrative?

**Spatial Narrative** is a Rust library for modeling, indexing, analyzing, and transforming **spatial narratives** — sequences of events anchored in both space and time.

Think of it as a toolkit for working with **events that happen somewhere and somewhen**.

## Key Features

| Feature | Description |
|---------|-------------|
| 🗺️ **Spatial Modeling** | Precise location handling with coordinates, elevation, and uncertainty |
| ⏱️ **Temporal Precision** | Timezone-aware timestamps with configurable precision |
| 📊 **Efficient Indexing** | R-tree spatial and B-tree temporal indexes for fast queries |
| 🔗 **Graph Analysis** | Build relationship graphs between events |
| 📁 **Format Support** | Import/export GeoJSON, CSV, and JSON |
| ⚡ **Performance** | Designed for large-scale event processing |

## Use Cases

- **Journalism**: Track story development across locations and time
- **Historical Research**: Model timelines with precise geographic context
- **Urban Planning**: Analyze event patterns in urban environments
- **Disaster Response**: Correlate incident reports spatiotemporally
- **Academic Research**: Process geographic and temporal research data

## Quick Example

```rust
use spatial_narrative::core::{Event, Location, Timestamp, NarrativeBuilder};
use spatial_narrative::graph::{NarrativeGraph, EdgeType};

// Create events
let event1 = Event::new(
    Location::new(40.7128, -74.0060),  // NYC
    Timestamp::parse("2024-01-15T10:00:00Z").unwrap(),
    "Conference begins in Manhattan"
);

let event2 = Event::new(
    Location::new(40.7580, -73.9855),  // Times Square
    Timestamp::parse("2024-01-15T14:00:00Z").unwrap(),
    "Press conference at Times Square"
);

// Build a narrative
let narrative = NarrativeBuilder::new()
    .title("NYC Conference Coverage")
    .events(vec![event1, event2])
    .build();

// Build a graph and connect events
let mut graph = NarrativeGraph::from_events(narrative.events);
graph.connect_temporal();  // Auto-connect by time sequence

println!("Narrative has {} events, {} connections", 
    graph.node_count(), 
    graph.edge_count()
);
```

## Getting Started

Ready to dive in? Start with the [Installation](./getting-started/installation.md) guide!

## Navigation

- **Getting Started**: Installation, quick start, and core concepts
- **Core Types**: Detailed documentation of Location, Event, Timestamp, etc.
- **I/O Formats**: Import/export to GeoJSON, CSV, JSON
- **Indexing**: Efficient spatial and temporal queries
- **Graph Analysis**: Build and analyze event relationship graphs
- **Cookbook**: Common patterns and recipes

## Links

- [📦 Crates.io](https://crates.io/crates/spatial-narrative)
- [📖 API Documentation](https://docs.rs/spatial-narrative)
- [🐙 GitHub Repository](https://github.com/yourusername/spatial-narrative)
- [💬 Discussions](https://github.com/yourusername/spatial-narrative/discussions)
