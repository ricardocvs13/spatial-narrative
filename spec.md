# spatial-narrative: Rust Library for Real-World Spatial Narratives

**Version:** 1.0.0  
**Status:** Planning Document  
**License:** MIT/Apache-2.0 (dual license)

## Executive Summary

`spatial-narrative` is a Rust library for representing, analyzing, and working with narratives that unfold across real-world geographic space. Target use cases include journalism, historical research, event documentation, investigative reporting, travel writing, and spatial humanities research.

The library provides zero-copy parsing, efficient spatial indexing, and rich analytical capabilities for understanding how real-world events, movements, and stories develop across geography..

---

## Table of Contents

1. [Core Concepts](#core-concepts)
2. [Architecture Overview](#architecture-overview)
3. [Module Specifications](#module-specifications)
4. [API Design](#api-design)
5. [Data Formats](#data-formats)
6. [Implementation Plan](#implementation-plan)
7. [Dependencies](#dependencies)
8. [Performance Considerations](#performance-considerations)
9. [Testing Strategy](#testing-strategy)
10. [Documentation Plan](#documentation-plan)

---

## Core Concepts

### What is a Spatial Narrative?

A spatial narrative is any real-world story or sequence of events with geographic coordinates and temporal ordering. Examples:

- **Journalism**: How a wildfire spreads across counties over several days
- **Historical research**: Migration patterns of refugees during conflicts
- **Investigative reporting**: Tracking movements of political figures or suspects
- **Conflict documentation**: Mapping incidents of violence chronologically
- **Environmental studies**: Documenting pollution events along a river system
- **Travel narratives**: Recording a journey with timestamped locations
- **Disaster response**: Timeline of earthquake aftershocks and aid distribution

### Key Entities

1. **Event**: Something that happened at a specific place and time with associated text/metadata
2. **Narrative**: A collection of related events forming a coherent story
3. **Location**: Geographic coordinates (WGS84 lat/lon) with optional place names
4. **Trajectory**: Temporal sequence of locations (e.g., a person's movements)
5. **Region**: Named geographic boundary relevant to the narrative
6. **Source**: Origin of event data (news article, witness account, sensor data)

### Design Philosophy

- **Performance**: Leverage Rust's zero-copy parsing and efficient memory management
- **Correctness**: Strong typing prevents invalid geographic/temporal relationships
- **Extensibility**: Plugin architecture for custom analyzers and parsers
- **Interoperability**: Standard formats (GeoJSON, CSV, GPX) with custom extensions
- **Real-world focus**: Handle messy data, uncertain locations, timezone complexity

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                        Application Layer                     │
│  (CLI tools, web services, analysis scripts)                │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                         Core Library                         │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │  Types   │  │  Parser  │  │ Analysis │  │   I/O    │   │
│  │  Core    │  │  Extract │  │ Metrics  │  │  Format  │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │  Spatial │  │  Temporal│  │  Graph   │  │Transform │   │
│  │  Index   │  │  Index   │  │ Topology │  │  Coord   │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                    Dependency Layer                          │
│  geo, rstar, chrono, serde, petgraph, rayon                 │
└─────────────────────────────────────────────────────────────┘
```

---

## Module Specifications

### 1. `spatial_narrative::core`

**Purpose**: Fundamental types and traits

**Key Types**:

```rust
/// Geographic point in WGS84
pub struct Location {
    lat: f64,
    lon: f64,
    elevation: Option<f64>,
    uncertainty_meters: Option<f64>,
}

/// Timestamp with timezone awareness
pub struct Timestamp {
    datetime: DateTime<Utc>,
    precision: TemporalPrecision, // Year, Month, Day, Hour, Minute, Second
}

/// A single event in a narrative
pub struct Event {
    id: EventId,
    location: Location,
    timestamp: Timestamp,
    text: String,
    metadata: HashMap<String, String>,
    sources: Vec<SourceRef>,
    tags: Vec<String>,
}

/// A collection of related events
pub struct Narrative {
    id: NarrativeId,
    title: String,
    events: Vec<Event>,
    metadata: NarrativeMetadata,
}

/// Reference to source material
pub struct SourceRef {
    source_type: SourceType, // Article, Report, Witness, Sensor
    url: Option<String>,
    title: Option<String>,
    date: Option<Timestamp>,
}
```

**Key Traits**:

```rust
pub trait SpatialEntity {
    fn location(&self) -> &Location;
    fn bounds(&self) -> GeoBounds;
}

pub trait TemporalEntity {
    fn timestamp(&self) -> &Timestamp;
    fn time_range(&self) -> (Timestamp, Timestamp);
}

pub trait Narrative {
    fn events(&self) -> &[Event];
    fn add_event(&mut self, event: Event);
    fn filter_spatial(&self, bounds: GeoBounds) -> Vec<&Event>;
    fn filter_temporal(&self, range: TimeRange) -> Vec<&Event>;
}
```

**Design Decisions**:
- Use `f64` for coordinates (industry standard)
- Optional uncertainty for imprecise locations
- Metadata as HashMap for flexibility
- Immutable by default, builder pattern for construction

---

### 2. `spatial_narrative::parser`

**Purpose**: Extract spatial information from unstructured text

**Key Components**:

```rust
/// Parse location references from text
pub struct GeoParser {
    gazetteer: Gazetteer,
    patterns: Vec<LocationPattern>,
}

impl GeoParser {
    /// Extract location mentions from text
    pub fn extract(&self, text: &str) -> Vec<LocationMention>;
    
    /// Resolve location names to coordinates
    pub fn geocode(&self, mention: &LocationMention) -> Option<Location>;
}

/// Pattern matching for spatial expressions
pub struct LocationPattern {
    pattern: Regex,
    extractor: Box<dyn Fn(&Captures) -> PartialLocation>,
}

/// Mentioned location (before geocoding)
pub struct LocationMention {
    text: String,
    span: (usize, usize), // character offsets
    context: String,
    mention_type: MentionType, // PlaceName, Address, Coordinates
}
```

**Parsing Capabilities**:
- Place names: "Paris", "New York City", "Mount Everest"
- Addresses: "123 Main St, Boston, MA"
- Coordinate formats: "40.7128°N, 74.0060°W", "40.7128, -74.0060"
- Relative locations: "10km north of Denver"
- Natural language: "near the border", "along the coast"

**Gazetteer Integration**:
- Built-in lightweight gazetteer (major cities/countries)
- Plugin support for external services (Nominatim, GeoNames)
- Custom gazetteers for domain-specific locations

**Design Decisions**:
- Return mentions with context for disambiguation
- Separate extraction from geocoding for pipeline flexibility
- Support multiple coordinate formats
- Handle ambiguous place names (multiple candidates)

---

### 3. `spatial_narrative::index`

**Purpose**: Efficient spatial and temporal indexing

**Spatial Index** (R-tree):

```rust
pub struct SpatialIndex<T: SpatialEntity> {
    rtree: RTree<IndexedItem<T>>,
}

impl<T: SpatialEntity> SpatialIndex<T> {
    /// Find items within bounding box
    pub fn query_bbox(&self, bounds: GeoBounds) -> Vec<&T>;
    
    /// Find items within radius of point
    pub fn query_radius(&self, center: Location, radius_meters: f64) -> Vec<&T>;
    
    /// Find k nearest neighbors
    pub fn nearest(&self, location: Location, k: usize) -> Vec<&T>;
    
    /// Range query with custom predicate
    pub fn query_predicate<F>(&self, bounds: GeoBounds, pred: F) -> Vec<&T>
    where F: Fn(&T) -> bool;
}
```

**Temporal Index** (B-tree):

```rust
pub struct TemporalIndex<T: TemporalEntity> {
    btree: BTreeMap<Timestamp, Vec<IndexEntry<T>>>,
}

impl<T: TemporalEntity> TemporalIndex<T> {
    /// Events in time range
    pub fn query_range(&self, range: TimeRange) -> Vec<&T>;
    
    /// Events before/after timestamp
    pub fn before(&self, time: Timestamp) -> Vec<&T>;
    pub fn after(&self, time: Timestamp) -> Vec<&T>;
    
    /// Sliding window query
    pub fn sliding_window(&self, duration: Duration) -> SlidingWindowIter<T>;
}
```

**Spatiotemporal Index** (Combined):

```rust
pub struct SpatiotemporalIndex<T: SpatialEntity + TemporalEntity> {
    spatial: SpatialIndex<T>,
    temporal: TemporalIndex<T>,
}

impl<T: SpatialEntity + TemporalEntity> SpatiotemporalIndex<T> {
    /// Events in space AND time
    pub fn query(&self, bounds: GeoBounds, range: TimeRange) -> Vec<&T>;
    
    /// Efficient heatmap generation
    pub fn heatmap(&self, grid: GridSpec, range: TimeRange) -> Heatmap;
}
```

**Design Decisions**:
- Use `rstar` crate for R-tree (mature, well-tested)
- Dual indexing for spatiotemporal queries
- Lazy iteration for large result sets
- Support custom predicates for filtering

---

### 4. `spatial_narrative::graph`

**Purpose**: Represent narratives as spatial-temporal graphs

**Graph Structure**:

```rust
/// Node represents an event
pub struct EventNode {
    event: Event,
    metadata: NodeMetadata,
}

/// Edge represents relationship between events
pub struct EventEdge {
    edge_type: EdgeType,
    weight: f64,
    metadata: EdgeMetadata,
}

pub enum EdgeType {
    Temporal(Duration),        // Sequential in time
    Spatial(f64),             // Distance in meters
    Causal,                   // Cause-effect relationship
    Thematic,                 // Related by topic/tag
    Source,                   // From same source
}

pub struct NarrativeGraph {
    graph: DiGraph<EventNode, EventEdge>,
    spatial_index: SpatialIndex<NodeId>,
    temporal_index: TemporalIndex<NodeId>,
}
```

**Graph Operations**:

```rust
impl NarrativeGraph {
    /// Build from events (auto-connect based on rules)
    pub fn from_narrative(narrative: &Narrative, rules: &[ConnectionRule]) -> Self;
    
    /// Find path between events
    pub fn path(&self, start: NodeId, end: NodeId) -> Option<Vec<NodeId>>;
    
    /// Extract subgraph by region/time
    pub fn subgraph(&self, bounds: GeoBounds, range: TimeRange) -> NarrativeGraph;
    
    /// Detect communities/clusters
    pub fn communities(&self) -> Vec<Vec<NodeId>>;
    
    /// Critical path analysis
    pub fn critical_path(&self) -> Vec<NodeId>;
}
```

**Connection Rules**:

```rust
pub enum ConnectionRule {
    /// Connect events within time window
    TemporalProximity { max_gap: Duration },
    
    /// Connect events within distance
    SpatialProximity { max_distance: f64 },
    
    /// Connect events with shared tags
    SharedTags { min_overlap: usize },
    
    /// Custom predicate
    Custom { predicate: Box<dyn Fn(&Event, &Event) -> bool> },
}
```

**Design Decisions**:
- Use `petgraph` for graph data structure
- Directed graph (temporal causality)
- Multiple edge types for different relationships
- Pluggable connection rules
- Dual indexing for efficient spatial/temporal queries on graph

---

### 5. `spatial_narrative::analysis`

**Purpose**: Analytical tools and metrics

**Spatial Metrics**:

```rust
pub struct SpatialMetrics;

impl SpatialMetrics {
    /// Geographic extent of narrative
    pub fn bounds(events: &[Event]) -> GeoBounds;
    
    /// Total distance traveled (sum of event-to-event distances)
    pub fn total_distance(events: &[Event]) -> f64;
    
    /// Spatial dispersion (variance from centroid)
    pub fn dispersion(events: &[Event]) -> f64;
    
    /// Geographic center of mass
    pub fn centroid(events: &[Event]) -> Location;
    
    /// Density map (events per unit area)
    pub fn density_map(events: &[Event], grid: GridSpec) -> DensityMap;
}
```

**Temporal Metrics**:

```rust
pub struct TemporalMetrics;

impl TemporalMetrics {
    /// Duration of narrative
    pub fn duration(events: &[Event]) -> Duration;
    
    /// Event rate over time
    pub fn event_rate(events: &[Event], bin_size: Duration) -> Vec<(Timestamp, f64)>;
    
    /// Time gaps between events
    pub fn inter_event_times(events: &[Event]) -> Vec<Duration>;
    
    /// Detect temporal clusters
    pub fn temporal_clusters(events: &[Event], max_gap: Duration) -> Vec<Vec<Event>>;
}
```

**Movement Analysis**:

```rust
pub struct MovementAnalyzer;

impl MovementAnalyzer {
    /// Extract trajectories from events
    pub fn extract_trajectories(events: &[Event]) -> Vec<Trajectory>;
    
    /// Compute velocity over time
    pub fn velocity_profile(trajectory: &Trajectory) -> Vec<(Timestamp, f64)>;
    
    /// Detect stops (stationary periods)
    pub fn detect_stops(trajectory: &Trajectory, threshold: StopThreshold) -> Vec<Stop>;
    
    /// Simplify trajectory (Douglas-Peucker)
    pub fn simplify(trajectory: &Trajectory, epsilon: f64) -> Trajectory;
}
```

**Clustering**:

```rust
pub struct SpatialClustering;

impl SpatialClustering {
    /// DBSCAN clustering
    pub fn dbscan(events: &[Event], eps: f64, min_pts: usize) -> Vec<Cluster>;
    
    /// K-means with geographic distance
    pub fn kmeans(events: &[Event], k: usize) -> Vec<Cluster>;
    
    /// Hierarchical clustering
    pub fn hierarchical(events: &[Event]) -> ClusterTree;
}
```

**Comparison**:

```rust
pub struct NarrativeComparison;

impl NarrativeComparison {
    /// Compare spatial overlap
    pub fn spatial_similarity(n1: &Narrative, n2: &Narrative) -> f64;
    
    /// Compare temporal alignment
    pub fn temporal_alignment(n1: &Narrative, n2: &Narrative) -> f64;
    
    /// Find common locations
    pub fn common_locations(n1: &Narrative, n2: &Narrative, radius: f64) -> Vec<Location>;
}
```

**Design Decisions**:
- Use established algorithms (DBSCAN, k-means)
- Return structured results (not just numbers)
- Support both batch and streaming analysis
- Parallel processing with `rayon` for large datasets

---

### 6. `spatial_narrative::io`

**Purpose**: Import/export data in standard formats

**Format Support**:

```rust
pub trait Format {
    fn import(&self, reader: impl Read) -> Result<Narrative>;
    fn export(&self, narrative: &Narrative, writer: impl Write) -> Result<()>;
}

/// GeoJSON with temporal extensions
pub struct GeoJsonFormat {
    options: GeoJsonOptions,
}

/// CSV with configurable columns
pub struct CsvFormat {
    lat_col: String,
    lon_col: String,
    time_col: String,
    text_col: String,
}

/// GPX (GPS Exchange Format)
pub struct GpxFormat;

/// KML (Keyhole Markup Language)
pub struct KmlFormat;

/// Custom JSON format optimized for narratives
pub struct NarrativeJsonFormat;
```

**GeoJSON Extensions**:

```json
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [-122.4194, 37.7749]
      },
      "properties": {
        "timestamp": "2024-03-15T14:30:00Z",
        "text": "Protest began at City Hall",
        "tags": ["protest", "politics"],
        "source": {
          "type": "article",
          "url": "https://example.com/article"
        }
      }
    }
  ]
}
```

**Streaming Support**:

```rust
/// Stream large files without loading into memory
pub struct StreamingReader<R: Read, F: Format> {
    reader: BufReader<R>,
    format: F,
}

impl<R: Read, F: Format> Iterator for StreamingReader<R, F> {
    type Item = Result<Event>;
    fn next(&mut self) -> Option<Self::Item>;
}
```

**Design Decisions**:
- Support standard geo formats (interoperability)
- Custom format for rich narrative metadata
- Streaming for large datasets
- Validation during import
- Configurable export options (precision, included fields)

---

### 7. `spatial_narrative::transform`

**Purpose**: Coordinate transformations and projections

**Coordinate Systems**:

```rust
pub trait CoordinateSystem {
    fn to_wgs84(&self, point: Point) -> Location;
    fn from_wgs84(&self, location: Location) -> Point;
}

/// WGS84 (standard lat/lon)
pub struct Wgs84;

/// Web Mercator (web maps)
pub struct WebMercator;

/// UTM zones
pub struct Utm {
    zone: u8,
    northern: bool,
}

/// Custom projected coordinate system
pub struct CustomProjection {
    proj_string: String,
}
```

**Transformations**:

```rust
pub struct Transform;

impl Transform {
    /// Convert between coordinate systems
    pub fn convert<From: CoordinateSystem, To: CoordinateSystem>(
        point: Point,
        from: &From,
        to: &To
    ) -> Point;
    
    /// Compute distance (geodesic on WGS84)
    pub fn distance(loc1: Location, loc2: Location) -> f64;
    
    /// Compute bearing
    pub fn bearing(from: Location, to: Location) -> f64;
    
    /// Point at distance and bearing
    pub fn destination(start: Location, distance: f64, bearing: f64) -> Location;
}
```

**Design Decisions**:
- Default to WGS84 for all storage
- Support common projections for analysis
- Use `geo` crate for geodesic calculations
- Optional `proj` integration for advanced projections

---

### 8. `spatial_narrative::text`

**Purpose**: Natural language processing for spatial narratives

**Text Analysis**:

```rust
pub struct TextAnalyzer;

impl TextAnalyzer {
    /// Extract named entities
    pub fn entities(&self, text: &str) -> Vec<Entity>;
    
    /// Tokenization
    pub fn tokenize(&self, text: &str) -> Vec<Token>;
    
    /// Detect language
    pub fn detect_language(&self, text: &str) -> Language;
    
    /// Summarize event text
    pub fn summarize(&self, events: &[Event]) -> String;
}

/// Named entity
pub struct Entity {
    text: String,
    entity_type: EntityType,
    span: (usize, usize),
}

pub enum EntityType {
    Location,
    Person,
    Organization,
    Date,
    Time,
    Event,
}
```

**Keyword Extraction**:

```rust
pub struct KeywordExtractor;

impl KeywordExtractor {
    /// Extract keywords from narrative
    pub fn extract(&self, narrative: &Narrative) -> Vec<Keyword>;
    
    /// Topic modeling
    pub fn topics(&self, narratives: &[Narrative], num_topics: usize) -> Vec<Topic>;
}
```

**Design Decisions**:
- Lightweight NLP (no heavy ML dependencies by default)
- Plugin architecture for external NLP services
- Focus on spatial/temporal entities
- Support multiple languages

---

## API Design

### High-Level API (User-Friendly)

```rust
use spatial_narrative::prelude::*;

// Load narrative from GeoJSON
let narrative = Narrative::from_geojson("events.geojson")?;

// Filter by region and time
let bay_area = GeoBounds::new(37.0, -123.0, 38.0, -122.0);
let march = TimeRange::month(2024, 3);
let filtered = narrative.filter()
    .spatial(bay_area)
    .temporal(march)
    .build();

// Analyze
let metrics = filtered.analyze();
println!("Total distance: {:.2} km", metrics.spatial.total_distance / 1000.0);
println!("Duration: {} days", metrics.temporal.duration.num_days());

// Cluster events
let clusters = filtered.cluster_spatial(eps: 1000.0, min_pts: 3);

// Export results
clusters.to_geojson("clusters.geojson")?;
```

### Low-Level API (Advanced Control)

```rust
use spatial_narrative::{core::*, index::*, analysis::*};

// Build narrative manually
let mut narrative = Narrative::builder()
    .title("Wildfire Progression")
    .metadata("category", "disaster")
    .build();

narrative.add_event(Event::builder()
    .location(Location::new(40.0, -122.0))
    .timestamp(Timestamp::parse("2024-08-01T14:00:00Z")?)
    .text("Fire reported near campground")
    .tag("fire")
    .source(SourceRef::article("https://news.example/fire"))
    .build());

// Build spatial index
let mut index = SpatialIndex::new();
for event in narrative.events() {
    index.insert(event);
}

// Query index
let nearby = index.query_radius(
    Location::new(40.1, -121.9),
    5000.0 // 5km
);

// Build graph with custom rules
let graph = NarrativeGraph::from_narrative(
    &narrative,
    &[
        ConnectionRule::TemporalProximity { max_gap: Duration::hours(2) },
        ConnectionRule::SpatialProximity { max_distance: 10000.0 },
    ]
);

// Analyze graph
let communities = graph.communities();
```

### Builder Pattern

```rust
// Event builder
let event = Event::builder()
    .location(Location::new(40.7128, -74.0060))
    .timestamp(Timestamp::now())
    .text("Event description")
    .tag("protest")
    .metadata("participants", "5000")
    .source(SourceRef::article("https://example.com"))
    .build();

// Narrative builder
let narrative = Narrative::builder()
    .title("January 6 Timeline")
    .add_event(event1)
    .add_event(event2)
    .metadata("topic", "politics")
    .build();
```

### Fluent Query API

```rust
let results = narrative
    .query()
    .within_bounds(bbox)
    .between_times(start, end)
    .with_tag("protest")
    .sorted_by_time()
    .limit(100)
    .collect();
```

---

## Data Formats

### Custom JSON Format

```json
{
  "version": "1.0",
  "narrative": {
    "id": "narrative-001",
    "title": "Hurricane Relief Effort",
    "created": "2024-09-15T00:00:00Z",
    "metadata": {
      "category": "disaster-response",
      "region": "Southeast USA"
    },
    "events": [
      {
        "id": "event-001",
        "timestamp": {
          "datetime": "2024-09-01T08:00:00Z",
          "precision": "minute"
        },
        "location": {
          "lat": 29.7604,
          "lon": -95.3698,
          "elevation": 12.0,
          "uncertainty_meters": 100.0,
          "name": "Houston, TX"
        },
        "text": "Hurricane makes landfall",
        "tags": ["hurricane", "landfall"],
        "sources": [
          {
            "type": "sensor",
            "url": "https://noaa.gov/data/123"
          }
        ],
        "metadata": {
          "wind_speed_mph": "130",
          "category": "4"
        }
      }
    ]
  }
}
```

### SQLite Schema (Optional Backend)

```sql
CREATE TABLE narratives (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    created TIMESTAMP,
    metadata TEXT -- JSON blob
);

CREATE TABLE events (
    id TEXT PRIMARY KEY,
    narrative_id TEXT REFERENCES narratives(id),
    timestamp TIMESTAMP NOT NULL,
    timestamp_precision TEXT,
    lat REAL NOT NULL,
    lon REAL NOT NULL,
    elevation REAL,
    uncertainty_meters REAL,
    location_name TEXT,
    text TEXT,
    metadata TEXT -- JSON blob
);

CREATE TABLE tags (
    event_id TEXT REFERENCES events(id),
    tag TEXT,
    PRIMARY KEY (event_id, tag)
);

CREATE TABLE sources (
    event_id TEXT REFERENCES events(id),
    source_type TEXT,
    url TEXT,
    title TEXT,
    date TIMESTAMP
);

CREATE INDEX idx_events_time ON events(timestamp);
CREATE INDEX idx_events_location ON events(lat, lon);
CREATE INDEX idx_tags_tag ON tags(tag);
```

---

## Implementation Plan

### Phase 1: Core Foundation (Week 1-2)

**Goal**: Basic types and data structures

- [ ] Define core types: `Location`, `Timestamp`, `Event`, `Narrative`
- [ ] Implement traits: `SpatialEntity`, `TemporalEntity`
- [ ] Builder patterns for ergonomic construction
- [ ] Unit tests for all core types
- [ ] Documentation with examples

**Deliverable**: `spatial_narrative::core` module with full test coverage

### Phase 2: I/O and Parsing (Week 3-4)

**Goal**: Read and write data

- [ ] GeoJSON import/export
- [ ] CSV import with configurable columns
- [ ] GPX support for trajectory data
- [ ] Validation during import
- [ ] Error handling and reporting
- [ ] Integration tests with real datasets

**Deliverable**: `spatial_narrative::io` module with format converters

### Phase 3: Indexing (Week 5-6)

**Goal**: Efficient spatial queries

- [ ] Integrate `rstar` for R-tree indexing
- [ ] Implement `SpatialIndex` with query methods
- [ ] Implement `TemporalIndex` with BTreeMap
- [ ] Combined spatiotemporal queries
- [ ] Benchmarks against naive iteration
- [ ] Documentation on when to use indexes

**Deliverable**: `spatial_narrative::index` module with benchmarks

### Phase 4: Graph Structures (Week 7-8)

**Goal**: Represent narratives as graphs

- [ ] Integrate `petgraph` for graph storage
- [ ] Define edge types and connection rules
- [ ] Implement graph construction from narratives
- [ ] Path finding and subgraph extraction
- [ ] Community detection algorithms
- [ ] Graph visualization export (DOT format)

**Deliverable**: `spatial_narrative::graph` module with examples

### Phase 5: Analysis Tools (Week 9-11)

**Goal**: Analytical capabilities

- [ ] Spatial metrics (distance, dispersion, centroid)
- [ ] Temporal metrics (duration, event rate)
- [ ] Movement analysis (trajectories, velocity)
- [ ] Clustering (DBSCAN, k-means)
- [ ] Heatmap generation
- [ ] Comparative analysis between narratives
- [ ] Parallel processing with `rayon`

**Deliverable**: `spatial_narrative::analysis` module with benchmarks

### Phase 6: Text Processing (Week 12-13)

**Goal**: Extract spatial info from text

- [ ] Geoparser for location mentions
- [ ] Gazetteer integration (built-in + plugins)
- [ ] Coordinate format detection
- [ ] Basic NER for entities
- [ ] Keyword extraction
- [ ] Multilingual support

**Deliverable**: `spatial_narrative::text` module with accuracy tests

### Phase 7: CLI Tools (Week 14)

**Goal**: Command-line utilities

- [ ] `sn-convert`: Format conversion
- [ ] `sn-analyze`: Compute metrics
- [ ] `sn-cluster`: Spatial/temporal clustering
- [ ] `sn-graph`: Generate graph visualizations
- [ ] `sn-query`: Interactive querying
- [ ] Comprehensive help text

**Deliverable**: CLI binary with user guide

### Phase 8: Documentation and Examples (Week 15-16)

**Goal**: Comprehensive documentation

- [ ] API documentation (rustdoc)
- [ ] User guide with tutorials
- [ ] Example datasets
- [ ] Cookbook of common tasks
- [ ] Performance tuning guide
- [ ] Migration guide (for future versions)

**Deliverable**: Published documentation and examples

---

## Dependencies

### Core Dependencies

```toml
[dependencies]
# Geometric types and algorithms
geo = "0.28"
geo-types = "0.7"

# Spatial indexing
rstar = "0.12"

# Date/time handling
chrono = "0.4"
chrono-tz = "0.9"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Graph algorithms
petgraph = "0.6"

# Parallel processing
rayon = "1.10"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Hashing (for IDs)
uuid = { version = "1.10", features = ["v4", "serde"] }

# CSV parsing
csv = "1.3"

# Regular expressions
regex = "1.10"
```

### Optional Dependencies

```toml
[dependencies]
# Advanced projections
proj = { version = "0.27", optional = true }

# Natural language processing
unicode-segmentation = { version = "1.11", optional = true }

# GPX parsing
gpx = { version = "0.10", optional = true }

# SQLite backend
rusqlite = { version = "0.31", optional = true, features = ["bundled"] }

# HTTP client for geocoding services
reqwest = { version = "0.12", optional = true, features = ["json"] }

# Command-line interface
clap = { version = "4.5", optional = true, features = ["derive"] }

[features]
default = ["geojson", "csv"]
cli = ["clap"]
projections = ["proj"]
nlp = ["unicode-segmentation"]
gpx-support = ["gpx"]
database = ["rusqlite"]
geocoding = ["reqwest"]
full = ["cli", "projections", "nlp", "gpx-support", "database", "geocoding"]
```

---

## Performance Considerations

### Memory Usage

**Optimization Strategies**:

1. **Zero-copy where possible**: Use string slices and references
2. **Lazy loading**: Stream large files instead of loading into memory
3. **Pooling**: Reuse allocations for repeated operations
4. **Compact representations**: Use `SmallVec` for tags, `CompactString` for IDs

**Memory Benchmarks**:
- Target: < 1KB per event (excluding text)
- Narrative with 100K events: ~100MB
- Spatial index overhead: ~20% of data size

### Query Performance

**Optimization Strategies**:

1. **Spatial indexing**: R-tree for O(log n) spatial queries vs O(n) linear scan
2. **Temporal indexing**: BTree for O(log n) temporal queries
3. **Caching**: Memoize expensive computations (distances, clusters)
4. **Parallel processing**: Use `rayon` for independent operations

**Performance Targets**:
- Spatial query (10K events): < 1ms with index
- Clustering (100K events): < 5s with parallel DBSCAN
- Graph construction (10K events): < 100ms

### Scalability

**Design for Scale**:

1. **Streaming APIs**: Process events one at a time
2. **Chunking**: Break large narratives into spatial/temporal chunks
3. **Incremental updates**: Add events to existing indexes
4. **Disk-backed storage**: SQLite backend for huge datasets

**Scalability Targets**:
- In-memory: Up to 1M events
- Disk-backed: Up to 10M+ events
- Streaming: Unlimited (bounded by disk space)

---

## Testing Strategy

### Unit Tests

**Coverage Areas**:
- Core type construction and validation
- Coordinate transformations
- Distance calculations
- Time parsing and precision
- Builder pattern edge cases

**Example**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_distance() {
        let loc1 = Location::new(40.7128, -74.0060); // NYC
        let loc2 = Location::new(34.0522, -118.2437); // LA
        let dist = Transform::distance(loc1, loc2);
        assert!((dist - 3936000.0).abs() < 10000.0); // ~3936km ±10km
    }

    #[test]
    fn test_event_builder() {
        let event = Event::builder()
            .location(Location::new(0.0, 0.0))
            .timestamp(Timestamp::now())
            .text("Test")
            .build();
        assert_eq!(event.text, "Test");
    }
}
```

### Integration Tests

**Test Scenarios**:
- Load narrative from GeoJSON, analyze, export
- Parse text, geocode, build narrative
- Large dataset performance (1M events)
- Concurrent access to indexes
- Format round-trip (import then export)

**Example**:

```rust
#[test]
fn test_geojson_roundtrip() {
    let original = Narrative::from_geojson("tests/data/sample.geojson").unwrap();
    original.to_geojson("tests/output/roundtrip.geojson").unwrap();
    let roundtrip = Narrative::from_geojson("tests/output/roundtrip.geojson").unwrap();
    assert_eq!(original.events().len(), roundtrip.events().len());
}
```

### Property-Based Tests

**Using `proptest`**:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn location_always_in_bounds(lat in -90.0..90.0, lon in -180.0..180.0) {
        let loc = Location::new(lat, lon);
        assert!(loc.is_valid());
    }

    #[test]
    fn distance_commutative(
        lat1 in -90.0..90.0, lon1 in -180.0..180.0,
        lat2 in -90.0..90.0, lon2 in -180.0..180.0
    ) {
        let loc1 = Location::new(lat1, lon1);
        let loc2 = Location::new(lat2, lon2);
        let d1 = Transform::distance(loc1, loc2);
        let d2 = Transform::distance(loc2, loc1);
        assert!((d1 - d2).abs() < 0.01);
    }
}
```

### Benchmark Tests

**Using `criterion`**:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_spatial_query(c: &mut Criterion) {
    let narrative = generate_test_narrative(10000);
    let index = SpatialIndex::from_events(narrative.events());
    let bbox = GeoBounds::new(40.0, -75.0, 41.0, -74.0);

    c.bench_function("spatial_query_10k", |b| {
        b.iter(|| index.query_bbox(black_box(bbox)))
    });
}

criterion_group!(benches, bench_spatial_query);
criterion_main!(benches);
```

### Test Data

**Sample Datasets**:
1. **Hurricane tracking**: 500 events over 2 weeks (real NOAA data)
2. **Protest movements**: 1000 events across 20 cities (synthetic)
3. **Migration routes**: 10K events spanning 6 months (synthetic)
4. **Wildfire progression**: 5K events with hourly updates (real data)

---

## Documentation Plan

### API Documentation (rustdoc)

**Requirements**:
- Every public type, trait, function documented
- Examples for common use cases
- Links to related functionality
- Performance notes for key operations

**Example**:

```rust
/// Represents a geographic location using WGS84 coordinates.
///
/// Locations are the fundamental spatial unit in spatial narratives.
/// They support optional elevation and uncertainty for real-world data.
///
/// # Examples
///
/// ```
/// use spatial_narrative::Location;
///
/// // Create a location (New York City)
/// let nyc = Location::new(40.7128, -74.0060);
///
/// // With elevation
/// let peak = Location::with_elevation(27.9881, 86.9250, 8848.86);
///
/// // With uncertainty
/// let approximate = Location::builder()
///     .coordinates(40.7, -74.0)
///     .uncertainty_meters(1000.0)
///     .build();
/// ```
///
/// # Coordinate System
///
/// Locations use WGS84 (EPSG:4326):
/// - Latitude: -90° to +90° (negative = South)
/// - Longitude: -180° to +180° (negative = West)
/// - Elevation: meters above sea level (optional)
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    /// Latitude in decimal degrees
    pub lat: f64,
    /// Longitude in decimal degrees  
    pub lon: f64,
    /// Elevation in meters above sea level
    pub elevation: Option<f64>,
    /// Uncertainty radius in meters
    pub uncertainty_meters: Option<f64>,
}
```

### User Guide

**Chapters**:

1. **Getting Started**
   - Installation
   - First narrative
   - Basic concepts

2. **Loading Data**
   - GeoJSON import
   - CSV with custom columns
   - GPX trajectories
   - Parsing text

3. **Working with Narratives**
   - Building manually
   - Filtering and querying
   - Modifying events
   - Validation

4. **Spatial Analysis**
   - Computing metrics
   - Clustering events
   - Heatmap generation
   - Comparing narratives

5. **Graph Analysis**
   - Building graphs
   - Finding paths
   - Community detection
   - Visualization

6. **Advanced Topics**
   - Custom coordinate systems
   - Performance tuning
   - Parallel processing
   - Database backends

7. **CLI Tools**
   - Command reference
   - Common workflows
   - Scripting

### Cookbook

**Recipes**:

- **Recipe 1**: Import Twitter data with geotagged posts
- **Recipe 2**: Analyze protest movements across cities
- **Recipe 3**: Track hurricane progression from weather data
- **Recipe 4**: Build timeline visualization from news articles
- **Recipe 5**: Detect anomalous event patterns
- **Recipe 6**: Merge narratives from multiple sources
- **Recipe 7**: Export to interactive web map
- **Recipe 8**: Compute similarity between historical events

### Example Code

**Repository Structure**:

```
examples/
├── basic/
│   ├── hello_world.rs
│   ├── load_geojson.rs
│   └── simple_analysis.rs
├── intermediate/
│   ├── clustering.rs
│   ├── graph_analysis.rs
│   └── text_parsing.rs
└── advanced/
    ├── custom_format.rs
    ├── parallel_processing.rs
    └── web_service.rs
```

---

## Future Enhancements

### Version 2.0 Ideas

1. **Real-time streaming**: Process live event feeds
2. **Machine learning**: Predictive models for event propagation
3. **Multi-scale analysis**: Automatic zoom levels
4. **Network integration**: REST API and GraphQL
5. **Visualization**: Built-in map rendering
6. **Collaboration**: Multi-user editing with conflict resolution
7. **Provenance tracking**: Full audit trail of changes
8. **Privacy**: Anonymization and differential privacy

### Research Directions

1. **Spatial narrative similarity**: Novel metrics for comparing stories
2. **Automatic summarization**: Generate narrative summaries
3. **Event extraction**: ML-based extraction from unstructured text
4. **Causal inference**: Detect cause-effect relationships spatially
5. **Anomaly detection**: Identify unusual spatial-temporal patterns

---

## Contributing Guidelines

### Code Standards

- Follow Rust naming conventions
- Run `cargo fmt` and `cargo clippy` before committing
- Write documentation for public APIs
- Include tests for new functionality
- Benchmark performance-critical code

### Pull Request Process

1. Fork the repository
2. Create feature branch
3. Implement changes with tests
4. Update documentation
5. Submit PR with clear description
6. Address review feedback

### Communication

- GitHub Issues for bug reports
- GitHub Discussions for questions
- Monthly contributor calls
- Public roadmap

---

## License

Dual-licensed under MIT and Apache-2.0, allowing use in both open-source and commercial projects.

---

## Conclusion

This specification provides a comprehensive plan for building `spatial-narrative`, a Rust library for working with real-world spatial narratives. The phased implementation plan allows for iterative development with clear milestones, while the modular architecture ensures extensibility for future enhancements.

The focus on performance, correctness, and real-world use cases makes this library suitable for journalism, research, and data analysis applications where understanding how stories unfold across space and time is critical.

**Next Steps**:
1. Set up repository and CI/CD
2. Begin Phase 1 implementation
3. Recruit beta testers from journalism/research communities
4. Iterate based on user feedback