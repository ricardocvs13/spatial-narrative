//! Graph representation of narrative events.
//!
//! Uses petgraph for the underlying graph structure.

use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::algo::{dijkstra, has_path_connecting};
use petgraph::Direction;

use crate::core::{Event, EventId, GeoBounds, TimeRange, Location};

/// Unique identifier for a node in the narrative graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub(crate) NodeIndex);

impl NodeId {
    /// Returns the raw index value.
    pub fn index(&self) -> usize {
        self.0.index()
    }
}

/// Type of relationship between events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeType {
    /// Temporal sequence (A happens before B)
    Temporal,
    /// Spatial proximity (A and B are geographically close)
    Spatial,
    /// Causal relationship (A causes B)
    Causal,
    /// Thematic connection (A and B share themes/tags)
    Thematic,
    /// Reference (A references B)
    Reference,
    /// Custom relationship type
    Custom,
}

impl Default for EdgeType {
    fn default() -> Self {
        Self::Temporal
    }
}

/// Weight/metadata for an edge in the graph.
#[derive(Debug, Clone)]
pub struct EdgeWeight {
    /// Type of relationship
    pub edge_type: EdgeType,
    /// Strength of the connection (0.0 to 1.0)
    pub weight: f64,
    /// Optional label for the edge
    pub label: Option<String>,
}

impl EdgeWeight {
    /// Create a new edge weight with given type.
    pub fn new(edge_type: EdgeType) -> Self {
        Self {
            edge_type,
            weight: 1.0,
            label: None,
        }
    }

    /// Create an edge weight with a specific weight value.
    pub fn with_weight(edge_type: EdgeType, weight: f64) -> Self {
        Self {
            edge_type,
            weight: weight.clamp(0.0, 1.0),
            label: None,
        }
    }

    /// Set a label on this edge.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Default for EdgeWeight {
    fn default() -> Self {
        Self::new(EdgeType::Temporal)
    }
}

/// A directed graph representing events and their relationships.
///
/// The graph uses petgraph internally and provides methods for:
/// - Adding events as nodes
/// - Connecting events with typed edges
/// - Querying paths and neighbors
/// - Subgraph extraction
#[derive(Debug)]
pub struct NarrativeGraph {
    /// The underlying directed graph
    graph: DiGraph<Event, EdgeWeight>,
    /// Map from EventId to NodeIndex for fast lookup
    id_map: HashMap<EventId, NodeIndex>,
}

