# spatial-narrative Implementation Checklist

## Project Setup

- [x] Initialize Rust project with `cargo new spatial-narrative --lib`
- [x] Configure Cargo.toml with dependencies and features
- [x] Set up CI/CD pipeline (GitHub Actions)
- [x] Configure code formatting (`rustfmt.toml`)
- [x] Configure linting (`clippy.toml`)
- [x] Create module structure skeleton
- [x] Set up benchmark harness with `criterion` (configured in Cargo.toml)
- [x] Set up property-based testing with `proptest` (added to dev-dependencies)
- [x] Create examples folder with demos

---

## Phase 1: Core Foundation (Week 1-2)

**Goal**: Basic types and data structures

### Core Types
- [x] Define `Location` struct (lat, lon, elevation, uncertainty)
- [x] Define `Timestamp` struct with timezone awareness
- [x] Define `TemporalPrecision` enum (Year, Month, Day, Hour, Minute, Second)
- [x] Define `Event` struct with all fields
- [x] Define `EventId` type (UUID wrapper)
- [x] Define `Narrative` struct
- [x] Define `NarrativeId` type
- [x] Define `NarrativeMetadata` struct
- [x] Define `SourceRef` struct
- [x] Define `SourceType` enum (Article, Report, Witness, Sensor)
- [x] Define `GeoBounds` struct for bounding boxes
- [x] Define `TimeRange` struct for temporal ranges

### Traits
- [x] Implement `SpatialEntity` trait
- [x] Implement `TemporalEntity` trait
- [ ] Implement `Narrative` trait (deferred - struct methods sufficient)

### Builder Patterns
- [x] Implement `LocationBuilder`
- [x] Implement `EventBuilder`
- [x] Implement `NarrativeBuilder`
- [x] Implement `SourceRefBuilder`

### Validation
- [x] Coordinate validation (lat: -90 to 90, lon: -180 to 180)
- [x] Timestamp validation
- [x] Event validation (required fields)

### Testing
- [x] Unit tests for `Location`
- [x] Unit tests for `Timestamp`
- [x] Unit tests for `Event`
- [x] Unit tests for `Narrative`
- [x] Unit tests for all builders
- [ ] Property-based tests for coordinate bounds

### Documentation
- [x] Rustdoc for all public types
- [x] Examples in documentation
- [x] Module-level documentation for `core`

**Deliverable**: `spatial_narrative::core` module with full test coverage ✅ (52 unit tests + 9 doc tests passing)

---

## Phase 2: I/O and Parsing (Week 3-4)

**Goal**: Read and write data

### Format Trait
- [x] Define `Format` trait with import/export methods
- [x] Define common error types for I/O operations

### GeoJSON Support
- [x] Implement `GeoJsonFormat` struct
- [x] Implement `GeoJsonOptions` configuration
- [x] GeoJSON import (FeatureCollection → Narrative)
- [x] GeoJSON export (Narrative → FeatureCollection)
- [x] Handle temporal extensions in properties
- [x] Handle source metadata in properties

### CSV Support
- [x] Implement `CsvFormat` struct with configurable columns
- [x] CSV import with column mapping
- [x] CSV export with configurable columns
- [x] Handle missing/optional columns

### GPX Support (Optional Feature)
- [ ] Implement `GpxFormat` struct
- [ ] GPX import (tracks/waypoints → Events)
- [ ] GPX export (Trajectory → track)

### Custom JSON Format
- [x] Define JSON schema for narrative format
- [x] Implement `JsonFormat`
- [x] Import with version checking
- [x] Export with version tagging

### Streaming Support
- [ ] Implement `StreamingReader` for large files
- [ ] Implement iterator interface for streaming
- [ ] Memory-efficient parsing

### Validation & Errors
- [x] Validation during import
- [x] Meaningful error messages with context
- [ ] Error recovery options

### Testing
- [x] Integration tests with sample GeoJSON files
- [x] Integration tests with sample CSV files
- [x] Round-trip tests (import → export → import)
- [x] Error handling tests (malformed input)
- [ ] Large file streaming tests

### Documentation
- [x] Format-specific documentation
- [x] Examples for each format
- [ ] Error handling guide

**Deliverable**: `spatial_narrative::io` module with format converters ✅ (11 new tests + 4 doc tests passing)

---

## Phase 3: Indexing (Week 5-6)

**Goal**: Efficient spatial queries

