# Spatial Narrative

<div align="center">

<svg viewBox="0 0 600 160" xmlns="http://www.w3.org/2000/svg" style="max-width: 500px; width: 100%;">
    <defs>
        <linearGradient id="primaryGrad" x1="0%" y1="0%" x2="100%" y2="0%">
            <stop offset="0%" style="stop-color:#1e3a8a;stop-opacity:1" />
            <stop offset="100%" style="stop-color:#f97316;stop-opacity:1" />
        </linearGradient>
    </defs>
    <text x="40" y="115" font-family="'Segoe UI', sans-serif" font-weight="800" font-size="64" fill="#1e3a8a" letter-spacing="-1.5">spatial</text>
    <text x="270" y="85" font-family="'Segoe UI', sans-serif" font-weight="300" font-size="64" fill="#f97316" letter-spacing="-1">narrative</text>
    <path id="pathStepUp" d="M45,135 L230,135 C250,135 250,105 270,105 L540,105" fill="none" stroke="url(#primaryGrad)" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"/>
    <circle cx="45" cy="135" r="5" fill="#1e3a8a"/> 
    <circle cx="540" cy="105" r="5" fill="#f97316"/>
    <circle r="5" fill="white" stroke="#f97316" stroke-width="2">
        <animateMotion dur="4s" repeatCount="indefinite" keyPoints="0;1" keyTimes="0;1" calcMode="spline" keySplines="0.4 0 0.2 1">
            <mpath href="#pathStepUp"/>
        </animateMotion>
        <animate attributeName="opacity" values="0;1;1;0" keyTimes="0;0.1;0.9;1" dur="4s" repeatCount="indefinite" />
    </circle>
</svg>

</div>

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
| � **Analysis Tools** | Metrics, clustering, trajectory analysis, and comparison |
| �📁 **Format Support** | Import/export GeoJSON, CSV, and JSON |
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
- **Analysis**: Spatial metrics, temporal analysis, clustering, movement detection
- **Cookbook**: Common patterns and recipes

## Links

- [📦 Crates.io](https://crates.io/crates/spatial-narrative)
- [📖 API Documentation](https://docs.rs/spatial-narrative)
- [🐙 GitHub Repository](https://github.com/jwilliamsresearch/spatial-narrative)
- [💬 Discussions](https://github.com/jwilliamsresearch/spatial-narrative/discussions)