impl NarrativeGraph {
    /// Create an empty narrative graph.
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            id_map: HashMap::new(),
        }
    }

    /// Create a graph from a vector of events.
    ///
    /// Events are added as nodes but no edges are created.
    pub fn from_events(events: impl IntoIterator<Item = Event>) -> Self {
        let mut graph = Self::new();
        for event in events {
            graph.add_event(event);
        }
        graph
    }

    /// Add an event as a node in the graph.
    ///
    /// Returns the NodeId for the newly added node.
    pub fn add_event(&mut self, event: Event) -> NodeId {
        let event_id = event.id.clone();
        let idx = self.graph.add_node(event);
        self.id_map.insert(event_id, idx);
        NodeId(idx)
    }

    /// Get the NodeId for an event by its EventId.
    pub fn get_node(&self, event_id: &EventId) -> Option<NodeId> {
        self.id_map.get(event_id).map(|&idx| NodeId(idx))
    }

    /// Get the event at a node.
    pub fn event(&self, node: NodeId) -> Option<&Event> {
        self.graph.node_weight(node.0)
    }

    /// Get a mutable reference to the event at a node.
    pub fn event_mut(&mut self, node: NodeId) -> Option<&mut Event> {
        self.graph.node_weight_mut(node.0)
    }

    /// Connect two events with an edge.
    pub fn connect(&mut self, from: NodeId, to: NodeId, edge_type: EdgeType) {
        self.graph.add_edge(from.0, to.0, EdgeWeight::new(edge_type));
    }

    /// Connect two events with a weighted edge.
    pub fn connect_weighted(&mut self, from: NodeId, to: NodeId, weight: EdgeWeight) {
        self.graph.add_edge(from.0, to.0, weight);
    }

    /// Check if two nodes are connected (directly).
    pub fn are_connected(&self, from: NodeId, to: NodeId) -> bool {
        self.graph.contains_edge(from.0, to.0)
    }

    /// Check if there's a path between two nodes.
    pub fn has_path(&self, from: NodeId, to: NodeId) -> bool {
        has_path_connecting(&self.graph, from.0, to.0, None)
    }

    /// Get all outgoing neighbors of a node.
    pub fn successors(&self, node: NodeId) -> Vec<NodeId> {
        self.graph
            .neighbors_directed(node.0, Direction::Outgoing)
            .map(NodeId)
            .collect()
    }

    /// Get all incoming neighbors of a node.
    pub fn predecessors(&self, node: NodeId) -> Vec<NodeId> {
        self.graph
            .neighbors_directed(node.0, Direction::Incoming)
            .map(NodeId)
            .collect()
    }

    /// Get the number of nodes.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get the number of edges.
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Check if the graph is empty.
    pub fn is_empty(&self) -> bool {
        self.graph.node_count() == 0
    }

    /// Iterate over all nodes.
    pub fn nodes(&self) -> impl Iterator<Item = (NodeId, &Event)> {
        self.graph.node_indices().filter_map(|idx| {
            self.graph.node_weight(idx).map(|event| (NodeId(idx), event))
        })
    }

    /// Iterate over all edges.
    pub fn edges(&self) -> impl Iterator<Item = (NodeId, NodeId, &EdgeWeight)> {
        self.graph.edge_references().map(|edge| {
            (NodeId(edge.source()), NodeId(edge.target()), edge.weight())
        })
    }

    /// Find the shortest path between two nodes.
    ///
    /// Returns path information including the sequence of nodes and total weight.
    pub fn shortest_path(&self, from: NodeId, to: NodeId) -> Option<PathInfo> {
        // Use Dijkstra with inverted weights (higher weight = lower cost)
        let costs = dijkstra(&self.graph, from.0, Some(to.0), |e| {
            1.0 - e.weight().weight
        });

        if !costs.contains_key(&to.0) {
            return None;
        }

        // Reconstruct path
        let mut path = vec![to];
        let mut current = to.0;

        while current != from.0 {
            let predecessors: Vec<_> = self.graph
                .neighbors_directed(current, Direction::Incoming)
                .collect();

            let best = predecessors.iter()
                .filter(|&&n| costs.contains_key(&n))
                .min_by(|&&a, &&b| {
                    costs[&a].partial_cmp(&costs[&b]).unwrap()
                });

            if let Some(&next) = best {
                path.push(NodeId(next));
                current = next;
            } else {
                break;
            }
        }

        path.reverse();

        Some(PathInfo {
            nodes: path,
            total_weight: costs[&to.0],
        })
    }

    /// Get edges of a specific type.
    pub fn edges_of_type(&self, edge_type: EdgeType) -> Vec<(NodeId, NodeId)> {
        self.graph
            .edge_references()
            .filter(|e| e.weight().edge_type == edge_type)
            .map(|e| (NodeId(e.source()), NodeId(e.target())))
            .collect()
    }

    /// Automatically connect events based on temporal sequence.
    ///
    /// Creates edges from earlier events to later events.
    pub fn connect_temporal(&mut self) {
        let mut nodes: Vec<_> = self.graph.node_indices()
            .filter_map(|idx| {
                self.graph.node_weight(idx).map(|e| (idx, e.timestamp.clone()))
            })
            .collect();

        nodes.sort_by(|a, b| a.1.cmp(&b.1));

        for window in nodes.windows(2) {
            if let [a, b] = window {
                if !self.graph.contains_edge(a.0, b.0) {
                    self.graph.add_edge(a.0, b.0, EdgeWeight::new(EdgeType::Temporal));
                }
            }
        }
    }

    /// Automatically connect events that are spatially close.
    ///
    /// Creates edges between events within the given distance threshold (in meters).
    pub fn connect_spatial(&mut self, max_distance_km: f64) {
        let nodes: Vec<_> = self.graph.node_indices()
            .filter_map(|idx| {
                self.graph.node_weight(idx).map(|e| (idx, e.location.clone()))
            })
            .collect();

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let dist = haversine_distance(&nodes[i].1, &nodes[j].1);
                if dist <= max_distance_km {
                    let weight = 1.0 - (dist / max_distance_km);
                    let edge = EdgeWeight::with_weight(EdgeType::Spatial, weight);
                    
                    // Add bidirectional edges for spatial proximity
                    if !self.graph.contains_edge(nodes[i].0, nodes[j].0) {
                        self.graph.add_edge(nodes[i].0, nodes[j].0, edge.clone());
                    }
                    if !self.graph.contains_edge(nodes[j].0, nodes[i].0) {
                        self.graph.add_edge(nodes[j].0, nodes[i].0, edge);
                    }
                }
            }
        }
    }

    /// Automatically connect events that share tags.
    pub fn connect_thematic(&mut self) {
        let nodes: Vec<_> = self.graph.node_indices()
            .filter_map(|idx| {
                self.graph.node_weight(idx).map(|e| (idx, e.tags.clone()))
            })
            .collect();

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let shared: usize = nodes[i].1.iter()
                    .filter(|t| nodes[j].1.contains(t))
                    .count();

                if shared > 0 {
                    let total = nodes[i].1.len().max(nodes[j].1.len());
                    let weight = shared as f64 / total as f64;
                    let edge = EdgeWeight::with_weight(EdgeType::Thematic, weight);
                    
                    // Add bidirectional edges for thematic similarity
                    if !self.graph.contains_edge(nodes[i].0, nodes[j].0) {
                        self.graph.add_edge(nodes[i].0, nodes[j].0, edge.clone());
                    }
                    if !self.graph.contains_edge(nodes[j].0, nodes[i].0) {
                        self.graph.add_edge(nodes[j].0, nodes[i].0, edge);
                    }
                }
            }
        }
    }

    /// Extract a subgraph containing only events within a time range.
    pub fn subgraph_temporal(&self, range: &TimeRange) -> SubgraphResult {
        let nodes: Vec<NodeId> = self.nodes()
            .filter(|(_, event)| range.contains(&event.timestamp))
            .map(|(id, _)| id)
            .collect();

        self.subgraph_from_nodes(&nodes)
    }

    /// Extract a subgraph containing only events within geographic bounds.
    pub fn subgraph_spatial(&self, bounds: &GeoBounds) -> SubgraphResult {
        let nodes: Vec<NodeId> = self.nodes()
            .filter(|(_, event)| bounds.contains(&event.location))
            .map(|(id, _)| id)
            .collect();

        self.subgraph_from_nodes(&nodes)
    }

    /// Extract a subgraph containing only specified nodes.
    fn subgraph_from_nodes(&self, nodes: &[NodeId]) -> SubgraphResult {
        let mut new_graph = NarrativeGraph::new();
        let mut id_map = HashMap::new();

        // Add nodes
        for &node_id in nodes {
            if let Some(event) = self.event(node_id) {
                let new_id = new_graph.add_event(event.clone());
                id_map.insert(node_id, new_id);
            }
        }

        // Add edges between included nodes
        for (from, to, weight) in self.edges() {
            if let (Some(&new_from), Some(&new_to)) = (id_map.get(&from), id_map.get(&to)) {
                new_graph.connect_weighted(new_from, new_to, weight.clone());
            }
        }

        SubgraphResult {
            graph: new_graph,
            node_mapping: id_map,
        }
    }

    /// Get the in-degree of a node (number of incoming edges).
    pub fn in_degree(&self, node: NodeId) -> usize {
        self.graph.edges_directed(node.0, Direction::Incoming).count()
    }

    /// Get the out-degree of a node (number of outgoing edges).
    pub fn out_degree(&self, node: NodeId) -> usize {
        self.graph.edges_directed(node.0, Direction::Outgoing).count()
    }

    /// Find nodes with no predecessors (roots/sources).
    pub fn roots(&self) -> Vec<NodeId> {
        self.graph.node_indices()
            .filter(|&idx| self.graph.edges_directed(idx, Direction::Incoming).count() == 0)
            .map(NodeId)
            .collect()
    }

    /// Find nodes with no successors (leaves/sinks).
    pub fn leaves(&self) -> Vec<NodeId> {
        self.graph.node_indices()
            .filter(|&idx| self.graph.edges_directed(idx, Direction::Outgoing).count() == 0)
            .map(NodeId)
            .collect()
    }

    // ========== Export Methods ==========

    /// Export the graph to DOT format (Graphviz).
    ///
    /// The DOT output can be rendered with Graphviz tools like `dot`, `neato`, etc.
    ///
    /// # Example
    ///
    /// ```rust
    /// use spatial_narrative::graph::{NarrativeGraph, EdgeType};
    /// use spatial_narrative::core::{Event, Location, Timestamp};
    ///
    /// let mut graph = NarrativeGraph::new();
    /// let e1 = Event::new(Location::new(40.7, -74.0), Timestamp::now(), "Event 1");
    /// let e2 = Event::new(Location::new(40.8, -74.1), Timestamp::now(), "Event 2");
    /// let n1 = graph.add_event(e1);
    /// let n2 = graph.add_event(e2);
    /// graph.connect(n1, n2, EdgeType::Temporal);
    ///
    /// let dot = graph.to_dot();
    /// assert!(dot.contains("digraph"));
    /// ```
    pub fn to_dot(&self) -> String {
        self.to_dot_with_options(DotOptions::default())
    }

    /// Export the graph to DOT format with custom options.
    pub fn to_dot_with_options(&self, options: DotOptions) -> String {
        let mut output = String::new();
        output.push_str("digraph NarrativeGraph {\n");
        
        // Graph attributes
        output.push_str(&format!("    rankdir={};\n", options.rank_direction));
        output.push_str(&format!("    node [shape={}, fontname=\"{}\"];\n", 
            options.node_shape, options.font_name));
        output.push_str(&format!("    edge [fontname=\"{}\"];\n", options.font_name));
        output.push('\n');

        // Nodes
        for idx in self.graph.node_indices() {
            let event = &self.graph[idx];
            let label = Self::escape_dot_string(&Self::truncate_text(&event.text, 30));
            let tooltip = Self::escape_dot_string(&format!(
                "{}\\n({:.4}, {:.4})\\n{}",
                event.text,
                event.location.lat,
                event.location.lon,
                event.timestamp.to_rfc3339()
            ));
            
            // Color by edge type if connected
            let color = self.get_node_color(NodeId(idx));
            
            output.push_str(&format!(
                "    n{} [label=\"{}\", tooltip=\"{}\", fillcolor=\"{}\", style=filled];\n",
                idx.index(), label, tooltip, color
            ));
        }
        
        output.push('\n');

        // Edges
        for edge in self.graph.edge_references() {
            let weight = edge.weight();
            let color = Self::edge_type_color(&weight.edge_type);
            let style = Self::edge_type_style(&weight.edge_type);
            let label = weight.label.as_deref().unwrap_or("");
            
            output.push_str(&format!(
                "    n{} -> n{} [color=\"{}\", style={}, label=\"{}\", penwidth={}];\n",
                edge.source().index(),
                edge.target().index(),
                color,
                style,
                Self::escape_dot_string(label),
                1.0 + weight.weight * 2.0
            ));
        }

        output.push_str("}\n");
        output
    }

    /// Export the graph to JSON format.
    ///
    /// Returns a JSON object with nodes and edges arrays.
    pub fn to_json(&self) -> String {
        let nodes: Vec<serde_json::Value> = self.graph.node_indices()
            .map(|idx| {
                let event = &self.graph[idx];
                serde_json::json!({
                    "id": idx.index(),
                    "event_id": event.id.to_string(),
                    "text": event.text,
                    "location": {
                        "lat": event.location.lat,
                        "lon": event.location.lon,
                        "elevation": event.location.elevation,
                        "name": event.location.name
                    },
                    "timestamp": event.timestamp.to_rfc3339(),
                    "tags": event.tags
                })
            })
            .collect();

        let edges: Vec<serde_json::Value> = self.graph.edge_references()
            .map(|edge| {
                let weight = edge.weight();
                serde_json::json!({
                    "source": edge.source().index(),
                    "target": edge.target().index(),
                    "type": format!("{:?}", weight.edge_type),
                    "weight": weight.weight,
                    "label": weight.label
                })
            })
            .collect();

        serde_json::json!({
            "nodes": nodes,
            "edges": edges,
            "metadata": {
                "node_count": self.node_count(),
                "edge_count": self.edge_count()
            }
        }).to_string()
    }

    /// Export to JSON with pretty printing.
    pub fn to_json_pretty(&self) -> String {
        let nodes: Vec<serde_json::Value> = self.graph.node_indices()
            .map(|idx| {
                let event = &self.graph[idx];
                serde_json::json!({
                    "id": idx.index(),
                    "event_id": event.id.to_string(),
                    "text": event.text,
                    "location": {
                        "lat": event.location.lat,
                        "lon": event.location.lon,
                        "elevation": event.location.elevation,
                        "name": event.location.name
                    },
                    "timestamp": event.timestamp.to_rfc3339(),
                    "tags": event.tags
                })
            })
            .collect();

        let edges: Vec<serde_json::Value> = self.graph.edge_references()
            .map(|edge| {
                let weight = edge.weight();
                serde_json::json!({
                    "source": edge.source().index(),
                    "target": edge.target().index(),
                    "type": format!("{:?}", weight.edge_type),
                    "weight": weight.weight,
                    "label": weight.label
                })
            })
            .collect();

        serde_json::to_string_pretty(&serde_json::json!({
            "nodes": nodes,
            "edges": edges,
            "metadata": {
                "node_count": self.node_count(),
                "edge_count": self.edge_count()
            }
        })).unwrap_or_default()
    }

    // Helper methods for DOT export
    fn escape_dot_string(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
    }

    fn truncate_text(text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[..max_len.saturating_sub(3)])
        }
    }

    fn get_node_color(&self, node: NodeId) -> &'static str {
        // Color based on connectivity
        let in_deg = self.in_degree(node);
        let out_deg = self.out_degree(node);
        
        if in_deg == 0 && out_deg > 0 {
            "#90EE90" // Light green for roots
        } else if out_deg == 0 && in_deg > 0 {
            "#FFB6C1" // Light pink for leaves  
        } else if in_deg > 2 || out_deg > 2 {
            "#87CEEB" // Light blue for hubs
        } else {
            "#FFFACD" // Light yellow for regular nodes
        }
    }

    fn edge_type_color(edge_type: &EdgeType) -> &'static str {
        match edge_type {
            EdgeType::Temporal => "#2E86AB",  // Blue
            EdgeType::Spatial => "#A23B72",   // Magenta
            EdgeType::Causal => "#F18F01",    // Orange
            EdgeType::Thematic => "#C73E1D",  // Red
            EdgeType::Reference => "#6B8E23", // Olive
            EdgeType::Custom => "#808080",    // Gray
        }
    }

    fn edge_type_style(edge_type: &EdgeType) -> &'static str {
        match edge_type {
            EdgeType::Temporal => "solid",
            EdgeType::Spatial => "dashed",
            EdgeType::Causal => "bold",
            EdgeType::Thematic => "dotted",
            EdgeType::Reference => "solid",
            EdgeType::Custom => "solid",
        }
    }
}