### Spatial Index (R-tree)
- [x] Integrate `rstar` crate
- [x] Implement `SpatialIndex<T>` struct
- [x] Implement `IndexedLocation` wrapper for R-tree
- [x] `query_bbox()` - bounding box queries
- [x] `query_radius()` - radius queries (degrees approximation)
- [x] `nearest()` - k-nearest neighbors
- [x] `query_bounds()` - GeoBounds queries
- [x] `insert()` - add items to index
- [ ] `remove()` - remove items from index (deferred)
- [x] `from_iter()` - efficient batch insertion

### Temporal Index (B-tree)
- [x] Implement `TemporalIndex<T>` struct
- [x] `query_range()` - time range queries
- [x] `before()` - events before timestamp
- [x] `after()` - events after timestamp
- [x] `first()` / `last()` - earliest/latest events
- [x] `chronological()` - iterator in time order
- [x] `time_range()` - get overall time range

### Spatiotemporal Index
- [x] Implement `SpatiotemporalIndex<T>` struct
- [x] Combined space + time queries
- [x] `heatmap()` - efficient heatmap generation
- [x] Implement `GridSpec` for heatmap configuration
- [x] Implement `Heatmap` result type
- [x] `nearest_in_range()` - k-nearest within time range

### Performance
- [ ] Benchmark spatial queries vs naive iteration
- [ ] Benchmark temporal queries
- [ ] Benchmark combined queries
- [ ] Memory usage profiling

### Testing
- [x] Unit tests for spatial index operations
- [x] Unit tests for temporal index operations
- [x] Unit tests for combined queries
- [x] Unit tests for heatmap generation
- [ ] Property-based tests for query correctness
- [ ] Benchmark tests with criterion

### Documentation
- [x] Module-level documentation with examples
- [x] Doc examples for all index types
- [ ] Performance characteristics guide

**Deliverable**: `spatial_narrative::index` module ✅ (12 tests passing)

---

## Phase 4: Graph Structures (Week 7-8)

**Goal**: Represent narratives as graphs

### Graph Types
- [x] Integrate `petgraph` crate
- [x] Implement `NodeId` struct for node references
- [x] Implement `EdgeType` enum (Temporal, Spatial, Causal, Thematic, Reference, Custom)
- [x] Implement `EdgeWeight` struct with type and weight
- [x] Implement `NarrativeGraph` struct

### Connection Rules
- [x] `connect_temporal()` - temporal sequence connections
- [x] `connect_spatial()` - spatial proximity connections (configurable distance)
- [x] `connect_thematic()` - shared tags connections
- [ ] Custom predicate connections (deferred)

### Graph Construction
- [x] `from_events()` - build graph from events
- [x] `add_event()` - add individual events
- [x] `connect()` / `connect_weighted()` - manual edge creation

### Graph Operations
- [x] `shortest_path()` - find path between events (Dijkstra)
- [x] `has_path()` - check connectivity
- [x] `subgraph_temporal()` - extract by time range
- [x] `subgraph_spatial()` - extract by geographic bounds
- [x] `successors()` / `predecessors()` - get neighbors
- [x] `roots()` / `leaves()` - find entry/exit points
- [x] `in_degree()` / `out_degree()` - node connectivity
- [x] `edges_of_type()` - filter edges by type
- [ ] `communities()` - community detection (deferred)
- [ ] `critical_path()` - critical path analysis (deferred)

### Graph Export
- [x] DOT format export for visualization (`to_dot()`, `to_dot_with_options()`)
- [x] JSON export of graph structure (`to_json()`, `to_json_pretty()`)
- [x] `DotOptions` for customizing DOT output
- [x] Node coloring by connectivity (roots, leaves, hubs)

### Testing
- [x] Unit tests for graph construction
- [x] Unit tests for temporal connections
- [x] Unit tests for thematic connections
- [x] Unit tests for path finding
- [x] Unit tests for roots/leaves detection
- [x] Unit tests for haversine distance
- [ ] Integration tests with sample narratives

### Documentation
- [x] Module-level documentation with examples
- [x] Rustdoc for all public types
- [ ] Graph concepts explanation guide

**Deliverable**: `spatial_narrative::graph` module ✅ (8 tests passing)

---

## Phase 5: Analysis Tools (Week 9-11)

**Goal**: Analytical capabilities

