//! Narrative Comparison Example
//!
//! Demonstrates comparing spatial narratives for similarity
//! across spatial, temporal, and thematic dimensions.
//!
//! Run with: `cargo run --example comparison`

use spatial_narrative::analysis::{
    common_locations, compare_narratives, spatial_intersection, spatial_similarity,
    spatial_union, temporal_similarity, thematic_similarity, ComparisonConfig,
    NarrativeSimilarity,
};
use spatial_narrative::core::{Event, Location, Narrative, Timestamp};

fn main() {
    println!("=== Narrative Comparison Example ===\n");

    // Create two narratives: Alice's and Bob's NYC trips

    // Alice's NYC Trip - Tourist focused
    let mut alice_events = vec![
        Event::new(
            Location::new(40.7484, -73.9857), // Empire State Building
            Timestamp::parse("2024-01-20T09:00:00Z").unwrap(),
            "Empire State Building",
        ),
        Event::new(
            Location::new(40.7580, -73.9855), // Times Square
            Timestamp::parse("2024-01-20T11:00:00Z").unwrap(),
            "Times Square",
        ),
        Event::new(
            Location::new(40.7614, -73.9776), // MoMA
            Timestamp::parse("2024-01-20T14:00:00Z").unwrap(),
            "Museum of Modern Art",
        ),
        Event::new(
            Location::new(40.7794, -73.9632), // Met Museum
            Timestamp::parse("2024-01-20T16:00:00Z").unwrap(),
            "Metropolitan Museum",
        ),
        Event::new(
            Location::new(40.7812, -73.9665), // Central Park
            Timestamp::parse("2024-01-20T18:00:00Z").unwrap(),
            "Central Park walk",
        ),
    ];

    // Add tags to Alice's events
    alice_events[0].tags.push("landmark".to_string());
    alice_events[0].tags.push("tourism".to_string());
    alice_events[1].tags.push("entertainment".to_string());
    alice_events[1].tags.push("tourism".to_string());
    alice_events[2].tags.push("art".to_string());
    alice_events[2].tags.push("museum".to_string());
    alice_events[3].tags.push("art".to_string());
    alice_events[3].tags.push("museum".to_string());
    alice_events[4].tags.push("nature".to_string());
    alice_events[4].tags.push("relaxation".to_string());

    let mut alice = Narrative::new("Alice's NYC Adventure");
    for event in alice_events {
        alice.events.push(event);
    }

    // Bob's NYC Trip - Business focused but overlapping locations
    let mut bob_events = vec![
        Event::new(
            Location::new(40.7580, -73.9855), // Times Square
            Timestamp::parse("2024-01-20T08:30:00Z").unwrap(),
            "Morning coffee near Times Square",
        ),
        Event::new(
            Location::new(40.7527, -73.9772), // Grand Central
            Timestamp::parse("2024-01-20T09:30:00Z").unwrap(),
            "Meeting at Grand Central",
        ),
        Event::new(
            Location::new(40.7484, -73.9857), // Empire State Building
            Timestamp::parse("2024-01-20T12:00:00Z").unwrap(),
            "Lunch near Empire State",
        ),
        Event::new(
            Location::new(40.7614, -73.9776), // MoMA area
            Timestamp::parse("2024-01-20T15:00:00Z").unwrap(),
            "Client meeting midtown",
        ),
        Event::new(
            Location::new(40.7282, -74.0776), // Financial District
            Timestamp::parse("2024-01-20T17:00:00Z").unwrap(),
            "Downtown office",
        ),
    ];

    bob_events[0].tags.push("coffee".to_string());
    bob_events[0].tags.push("business".to_string());
    bob_events[1].tags.push("meeting".to_string());
    bob_events[1].tags.push("business".to_string());
    bob_events[2].tags.push("food".to_string());
    bob_events[2].tags.push("landmark".to_string());
    bob_events[3].tags.push("meeting".to_string());
    bob_events[3].tags.push("business".to_string());
    bob_events[4].tags.push("office".to_string());
    bob_events[4].tags.push("business".to_string());

    let mut bob = Narrative::new("Bob's NYC Business Trip");
    for event in bob_events {
        bob.events.push(event);
    }

    let separator = "-".repeat(40);

    println!("📚 Narratives to Compare:");
    println!("{}", separator);
    println!("Alice's trip: {} events", alice.events.len());
    for e in &alice.events {
        println!("  - {}", e.text);
    }
    println!("\nBob's trip: {} events", bob.events.len());
    for e in &bob.events {
        println!("  - {}", e.text);
    }

    // === Individual Similarity Metrics ===
    println!("\n📏 Individual Similarity Metrics");
    println!("{}", separator);

    // Spatial similarity
    let spatial_sim = spatial_similarity(alice.events(), bob.events(), 500.0); // 500m tolerance
    println!(
        "Spatial Similarity: {:.1}% (within 500m tolerance)",
        spatial_sim * 100.0
    );

    // Temporal similarity (same day, overlapping hours)
    let temporal_sim = temporal_similarity(alice.events(), bob.events());
    println!("Temporal Similarity: {:.1}%", temporal_sim * 100.0);

    // Thematic similarity (tag overlap)
    let thematic_sim = thematic_similarity(alice.events(), bob.events());
    println!("Thematic Similarity: {:.1}%", thematic_sim * 100.0);

    // === Common Locations ===
    println!("\n📍 Common Locations (500m tolerance)");
    println!("{}", separator);

    let common = common_locations(&alice, &bob, 500.0);
    println!("Found {} common location pairs:", common.len());
    for (alice_idx, bob_idx) in &common {
        let alice_event = &alice.events[*alice_idx];
        let bob_event = &bob.events[*bob_idx];
        println!(
            "  Alice: \"{}\" ≈ Bob: \"{}\"",
            alice_event.text, bob_event.text
        );
    }

    // === Spatial Operations ===
    println!("\n🗺️ Spatial Operations");
    println!("{}", separator);

    let intersection = spatial_intersection(&alice, &bob, 500.0);
    println!("Spatial Intersection: {} events from Alice", intersection.len());
    for e in &intersection {
        println!("  - {}", e.text);
    }

    let union = spatial_union(&alice, &bob);
    if let Some(bounds) = union {
        println!(
            "\nSpatial Union Bounds: ({:.4}, {:.4}) to ({:.4}, {:.4})",
            bounds.min_lat, bounds.min_lon, bounds.max_lat, bounds.max_lon
        );
    }

    // === Full Comparison with Config ===
    println!("\n⚙️ Configurable Comparison");
    println!("{}", separator);

    // Create comparison config
    let config = ComparisonConfig {
        spatial_weight: 0.5,      // Prioritize spatial similarity
        temporal_weight: 0.3,
        thematic_weight: 0.2,
        location_threshold_m: 300.0, // Tighter tolerance
    };

    println!(
        "Weights: spatial={}, temporal={}, thematic={}",
        config.spatial_weight, config.temporal_weight, config.thematic_weight
    );
    println!("Distance tolerance: {}m", config.location_threshold_m);

    let similarity = compare_narratives(&alice, &bob, &config);
    print_similarity(&similarity);

    // === Different Configurations ===
    println!("\n📊 Comparing Different Weighting Strategies");
    println!("{}", separator);

    // Spatial-focused
    let spatial_focus = ComparisonConfig {
        spatial_weight: 1.0,
        temporal_weight: 0.0,
        thematic_weight: 0.0,
        location_threshold_m: 500.0,
    };

    // Temporal-focused
    let temporal_focus = ComparisonConfig {
        spatial_weight: 0.0,
        temporal_weight: 1.0,
        thematic_weight: 0.0,
        location_threshold_m: 500.0,
    };

    // Thematic-focused
    let thematic_focus = ComparisonConfig {
        spatial_weight: 0.0,
        temporal_weight: 0.0,
        thematic_weight: 1.0,
        location_threshold_m: 500.0,
    };

    let spatial_result = compare_narratives(&alice, &bob, &spatial_focus);
    let temporal_result = compare_narratives(&alice, &bob, &temporal_focus);
    let thematic_result = compare_narratives(&alice, &bob, &thematic_focus);

    println!(
        "Spatial-only score: {:.1}%",
        spatial_result.overall * 100.0
    );
    println!(
        "Temporal-only score: {:.1}%",
        temporal_result.overall * 100.0
    );
    println!(
        "Thematic-only score: {:.1}%",
        thematic_result.overall * 100.0
    );

    // === Self Comparison ===
    println!("\n🔄 Self Comparison (should be 100%)");
    println!("{}", separator);

    let self_sim = compare_narratives(
        &alice,
        &alice,
        &ComparisonConfig {
            spatial_weight: 0.33,
            temporal_weight: 0.33,
            thematic_weight: 0.34,
            location_threshold_m: 0.0,
        },
    );
    println!(
        "Alice vs Alice: {:.1}% overall similarity",
        self_sim.overall * 100.0
    );

    println!("\n✅ Comparison analysis complete!");
}

fn print_similarity(sim: &NarrativeSimilarity) {
    println!("\nSimilarity Results:");
    println!("  Spatial:  {:.1}%", sim.spatial * 100.0);
    println!("  Temporal: {:.1}%", sim.temporal * 100.0);
    println!("  Thematic: {:.1}%", sim.thematic * 100.0);
    println!("  ─────────────────");
    println!("  Overall:  {:.1}%", sim.overall * 100.0);
}
