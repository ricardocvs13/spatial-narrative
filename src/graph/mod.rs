//! Graph representation of narratives.
//!
//! This module provides tools for representing narratives as
//! directed graphs where events are nodes and relationships
//! (temporal, spatial, thematic) are edges.
//!
//! # Example
//!
//! ```rust
//! use spatial_narrative::graph::{NarrativeGraph, EdgeType};
//! use spatial_narrative::core::{Event, Location, Timestamp};
//!
//! // Create events
//! let e1 = Event::new(Location::new(40.7, -74.0), Timestamp::now(), "Event 1");
//! let e2 = Event::new(Location::new(40.7, -74.0), Timestamp::now(), "Event 2");
//!
//! // Build graph
//! let mut graph = NarrativeGraph::new();
//! let n1 = graph.add_event(e1);
//! let n2 = graph.add_event(e2);
//! graph.connect(n1, n2, EdgeType::Temporal);
//!
//! assert_eq!(graph.node_count(), 2);
//! assert_eq!(graph.edge_count(), 1);
//! ```

mod narrative_graph;

pub use narrative_graph::{
    DotOptions, EdgeType, EdgeWeight, NarrativeGraph, NodeId, PathInfo, SubgraphResult,
};
