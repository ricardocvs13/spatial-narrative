//! Graph Visualization Export Example
//!
//! This example demonstrates how to build a narrative graph and export it
//! to DOT format (Graphviz) and JSON for visualization in external tools.
//!
//! Run with: `cargo run --example graph_export`

use spatial_narrative::core::{Event, Location, Timestamp};
use spatial_narrative::graph::{DotOptions, EdgeType, NarrativeGraph};

fn main() {
    println!("=== Spatial Narrative - Graph Export ===\n");

    // Create events for a news story timeline
    let events = create_events();

    // Build the graph
    let mut graph = NarrativeGraph::from_events(events);

    println!("Graph created with {} nodes", graph.node_count());

    // Auto-connect by temporal sequence
    graph.connect_temporal();
    println!("After temporal connection: {} edges", graph.edge_count());

    // Connect events within 10km of each other
    graph.connect_spatial(10.0);
    println!("After spatial connection: {} edges", graph.edge_count());

    // Connect events sharing tags
    graph.connect_thematic();
    println!("After thematic connection: {} edges\n", graph.edge_count());

    // === DOT Export ===
    println!("=== DOT Export (Graphviz) ===\n");

    // Default DOT format
    let dot = graph.to_dot();
    println!("Default DOT output (first 500 chars):");
    println!("{}\n", &dot[..dot.len().min(500)]);

    // Timeline layout (left-to-right)
    let timeline_dot = graph.to_dot_with_options(DotOptions::timeline());
    println!("Timeline DOT starts with: {}", &timeline_dot[..100]);

    // To visualize: save to file and run `dot -Tpng graph.dot -o graph.png`
    // Or paste into https://dreampuf.github.io/GraphvizOnline/

    // === JSON Export ===
    println!("\n=== JSON Export ===\n");

    let json = graph.to_json_pretty();
    println!("JSON output (first 800 chars):");
    println!("{}\n", &json[..json.len().min(800)]);

    // This JSON can be used with:
    // - D3.js for web visualization
    // - Cytoscape.js for network graphs
    // - Sigma.js for large graphs
    // - Any other graph visualization library

    // === Graph Analysis ===
    println!("=== Graph Structure ===\n");

    let roots = graph.roots();
    let leaves = graph.leaves();

    println!("Root nodes (story beginnings): {}", roots.len());
    for root in &roots {
        if let Some(event) = graph.event(*root) {
            println!(
                "  - {} ({})",
                truncate(&event.text, 40),
                event.timestamp.to_rfc3339()
            );
        }
    }

    println!("\nLeaf nodes (story endings): {}", leaves.len());
    for leaf in &leaves {
        if let Some(event) = graph.event(*leaf) {
            println!(
                "  - {} ({})",
                truncate(&event.text, 40),
                event.timestamp.to_rfc3339()
            );
        }
    }

    // Edge type breakdown
    println!("\nEdge breakdown by type:");
    println!(
        "  - Temporal: {}",
        graph.edges_of_type(EdgeType::Temporal).len()
    );
    println!(
        "  - Spatial: {}",
        graph.edges_of_type(EdgeType::Spatial).len()
    );
    println!(
        "  - Thematic: {}",
        graph.edges_of_type(EdgeType::Thematic).len()
    );

    println!("\n=== Example Complete ===");
    println!("\nTo visualize the DOT output:");
    println!("  1. Save DOT to a file: graph.dot");
    println!("  2. Run: dot -Tpng graph.dot -o graph.png");
    println!("  3. Or paste at: https://dreampuf.github.io/GraphvizOnline/");
}

fn create_events() -> Vec<Event> {
    let mut events = Vec::new();

    // A fictional news story about a conference and its aftermath
    let mut e1 = Event::new(
        Location::builder()
            .lat(40.7128)
            .lon(-74.0060)
            .name("New York City")
            .build()
            .unwrap(),
        Timestamp::parse("2024-03-01T09:00:00Z").unwrap(),
        "Tech Summit 2024 opens in NYC with keynote on AI safety",
    );
    e1.add_tag("conference");
    e1.add_tag("AI");
    events.push(e1);

    let mut e2 = Event::new(
        Location::builder()
            .lat(40.7580)
            .lon(-73.9855)
            .name("Times Square")
            .build()
            .unwrap(),
        Timestamp::parse("2024-03-01T14:00:00Z").unwrap(),
        "Protesters gather at Times Square over AI regulation",
    );
    e2.add_tag("protest");
    e2.add_tag("AI");
    events.push(e2);

    let mut e3 = Event::new(
        Location::builder()
            .lat(40.7128)
            .lon(-74.0060)
            .name("New York City")
            .build()
            .unwrap(),
        Timestamp::parse("2024-03-01T18:00:00Z").unwrap(),
        "Summit concludes with policy recommendations",
    );
    e3.add_tag("conference");
    e3.add_tag("policy");
    events.push(e3);

    let mut e4 = Event::new(
        Location::builder()
            .lat(38.9072)
            .lon(-77.0369)
            .name("Washington DC")
            .build()
            .unwrap(),
        Timestamp::parse("2024-03-02T10:00:00Z").unwrap(),
        "Congress announces AI oversight committee",
    );
    e4.add_tag("policy");
    e4.add_tag("government");
    events.push(e4);

    let mut e5 = Event::new(
        Location::builder()
            .lat(37.7749)
            .lon(-122.4194)
            .name("San Francisco")
            .build()
            .unwrap(),
        Timestamp::parse("2024-03-02T15:00:00Z").unwrap(),
        "Major tech companies respond to summit recommendations",
    );
    e5.add_tag("technology");
    e5.add_tag("AI");
    events.push(e5);

    events
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
