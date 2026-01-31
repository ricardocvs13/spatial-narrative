# spatial-narrative

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-rustdoc-green.svg)](https://docs.rs/spatial-narrative)

A Rust library for modeling, indexing, analyzing, and transforming **spatial narratives** — sequences of events anchored in both space and time.

## Overview

`spatial-narrative` provides a comprehensive toolkit for working with geospatial event data. It enables researchers, journalists, urban planners, and developers to:

- **Model** events with precise locations, timestamps, and rich metadata
- **Index** large datasets for efficient spatial and temporal queries
- **Analyze** patterns, trajectories, clusters, and relationships
- **Transform** between formats (GeoJSON, CSV, JSON) for interoperability
- **Graph** event relationships for network analysis

## Features

| Module | Description | Status |
|--------|-------------|--------|
| `core` | Location, Timestamp, Event, Narrative types | ✅ Complete |
| `io` | Import/export (GeoJSON, CSV, JSON) | ✅ Complete |
| `index` | R-tree spatial, B-tree temporal, combined indexes | ✅ Complete |
| `graph` | Event relationship graphs with petgraph | ✅ Complete |
| `analysis` | Metrics, clustering, trajectory analysis | 🚧 Planned |
| `parser` | Geoparsing from unstructured text | 🚧 Planned |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
spatial-narrative = "1.0"
```

## Quick Start

```rust
use spatial_narrative::core::{Event, Location, Timestamp, Narrative, NarrativeBuilder};
use spatial_narrative::index::SpatiotemporalIndex;
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
    .events(vec![event1.clone(), event2.clone()])
    .build();

// Index for fast queries
let mut index = SpatiotemporalIndex::new();
for event in &narrative.events {
    index.insert(event.clone(), &event.location, &event.timestamp);
}

// Build relationship graph
let mut graph = NarrativeGraph::from_events(narrative.events.clone());
graph.connect_temporal();  // Auto-connect by time sequence
graph.connect_spatial(5.0); // Connect events within 5km
```

## Core Concepts

### Events

An `Event` is the fundamental unit — a thing that happened at a specific place and time:

```rust
use spatial_narrative::core::{Event, EventBuilder, Location, Timestamp, SourceRef, SourceType};

let event = EventBuilder::new()
    .location(Location::builder()
        .lat(48.8566)
        .lon(2.3522)
        .name("Paris, France")
        .build()
        .unwrap())
    .timestamp(Timestamp::parse("2024-07-14T10:00:00Z").unwrap())
    .text("Bastille Day celebrations commence")
    .tag("celebration")
    .tag("national-holiday")
    .source(SourceRef::builder()
        .title("Le Monde")
        .source_type(SourceType::Article)
        .url("https://lemonde.fr/article/123")
        .build())
    .build();
```

### Narratives

A `Narrative` is an ordered collection of related events:

```rust
use spatial_narrative::core::{Narrative, NarrativeBuilder, GeoBounds, TimeRange};

let narrative = NarrativeBuilder::new()
    .title("European Summit 2024")
    .author("Research Team")
    .description("Tracking diplomatic events across Europe")
    .events(events)
    .tag("diplomacy")
    .build();

// Query capabilities
let chronological = narrative.events_chronological();
let time_span = narrative.time_range();
let geographic_extent = narrative.bounds();

// Filtering
let paris_events = narrative.filter_spatial(&GeoBounds::new(48.0, 2.0, 49.0, 3.0));
let january_events = narrative.filter_temporal(&TimeRange::month(2024, 1));
```

## Indexing

Efficient queries over large event collections using specialized data structures.

### Spatial Index (R-tree)

```rust
use spatial_narrative::index::SpatialIndex;
use spatial_narrative::core::Location;

let mut index: SpatialIndex<Event> = SpatialIndex::new();

// Insert events
for event in &events {
    index.insert(event.clone(), &event.location);
}

// Bounding box query
let results = index.query_bbox(40.0, -75.0, 42.0, -73.0);

// K-nearest neighbors
let nearest = index.nearest(40.7128, -74.0060, 5);

// Radius query (approximate, in degrees)
let nearby = index.query_radius(40.7128, -74.0060, 0.1);
```

### Temporal Index (B-tree)

```rust
use spatial_narrative::index::TemporalIndex;
use spatial_narrative::core::{Timestamp, TimeRange};

let mut index: TemporalIndex<Event> = TemporalIndex::new();

for event in &events {
    index.insert(event.clone(), &event.timestamp);
}

// Time range query
let range = TimeRange::new(
    Timestamp::parse("2024-01-01T00:00:00Z").unwrap(),
    Timestamp::parse("2024-01-31T23:59:59Z").unwrap(),
);
let january_events = index.query_range(&range);

// Before/after queries
let early_events = index.before(&cutoff_time);
let recent_events = index.after(&start_time);

// Chronological iteration
for event in index.chronological() {
    println!("{}: {}", event.timestamp.to_rfc3339(), event.text);
}
```

### Spatiotemporal Index

Combined space-time queries with heatmap generation:

```rust
use spatial_narrative::index::{SpatiotemporalIndex, GridSpec};
use spatial_narrative::core::{GeoBounds, TimeRange};

let mut index = SpatiotemporalIndex::new();

for event in &events {
    index.insert(event.clone(), &event.location, &event.timestamp);
}

// Combined query: events in NYC during January
let bounds = GeoBounds::new(40.4, -74.3, 41.0, -73.7);
let range = TimeRange::month(2024, 1);
let results = index.query(&bounds, &range);

// Generate heatmap data for visualization
let grid = GridSpec::new(bounds, 50, 50);  // 50x50 grid
let heatmap = index.heatmap(grid);

// Export heatmap for visualization (see Visualization section)
for lat_idx in 0..heatmap.grid.lat_cells {
    for lon_idx in 0..heatmap.grid.lon_cells {
        let count = heatmap.get(lat_idx, lon_idx);
        let normalized = heatmap.get_normalized(lat_idx, lon_idx);
        // Use with mapping library...
    }
}
```

## Graph Analysis

Model event relationships as directed graphs using [petgraph](https://docs.rs/petgraph).

### Building Graphs

```rust
use spatial_narrative::graph::{NarrativeGraph, EdgeType, EdgeWeight};

// Create from events
let mut graph = NarrativeGraph::from_events(events);

// Automatic connection strategies
graph.connect_temporal();       // A → B if A happens before B
graph.connect_spatial(10.0);    // Connect events within 10km
graph.connect_thematic();       // Connect events sharing tags

// Manual connections
let n1 = graph.get_node(&event1.id).unwrap();
let n2 = graph.get_node(&event2.id).unwrap();
graph.connect(n1, n2, EdgeType::Causal);

// Weighted connections
graph.connect_weighted(n1, n2, EdgeWeight::with_weight(EdgeType::Reference, 0.8));
```

### Graph Queries

```rust
// Path finding
if let Some(path) = graph.shortest_path(start_node, end_node) {
    println!("Path length: {} nodes", path.len());
    println!("Total weight: {}", path.total_weight);
}

// Connectivity
let has_connection = graph.has_path(node_a, node_b);

// Neighborhood
let following_events = graph.successors(node);
let preceding_events = graph.predecessors(node);

// Structure analysis
let entry_points = graph.roots();   // Events with no predecessors
let endpoints = graph.leaves();      // Events with no successors

// Subgraph extraction
let january_subgraph = graph.subgraph_temporal(&TimeRange::month(2024, 1));
let nyc_subgraph = graph.subgraph_spatial(&nyc_bounds);
```

### Edge Types

| Type | Description | Use Case |
|------|-------------|----------|
| `Temporal` | Time sequence | A happens before B |
| `Spatial` | Geographic proximity | Events at same location |
| `Causal` | Cause and effect | A causes B |
| `Thematic` | Shared themes/tags | Related topics |
| `Reference` | Citation/mention | A references B |
| `Custom` | User-defined | Domain-specific |

## I/O Formats

### GeoJSON

Industry-standard format for geographic data. Compatible with Leaflet, Mapbox, QGIS, Google Earth.

```rust
use spatial_narrative::io::{Format, GeoJsonFormat, GeoJsonOptions};

// Export to GeoJSON
let format = GeoJsonFormat::with_options(GeoJsonOptions {
    include_ids: true,
    include_tags: true,
    include_sources: true,
    timestamp_property: "time".to_string(),
    text_property: "description".to_string(),
});

let mut output = Vec::new();
format.export(&narrative, &mut output)?;

// Import from GeoJSON
let narrative: Narrative = format.import(&mut geojson_reader)?;
```

### CSV

For spreadsheet analysis and data science workflows:

```rust
use spatial_narrative::io::{Format, CsvFormat, CsvOptions};

let format = CsvFormat::with_options(CsvOptions {
    lat_column: "latitude".to_string(),
    lon_column: "longitude".to_string(),
    timestamp_column: "datetime".to_string(),
    text_column: Some("description".to_string()),
    delimiter: b',',
    ..Default::default()
});

// Round-trip
format.export(&narrative, &mut csv_writer)?;
let imported: Narrative = format.import(&mut csv_reader)?;
```

### Native JSON

Full-fidelity format preserving all metadata:

```rust
use spatial_narrative::io::{Format, JsonFormat};

let format = JsonFormat::pretty();  // Human-readable
format.export(&narrative, &mut output)?;
```

## Visualization Integration

`spatial-narrative` is a **data processing library**, not a visualization tool. It produces data structures that integrate with mapping libraries:

### Web (JavaScript)

Export to GeoJSON and use with Leaflet or Mapbox:

```javascript
// Load exported GeoJSON
fetch('narrative.geojson')
  .then(res => res.json())
  .then(geojson => {
    L.geoJSON(geojson, {
      pointToLayer: (feature, latlng) => {
        return L.circleMarker(latlng, {
          radius: 8,
          fillColor: getColor(feature.properties.timestamp)
        });
      }
    }).addTo(map);
  });
```

### Heatmaps

Convert `Heatmap` output to visualization format:

```rust
// Generate heatmap data
let heatmap = index.heatmap(grid);

// Export as GeoJSON grid for visualization
let features: Vec<_> = (0..heatmap.grid.lat_cells)
    .flat_map(|lat_idx| {
        (0..heatmap.grid.lon_cells).map(move |lon_idx| {
            let count = heatmap.get(lat_idx, lon_idx);
            let (lat_size, lon_size) = heatmap.grid.cell_size();
            let min_lat = heatmap.grid.bounds.min_lat + lat_idx as f64 * lat_size;
            let min_lon = heatmap.grid.bounds.min_lon + lon_idx as f64 * lon_size;
            
            // Create GeoJSON polygon feature for each cell
            serde_json::json!({
                "type": "Feature",
                "properties": { "count": count, "intensity": heatmap.get_normalized(lat_idx, lon_idx) },
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [[
                        [min_lon, min_lat],
                        [min_lon + lon_size, min_lat],
                        [min_lon + lon_size, min_lat + lat_size],
                        [min_lon, min_lat + lat_size],
                        [min_lon, min_lat]
                    ]]
                }
            })
        })
    })
    .collect();
