//! Spatial Analysis Example
//!
//! Demonstrates spatial metrics, distance calculations, bearing,
//! and density mapping with the analysis module.
//!
//! Run with: `cargo run --example spatial_analysis`

use spatial_narrative::analysis::{
    bearing, density_map, destination_point, haversine_distance, SpatialMetrics,
};
use spatial_narrative::core::{Event, Location, Narrative, Timestamp};

fn main() {
    println!("=== Spatial Analysis Example ===\n");

    // Create a narrative representing a road trip from NYC to Washington DC
    let events = vec![
        Event::new(
            Location::new(40.7128, -74.0060), // NYC
            Timestamp::parse("2024-01-15T08:00:00Z").unwrap(),
            "Departed from New York City",
        ),
        Event::new(
            Location::new(40.2206, -74.7597), // Trenton, NJ
            Timestamp::parse("2024-01-15T09:30:00Z").unwrap(),
            "Stopped for coffee in Trenton",
        ),
        Event::new(
            Location::new(39.9526, -75.1652), // Philadelphia
            Timestamp::parse("2024-01-15T10:30:00Z").unwrap(),
            "Visited the Liberty Bell",
        ),
        Event::new(
            Location::new(39.2904, -76.6122), // Baltimore
            Timestamp::parse("2024-01-15T12:30:00Z").unwrap(),
            "Lunch in Baltimore",
        ),
        Event::new(
            Location::new(38.9072, -77.0369), // Washington DC
            Timestamp::parse("2024-01-15T14:00:00Z").unwrap(),
            "Arrived in Washington DC",
        ),
    ];

    let mut narrative = Narrative::new("NYC to DC Road Trip");
    for event in events.clone() {
        narrative.events.push(event);
    }

    // === Haversine Distance ===
    let separator = "-".repeat(40);
    println!("📍 Haversine Distance Calculation");
    println!("{}", separator);

    let nyc = Location::new(40.7128, -74.0060);
    let dc = Location::new(38.9072, -77.0369);

    // Note: haversine_distance takes lat/lon as separate f64 arguments
    let distance = haversine_distance(nyc.lat, nyc.lon, dc.lat, dc.lon);
    println!("NYC to Washington DC:");
    println!(
        "  Distance: {:.2} km ({:.2} miles)",
        distance / 1000.0,
        distance / 1609.34
    );

    // Calculate leg distances
    println!("\nLeg Distances:");
    let locations: Vec<_> = events.iter().map(|e| &e.location).collect();
    let leg_names = [
        "NYC→Trenton",
        "Trenton→Philly",
        "Philly→Baltimore",
        "Baltimore→DC",
    ];

    for (i, name) in leg_names.iter().enumerate() {
        let loc1 = locations[i];
        let loc2 = locations[i + 1];
        let dist = haversine_distance(loc1.lat, loc1.lon, loc2.lat, loc2.lon);
        println!("  {}: {:.1} km", name, dist / 1000.0);
    }

    // === Bearing Calculation ===
    println!("\n🧭 Bearing Calculation");
    println!("{}", separator);

    // Note: bearing takes lat/lon as separate f64 arguments
    let b = bearing(nyc.lat, nyc.lon, dc.lat, dc.lon);
    println!("Bearing from NYC to DC: {:.1}°", b);

    // Determine cardinal direction
    let direction = match b {
        b if b < 22.5 || b >= 337.5 => "North",
        b if b < 67.5 => "Northeast",
        b if b < 112.5 => "East",
        b if b < 157.5 => "Southeast",
        b if b < 202.5 => "South",
        b if b < 247.5 => "Southwest",
        b if b < 292.5 => "West",
        _ => "Northwest",
    };
    println!("Cardinal direction: {}", direction);

    // === Destination Point ===
    println!("\n🎯 Destination Point");
    println!("{}", separator);

    let start = Location::new(40.7128, -74.0060); // NYC
    let heading = 180.0; // Due south
    let dist = 100_000.0; // 100 km

    // Note: destination_point returns (lat, lon) tuple
    let (dest_lat, dest_lon) = destination_point(start.lat, start.lon, heading, dist);
    println!("Starting from NYC, heading South for 100 km:");
    println!("  Destination: ({:.4}, {:.4})", dest_lat, dest_lon);

    // === Spatial Metrics ===
    println!("\n📊 Spatial Metrics (from events)");
    println!("{}", separator);

    let metrics = SpatialMetrics::from_events(&events);

    println!(
        "Total distance traveled: {:.2} km",
        metrics.total_distance / 1000.0
    );

    if let Some(centroid) = &metrics.centroid {
        println!("Centroid: ({:.4}, {:.4})", centroid.lat, centroid.lon);
    }

    if let Some(bounds) = &metrics.bounds {
        println!("Bounding box:");
        println!("  North: {:.4}°", bounds.max_lat);
        println!("  South: {:.4}°", bounds.min_lat);
        println!("  East: {:.4}°", bounds.max_lon);
        println!("  West: {:.4}°", bounds.min_lon);
    }

    println!("Average distance between stops: {:.2} km", metrics.avg_distance / 1000.0);
    println!("Max distance between stops: {:.2} km", metrics.max_distance / 1000.0);
    println!("Dispersion from centroid: {:.2} km", metrics.dispersion / 1000.0);

    // === Density Map ===
    println!("\n🗺️ Density Map");
    println!("{}", separator);

    // More events for a meaningful density map
    let city_events = vec![
        // Manhattan cluster
        Event::new(
            Location::new(40.7580, -73.9855),
            Timestamp::now(),
            "Times Square",
        ),
        Event::new(
            Location::new(40.7484, -73.9857),
            Timestamp::now(),
            "Empire State",
        ),
        Event::new(
            Location::new(40.7614, -73.9776),
            Timestamp::now(),
            "MoMA",
        ),
        Event::new(
            Location::new(40.7527, -73.9772),
            Timestamp::now(),
            "Grand Central",
        ),
        Event::new(
            Location::new(40.7411, -73.9897),
            Timestamp::now(),
            "Flatiron",
        ),
        // Brooklyn cluster
        Event::new(
            Location::new(40.6892, -73.9857),
            Timestamp::now(),
            "Prospect Park",
        ),
        Event::new(
            Location::new(40.6782, -73.9442),
            Timestamp::now(),
            "Brooklyn Museum",
        ),
    ];

    let grid = density_map(&city_events, 5, 5); // 5x5 grid

    println!("Density Grid (5x5):");
    println!("  Total cells: {}", grid.len());

    let max_count = grid.iter().map(|c| c.count).max().unwrap_or(0);
    println!("  Max events in a cell: {}", max_count);

    // Print non-empty cells
    let non_empty: Vec<_> = grid.iter().filter(|c| c.count > 0).collect();
    println!("  Non-empty cells: {}", non_empty.len());

    for cell in non_empty.iter().take(3) {
        println!(
            "  Cell at ({:.4}, {:.4}): {} events, density: {:.2}/km²",
            cell.lat, cell.lon, cell.count, cell.density
        );
    }

    println!("\n✅ Spatial analysis complete!");
}