### Spatial Metrics
- [ ] Implement `SpatialMetrics` struct
- [ ] `bounds()` - geographic extent
- [ ] `total_distance()` - sum of event-to-event distances
- [ ] `dispersion()` - variance from centroid
- [ ] `centroid()` - geographic center of mass
- [ ] `density_map()` - events per unit area

### Temporal Metrics
- [ ] Implement `TemporalMetrics` struct
- [ ] `duration()` - total narrative duration
- [ ] `event_rate()` - events over time (binned)
- [ ] `inter_event_times()` - gaps between events
- [ ] `temporal_clusters()` - detect time-based clusters

### Movement Analysis
- [ ] Implement `MovementAnalyzer` struct
- [ ] Implement `Trajectory` struct
- [ ] Implement `Stop` struct
- [ ] Implement `StopThreshold` configuration
- [ ] `extract_trajectories()` - events → trajectories
- [ ] `velocity_profile()` - speed over time
- [ ] `detect_stops()` - stationary periods
- [ ] `simplify()` - Douglas-Peucker simplification

### Clustering
- [ ] Implement `SpatialClustering` struct
- [ ] Implement `Cluster` result type
- [ ] Implement `ClusterTree` for hierarchical results
- [ ] `dbscan()` - density-based clustering
- [ ] `kmeans()` - k-means with geographic distance
- [ ] `hierarchical()` - hierarchical clustering

### Comparison
- [ ] Implement `NarrativeComparison` struct
- [ ] `spatial_similarity()` - spatial overlap metric
- [ ] `temporal_alignment()` - temporal alignment metric
- [ ] `common_locations()` - shared locations within radius

### Parallel Processing
- [ ] Integrate `rayon` for parallel operations
- [ ] Parallel clustering for large datasets
- [ ] Parallel metric computation

### Testing
- [ ] Unit tests for each spatial metric
- [ ] Unit tests for each temporal metric
- [ ] Unit tests for movement analysis
- [ ] Unit tests for clustering algorithms
- [ ] Unit tests for comparison functions
- [ ] Benchmark tests for large datasets
- [ ] Accuracy tests against known results

### Documentation
- [ ] Metric explanations and use cases
- [ ] Algorithm descriptions
- [ ] Performance guidance

**Deliverable**: `spatial_narrative::analysis` module with benchmarks

---

## Phase 6: Text Processing (Week 12-13)

**Goal**: Extract spatial info from text

### Geoparser
- [ ] Implement `GeoParser` struct
- [ ] Implement `LocationPattern` struct
- [ ] Implement `LocationMention` struct
- [ ] Implement `MentionType` enum (PlaceName, Address, Coordinates)
- [ ] `extract()` - extract location mentions from text
- [ ] `geocode()` - resolve mentions to coordinates

### Coordinate Detection
- [ ] Decimal degrees: "40.7128, -74.0060"
- [ ] Degrees with symbols: "40.7128°N, 74.0060°W"
- [ ] DMS format: "40°42'46\"N, 74°0'22\"W"

### Gazetteer
- [ ] Implement `Gazetteer` trait
- [ ] Built-in lightweight gazetteer (major cities/countries)
- [ ] Plugin interface for external services
- [ ] Custom gazetteer support

### Named Entity Recognition (Basic)
- [ ] Implement `TextAnalyzer` struct
- [ ] Implement `Entity` struct
- [ ] Implement `EntityType` enum
- [ ] `entities()` - extract named entities
- [ ] `tokenize()` - text tokenization

### Keyword Extraction
- [ ] Implement `KeywordExtractor` struct
- [ ] Implement `Keyword` struct
- [ ] `extract()` - extract keywords from narrative

### Multilingual Support
- [ ] Language detection
- [ ] Unicode handling
- [ ] Common place name variants

### Testing
- [ ] Unit tests for coordinate parsing
- [ ] Unit tests for place name extraction
- [ ] Accuracy tests against labeled data
- [ ] Multilingual tests

### Documentation
- [ ] Supported formats and patterns
- [ ] Gazetteer integration guide
- [ ] Accuracy expectations

**Deliverable**: `spatial_narrative::text` module with accuracy tests

---

## Phase 7: CLI Tools (Week 14)

**Goal**: Command-line utilities

### CLI Framework
- [ ] Integrate `clap` for argument parsing
- [ ] Define common CLI options
- [ ] Implement output formatting (JSON, table, etc.)