/// Options for DOT export formatting.
#[derive(Debug, Clone)]
pub struct DotOptions {
    /// Graph rank direction: "TB" (top-bottom), "LR" (left-right), "BT", "RL"
    pub rank_direction: String,
    /// Node shape: "box", "ellipse", "circle", "diamond", etc.
    pub node_shape: String,
    /// Font name for labels
    pub font_name: String,
}

impl Default for DotOptions {
    fn default() -> Self {
        Self {
            rank_direction: "TB".to_string(),
            node_shape: "box".to_string(),
            font_name: "Arial".to_string(),
        }
    }
}

impl DotOptions {
    /// Create options for left-to-right layout (good for timelines).
    pub fn timeline() -> Self {
        Self {
            rank_direction: "LR".to_string(),
            node_shape: "box".to_string(),
            font_name: "Arial".to_string(),
        }
    }

    /// Create options for hierarchical layout.
    pub fn hierarchical() -> Self {
        Self {
            rank_direction: "TB".to_string(),
            node_shape: "ellipse".to_string(),
            font_name: "Arial".to_string(),
        }
    }
}

impl Default for NarrativeGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a path through the graph.
#[derive(Debug, Clone)]
pub struct PathInfo {
    /// Sequence of nodes in the path.
    pub nodes: Vec<NodeId>,
    /// Total weight/cost of the path.
    pub total_weight: f64,
}

