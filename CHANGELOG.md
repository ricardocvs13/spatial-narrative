# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-01-31

### Added

#### Core Module (`spatial_narrative::core`)
- `Location` struct with latitude, longitude, elevation, uncertainty, and name fields
- `LocationBuilder` for fluent location construction with validation
- `Timestamp` struct with timezone-aware datetime and precision tracking
- `TemporalPrecision` enum (Year, Month, Day, Hour, Minute, Second, Millisecond)
- `Event` struct representing spatiotemporal occurrences
- `EventId` UUID-based unique identifier
- `EventBuilder` for fluent event construction
- `Narrative` struct for collections of related events
- `NarrativeId` and `NarrativeMetadata` for narrative organization
- `NarrativeBuilder` for fluent narrative construction
- `SourceRef` and `SourceType` for data provenance tracking
- `GeoBounds` for geographic bounding boxes
- `TimeRange` for temporal intervals
- `SpatialEntity` and `TemporalEntity` traits

#### I/O Module (`spatial_narrative::io`)
- `Format` trait for import/export abstraction
- `GeoJsonFormat` with configurable options
- `CsvFormat` with flexible column mapping
- `JsonFormat` with version checking
- Round-trip serialization support

#### Index Module (`spatial_narrative::index`)
- `SpatialIndex<T>` using R-tree (rstar) for O(log n) spatial queries
- `TemporalIndex<T>` using B-tree for O(log n) temporal queries
- `SpatiotemporalIndex<T>` for combined space-time queries
- `GridSpec` and `Heatmap` for density visualization data
- Bounding box, radius, and k-nearest neighbor queries
- Time range, before/after, and chronological queries

#### Graph Module (`spatial_narrative::graph`)
- `NarrativeGraph` using petgraph for event relationship modeling
- `NodeId` for graph node references
- `EdgeType` enum (Temporal, Spatial, Causal, Thematic, Reference, Custom)
- `EdgeWeight` for weighted connections
- Automatic connection strategies: temporal, spatial, thematic
- Path finding with Dijkstra's algorithm
- Subgraph extraction by time range or geographic bounds
- Graph structure analysis (roots, leaves, degrees)

#### Examples
- `basic_usage` — Core types and operations
- `io_formats` — Format import/export
- `indexing` — Spatial and temporal indexing

#### Documentation
- Comprehensive README with API overview
- Contributing guidelines
- Module-level documentation
- Doc examples for all public types

#### Infrastructure
- GitHub Actions CI/CD pipeline
- rustfmt configuration
- clippy configuration
- MIT License

### Dependencies
- chrono 0.4 — Date and time handling
- serde 1.0 — Serialization framework
- serde_json 1.0 — JSON support
- csv 1.3 — CSV parsing
- uuid 1.10 — Unique identifiers
- geo 0.28 — Geospatial primitives
- rstar 0.12 — R-tree spatial indexing
- petgraph 0.6 — Graph data structures
- rayon 1.10 — Parallel processing
- thiserror 1.0 — Error handling

## [Unreleased]

### Planned
- Analysis module with clustering and trajectory extraction
- GPX format support
- Streaming import for large files
- DOT export for graph visualization
- Community detection algorithms
- Geoparsing from unstructured text