### sn-convert
- [ ] Format detection from file extension
- [ ] GeoJSON ↔ CSV conversion
- [ ] GeoJSON ↔ GPX conversion
- [ ] Batch conversion support
- [ ] Validation during conversion

### sn-analyze
- [ ] Compute spatial metrics
- [ ] Compute temporal metrics
- [ ] Output format options (JSON, human-readable)
- [ ] Filter options (region, time range)

### sn-cluster
- [ ] DBSCAN clustering CLI
- [ ] K-means clustering CLI
- [ ] Output clusters as GeoJSON
- [ ] Parameter tuning options

### sn-graph
- [ ] Build graph from narrative
- [ ] Export to DOT format
- [ ] Connection rule configuration
- [ ] Visualization hints

### sn-query
- [ ] Interactive querying mode
- [ ] Spatial queries (bbox, radius)
- [ ] Temporal queries (range)
- [ ] Tag filtering
- [ ] Output formatting

### Testing
- [ ] CLI integration tests
- [ ] Help text verification
- [ ] Error message tests

### Documentation
- [ ] Comprehensive help text for each command
- [ ] Man page generation
- [ ] Usage examples

**Deliverable**: CLI binary with user guide

---

## Phase 8: Documentation and Examples (Week 15-16)

**Goal**: Comprehensive documentation

### API Documentation (rustdoc)
- [ ] Every public type documented
- [ ] Every public function documented
- [ ] Every trait documented
- [ ] Code examples in docs
- [ ] Links between related items
- [ ] Performance notes where relevant

### User Guide
- [ ] Chapter 1: Getting Started
- [ ] Chapter 2: Loading Data
- [ ] Chapter 3: Working with Narratives
- [ ] Chapter 4: Spatial Analysis
- [ ] Chapter 5: Graph Analysis
- [ ] Chapter 6: Advanced Topics
- [ ] Chapter 7: CLI Tools

### Cookbook
- [ ] Recipe 1: Import Twitter data with geotagged posts
- [ ] Recipe 2: Analyze protest movements across cities
- [ ] Recipe 3: Track hurricane progression from weather data
- [ ] Recipe 4: Build timeline visualization from news articles
- [ ] Recipe 5: Detect anomalous event patterns
- [ ] Recipe 6: Merge narratives from multiple sources
- [ ] Recipe 7: Export to interactive web map
- [ ] Recipe 8: Compute similarity between historical events

### Example Code
- [ ] examples/basic/hello_world.rs
- [ ] examples/basic/load_geojson.rs
- [ ] examples/basic/simple_analysis.rs
- [ ] examples/intermediate/clustering.rs
- [ ] examples/intermediate/graph_analysis.rs
- [ ] examples/intermediate/text_parsing.rs
- [ ] examples/advanced/custom_format.rs
- [ ] examples/advanced/parallel_processing.rs
- [ ] examples/advanced/web_service.rs

### Sample Datasets
- [ ] Hurricane tracking (500 events, real NOAA data)
- [ ] Protest movements (1000 events, synthetic)
- [ ] Migration routes (10K events, synthetic)
- [ ] Wildfire progression (5K events, real data)

### Additional Documentation
- [ ] Performance tuning guide
- [ ] Migration guide template (for future versions)
- [ ] Contributing guidelines
- [ ] Architecture decision records

**Deliverable**: Published documentation and examples

---

## Post-Launch

### Community
- [ ] Publish to crates.io
- [ ] Create GitHub Discussions
- [ ] Set up issue templates
- [ ] Write announcement blog post
- [ ] Recruit beta testers from journalism/research communities

### Maintenance
- [ ] Set up dependabot for dependency updates
- [ ] Create release process documentation
- [ ] Plan version 2.0 features

---

## Summary Timeline

| Phase | Description | Duration |
|-------|-------------|----------|
| Setup | Project initialization | Before Week 1 |
| Phase 1 | Core Foundation | Week 1-2 |
| Phase 2 | I/O and Parsing | Week 3-4 |
| Phase 3 | Indexing | Week 5-6 |
| Phase 4 | Graph Structures | Week 7-8 |
| Phase 5 | Analysis Tools | Week 9-11 |
| Phase 6 | Text Processing | Week 12-13 |
| Phase 7 | CLI Tools | Week 14 |
| Phase 8 | Documentation | Week 15-16 |

**Total**: ~16 weeks to v1.0