```

### Desktop GIS

Export to GeoJSON and open in:
- **QGIS** — Full-featured open-source GIS
- **ArcGIS** — Professional GIS platform
- **Google Earth Pro** — 3D globe visualization

### Graph Visualization

Export graph structure for network visualization tools:

```rust
// Export to DOT format for Graphviz
fn export_dot(graph: &NarrativeGraph) -> String {
    let mut dot = String::from("digraph narrative {\n");
    
    for (node_id, event) in graph.nodes() {
        dot.push_str(&format!(
            "  {} [label=\"{}\"];\n",
            node_id.index(),
            event.text.chars().take(30).collect::<String>()
        ));
    }
    
    for (from, to, weight) in graph.edges() {
        dot.push_str(&format!(
            "  {} -> {} [label=\"{:?}\"];\n",
            from.index(),
            to.index(),
            weight.edge_type
        ));
    }
    
    dot.push_str("}\n");
    dot
}
```

## Examples

Run included examples:

```bash
# Core types and operations
cargo run --example basic_usage

# I/O format handling
cargo run --example io_formats

# Spatial and temporal indexing
cargo run --example indexing
```

## Performance

| Operation | Complexity | Notes |
|-----------|------------|-------|
| Spatial bbox query | O(log n + k) | R-tree, k = results |
| Temporal range query | O(log n + k) | B-tree |
| K-nearest neighbors | O(log n + k) | R-tree |
| Graph path finding | O((V + E) log V) | Dijkstra |
| Heatmap generation | O(n) | Single pass |

For datasets exceeding 1M events, consider:
- Streaming imports with `io::StreamingReader` (planned)
- Spatial partitioning by region
- Temporal partitioning by time period
- Parallel processing with `rayon` feature

## API Reference

Full API documentation available at:

```bash
cargo doc --open
```

Or view online at [docs.rs/spatial-narrative](https://docs.rs/spatial-narrative).

## Use Cases

| Domain | Application |
|--------|-------------|
| **Journalism** | Track story development across locations and time |
| **Historical Research** | Model timelines with precise geographic context |
| **Urban Planning** | Analyze event patterns in urban environments |
| **Disaster Response** | Correlate incident reports spatiotemporally |
| **Travel & Tourism** | Build location-aware travel narratives |
| **Academic Research** | Process geographic and temporal research data |
| **Security Analysis** | Pattern detection in event sequences |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Run tests
cargo test

# Run with all features
cargo test --all-features

# Check formatting
cargo fmt --check

# Run linter
cargo clippy
```

## License

MIT License — see [LICENSE](LICENSE) for details.

## Acknowledgments

Built with:
- [rstar](https://docs.rs/rstar) — R-tree implementation
- [petgraph](https://docs.rs/petgraph) — Graph data structures
- [chrono](https://docs.rs/chrono) — Date and time handling
- [serde](https://docs.rs/serde) — Serialization framework
- [geo](https://docs.rs/geo) — Geospatial primitives