impl PathInfo {
    /// Get the number of nodes in the path.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Check if the path is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

/// Result of subgraph extraction.
#[derive(Debug)]
pub struct SubgraphResult {
    /// The extracted subgraph.
    pub graph: NarrativeGraph,
    /// Mapping from original NodeIds to new NodeIds.
    pub node_mapping: HashMap<NodeId, NodeId>,
}

/// Calculate haversine distance between two locations in kilometers.
fn haversine_distance(loc1: &Location, loc2: &Location) -> f64 {
    let r = 6371.0; // Earth radius in km

    let lat1 = loc1.lat.to_radians();
    let lat2 = loc2.lat.to_radians();
    let dlat = (loc2.lat - loc1.lat).to_radians();
    let dlon = (loc2.lon - loc1.lon).to_radians();

    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    r * c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Timestamp;

    fn make_event(lat: f64, lon: f64, time: &str, text: &str) -> Event {
        Event::new(
            Location::new(lat, lon),
            Timestamp::parse(time).unwrap(),
            text,
        )
    }

    #[test]
    fn test_graph_new() {
        let graph = NarrativeGraph::new();
        assert!(graph.is_empty());
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_graph_add_event() {
        let mut graph = NarrativeGraph::new();
        let event = make_event(40.7128, -74.0060, "2024-01-01T12:00:00Z", "NYC Event");
        
        let node = graph.add_event(event.clone());
        
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.event(node).unwrap().text, "NYC Event");
    }

    #[test]
    fn test_graph_connect() {
        let mut graph = NarrativeGraph::new();
        let n1 = graph.add_event(make_event(40.7, -74.0, "2024-01-01T10:00:00Z", "Event 1"));
        let n2 = graph.add_event(make_event(40.7, -74.0, "2024-01-01T12:00:00Z", "Event 2"));
        
        graph.connect(n1, n2, EdgeType::Temporal);
        
        assert!(graph.are_connected(n1, n2));
        assert!(!graph.are_connected(n2, n1)); // Directed!
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_graph_successors_predecessors() {
        let mut graph = NarrativeGraph::new();
        let n1 = graph.add_event(make_event(40.7, -74.0, "2024-01-01T10:00:00Z", "Event 1"));
        let n2 = graph.add_event(make_event(40.7, -74.0, "2024-01-01T12:00:00Z", "Event 2"));
        let n3 = graph.add_event(make_event(40.7, -74.0, "2024-01-01T14:00:00Z", "Event 3"));
        
        graph.connect(n1, n2, EdgeType::Temporal);
        graph.connect(n1, n3, EdgeType::Temporal);
        
        assert_eq!(graph.successors(n1).len(), 2);
        assert_eq!(graph.predecessors(n2).len(), 1);
        assert_eq!(graph.predecessors(n1).len(), 0);
    }

    #[test]
    fn test_graph_connect_temporal() {
        let mut graph = NarrativeGraph::new();
        graph.add_event(make_event(40.7, -74.0, "2024-01-01T14:00:00Z", "Third"));
        graph.add_event(make_event(40.7, -74.0, "2024-01-01T10:00:00Z", "First"));
        graph.add_event(make_event(40.7, -74.0, "2024-01-01T12:00:00Z", "Second"));
        
        graph.connect_temporal();
        
        // Should have edges: First -> Second -> Third
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn test_graph_connect_thematic() {
        let mut graph = NarrativeGraph::new();
        
        let mut e1 = make_event(40.7, -74.0, "2024-01-01T10:00:00Z", "Event 1");
        e1.add_tag("politics");
        e1.add_tag("economy");
        
        let mut e2 = make_event(40.7, -74.0, "2024-01-01T12:00:00Z", "Event 2");
        e2.add_tag("politics");
        
        let mut e3 = make_event(40.7, -74.0, "2024-01-01T14:00:00Z", "Event 3");
        e3.add_tag("sports");
        
        graph.add_event(e1);
        graph.add_event(e2);
        graph.add_event(e3);
        
        graph.connect_thematic();
        
        // Only e1 and e2 share tags
        let thematic_edges = graph.edges_of_type(EdgeType::Thematic);
        assert_eq!(thematic_edges.len(), 2); // Bidirectional
    }

    #[test]
    fn test_graph_roots_leaves() {
        let mut graph = NarrativeGraph::new();
        let n1 = graph.add_event(make_event(40.7, -74.0, "2024-01-01T10:00:00Z", "Root"));
        let n2 = graph.add_event(make_event(40.7, -74.0, "2024-01-01T12:00:00Z", "Middle"));
        let n3 = graph.add_event(make_event(40.7, -74.0, "2024-01-01T14:00:00Z", "Leaf"));
        
        graph.connect(n1, n2, EdgeType::Temporal);
        graph.connect(n2, n3, EdgeType::Temporal);
        
        let roots = graph.roots();
        let leaves = graph.leaves();
        
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0], n1);
        assert_eq!(leaves.len(), 1);
        assert_eq!(leaves[0], n3);
    }

    #[test]
    fn test_haversine_distance() {
        let nyc = Location::new(40.7128, -74.0060);
        let la = Location::new(34.0522, -118.2437);
        
        let distance = haversine_distance(&nyc, &la);
        
        // NYC to LA is roughly 3940 km
        assert!(distance > 3900.0 && distance < 4000.0);
    }
}
