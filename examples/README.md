# Spatial Narrative Examples

This directory contains runnable examples demonstrating the `spatial-narrative` library.

## Running Examples

Use `cargo run --example <name>` to run any example:

```bash
# Core types and basic usage
cargo run --example basic_usage

# I/O format handling (JSON, GeoJSON, CSV)
cargo run --example io_formats

# Spatial and temporal indexing
cargo run --example indexing

# Graph building and export (DOT/JSON)
cargo run --example graph_export

# Spatial analysis (distances, bearings, density)
cargo run --example spatial_analysis

# Temporal analysis (event rates, gaps, bursts)
cargo run --example temporal_analysis

# Movement analysis (trajectories, stops, paths)
cargo run --example movement_analysis

# Clustering (DBSCAN, KMeans)
cargo run --example clustering

# Narrative comparison (similarity metrics)
cargo run --example comparison
```

## Examples Overview

### `basic_usage.rs`
Demonstrates the core types and operations:
- Creating `Location`, `Timestamp`, and `Event` objects
- Using builder patterns for configuration
- Building a `Narrative` from events
- Spatial and temporal filtering
- Querying chronological order

### `io_formats.rs`
Shows how to import/export narratives in different formats:
- **JSON**: Native format with full fidelity
- **GeoJSON**: Compatible with mapping tools (Leaflet, Mapbox, QGIS)
- **CSV**: For spreadsheets and data analysis

### `indexing.rs`
Demonstrates efficient spatial and temporal queries:
- **SpatialIndex**: R-tree for geographic queries (bounding box, nearest neighbor)
- **TemporalIndex**: B-tree for time-based queries (ranges, before/after)
- **SpatiotemporalIndex**: Combined space-time queries
- **Heatmap**: Density visualization

### `graph_export.rs`
Demonstrates building and exporting narrative graphs:
- Creating a `NarrativeGraph` from events
- Auto-connecting by temporal, spatial, and thematic relationships
- Exporting to **DOT format** (Graphviz) for visualization
- Exporting to **JSON** for web visualization (D3.js, Cytoscape.js)
- Graph analysis: roots, leaves, edge types

### `spatial_analysis.rs`
Demonstrates spatial metrics and calculations:
- **Haversine distance**: Calculate great-circle distances between points
- **Bearing calculation**: Compass direction between locations
- **Destination point**: Find location at distance and bearing from start
- **SpatialMetrics**: Total distance, bounds, centroid of narratives
- **Density maps**: Grid-based event density visualization

### `temporal_analysis.rs`
Demonstrates temporal metrics and patterns:
- **TemporalMetrics**: Duration, spans, gaps in narratives
- **Event rate**: Calculate events per time unit
- **Time binning**: Group events by hour, day, week, month
- **Gap detection**: Find periods of inactivity
- **Burst detection**: Identify clusters of rapid activity

### `movement_analysis.rs`
Demonstrates trajectory and movement pattern analysis:
- **Trajectory**: Model paths with total distance, duration, speed
- **Velocity profiles**: Speed over time analysis
- **Stop detection**: Find stationary periods using configurable thresholds
- **Movement segments**: Split trajectories at stops
- **Path simplification**: Douglas-Peucker algorithm for reducing points

### `clustering.rs`
Demonstrates spatial clustering algorithms:
- **DBSCAN**: Density-based clustering with eps/min_points parameters
- **KMeans**: Partition-based clustering with k clusters
- **Cluster analysis**: Centroids, membership, radius calculation
- **Parameter sensitivity**: Compare different clustering configurations

### `comparison.rs`
Demonstrates narrative comparison and similarity:
- **Spatial similarity**: How much geographic overlap exists
- **Temporal similarity**: How much time overlap exists
- **Thematic similarity**: How similar are the event tags/topics
- **Common locations**: Find shared places between narratives
- **Weighted comparison**: Custom weights for different dimensions
- **Set operations**: Spatial intersection and union

---

## Future Examples (Planned)

### `graph_analysis.rs` (Planned)
Will demonstrate:
- Finding paths and routes through narratives
- Network analysis (centrality, clustering)
- Community detection in event networks

### `text_analysis.rs` (Planned)
Will demonstrate:
- Extracting entities from narrative text
- Topic modeling across events
- Sentiment analysis over time and space
- Named entity recognition for locations and dates

### `parallel_processing.rs` (Planned)
Will demonstrate:
- Parallel iteration over large event collections
- Batch processing with rayon
- Efficient aggregations and transformations

### `visualization.rs` (Planned)
Will demonstrate:
- Generating map visualizations
- Creating timeline charts
- Exporting to web-friendly formats

### `streaming.rs` (Planned)
Will demonstrate:
- Processing large files in streaming fashion
- Memory-efficient imports
- Real-time event ingestion

---

## Use Cases

The library is designed for applications such as:

1. **Historical Research**: Model timelines of historical events with precise locations
2. **Journalism**: Track story development across locations and time
3. **Travel & Tourism**: Build location-aware travel narratives
4. **Urban Planning**: Analyze spatial patterns in urban events
5. **Academic Research**: Process geographic and temporal data for analysis

## Performance Notes

- **Spatial Index**: O(log n) for bounding box queries, efficient for large datasets
- **Temporal Index**: O(log n) for range queries using B-tree
- **Combined Index**: Intersection-based approach for space-time queries
- **Heatmaps**: Linear time relative to event count

For very large datasets (millions of events), consider:
- Using streaming imports for memory efficiency
- Partitioning data by time or region
- Using parallel processing with rayon
