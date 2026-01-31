//! Example demonstrating spatial and temporal indexing.
//!
//! Run with: `cargo run --example indexing`

use spatial_narrative::core::{GeoBounds, Location, TimeRange, Timestamp};
use spatial_narrative::index::{GridSpec, SpatialIndex, SpatiotemporalIndex, TemporalIndex};

fn main() {
    println!("=== Spatial Narrative - Indexing ===\n");

    // 1. Spatial Index Example
    println!("--- Spatial Index (R-tree) ---\n");
    demonstrate_spatial_index();

    // 2. Temporal Index Example
    println!("\n--- Temporal Index (B-tree) ---\n");
    demonstrate_temporal_index();

    // 3. Spatiotemporal Index Example
    println!("\n--- Spatiotemporal Index ---\n");
    demonstrate_spatiotemporal_index();

    // 4. Heatmap Generation
    println!("\n--- Heatmap Generation ---\n");
    demonstrate_heatmap();

    println!("\n=== Indexing Example Complete ===");
}

fn demonstrate_spatial_index() {
    // Create index for city names
    let mut index: SpatialIndex<&str> = SpatialIndex::new();

    // Add cities across the world
    let cities = vec![
        ("New York", 40.7128, -74.0060),
        ("Los Angeles", 34.0522, -118.2437),
        ("Chicago", 41.8781, -87.6298),
        ("Houston", 29.7604, -95.3698),
        ("Phoenix", 33.4484, -112.0740),
        ("Philadelphia", 39.9526, -75.1652),
        ("San Antonio", 29.4241, -98.4936),
        ("San Diego", 32.7157, -117.1611),
        ("Dallas", 32.7767, -96.7970),
        ("San Jose", 37.3382, -121.8863),
    ];

    for (name, lat, lon) in &cities {
        index.insert(*name, &Location::new(*lat, *lon));
    }

    println!("Indexed {} cities", index.len());

    // Bounding box query: West Coast
    let west_coast = index.query_bbox(32.0, -125.0, 38.0, -115.0);
    println!("\nWest Coast cities (lat 32-38, lon -125 to -115):");
    for city in west_coast {
        println!("  - {}", city);
    }

    // Nearest neighbor: What city is closest to Denver?
    let denver = (39.7392, -104.9903);
    if let Some(nearest) = index.nearest_one(denver.0, denver.1) {
        println!("\nNearest city to Denver: {}", nearest);
    }

    // K-nearest neighbors
    let k_nearest = index.nearest(denver.0, denver.1, 3);
    println!("\n3 nearest cities to Denver:");
    for city in k_nearest {
        println!("  - {}", city);
    }
}

fn demonstrate_temporal_index() {
    // Create index for event descriptions
    let mut index: TemporalIndex<&str> = TemporalIndex::new();

    // Add events throughout a day
    let events = vec![
        ("Wake up", "2024-03-20T07:00:00Z"),
        ("Morning coffee", "2024-03-20T07:30:00Z"),
        ("Start work", "2024-03-20T09:00:00Z"),
        ("Team meeting", "2024-03-20T10:00:00Z"),
        ("Lunch break", "2024-03-20T12:00:00Z"),
        ("Code review", "2024-03-20T14:00:00Z"),
        ("End work", "2024-03-20T17:00:00Z"),
        ("Dinner", "2024-03-20T19:00:00Z"),
        ("Reading", "2024-03-20T21:00:00Z"),
        ("Sleep", "2024-03-20T23:00:00Z"),
    ];

    for (desc, ts) in &events {
        let timestamp = Timestamp::parse(ts).unwrap();
        index.insert(*desc, &timestamp);
    }

    println!("Indexed {} events", index.len());

    // Range query: Work hours (9 AM to 5 PM)
    let work_start = Timestamp::parse("2024-03-20T09:00:00Z").unwrap();
    let work_end = Timestamp::parse("2024-03-20T17:00:00Z").unwrap();
    let work_range = TimeRange::new(work_start, work_end);

    let work_events = index.query_range(&work_range);
    println!("\nEvents during work hours (9 AM - 5 PM):");
    for event in work_events {
        println!("  - {}", event);
    }

    // Before/After queries
    let noon = Timestamp::parse("2024-03-20T12:00:00Z").unwrap();

    let morning_events = index.before(&noon);
    println!("\nEvents before noon:");
    for event in morning_events {
        println!("  - {}", event);
    }

    // First and last
    if let Some(first) = index.first() {
        println!("\nFirst event: {}", first);
    }
    if let Some(last) = index.last() {
        println!("Last event: {}", last);
    }
}

fn demonstrate_spatiotemporal_index() {
    // Create a spatiotemporal index tracking delivery truck locations
    let mut index: SpatiotemporalIndex<String> = SpatiotemporalIndex::new();

    // Simulate a delivery route
    let route = vec![
        (
            "Warehouse departure",
            40.7128,
            -74.0060,
            "2024-03-20T08:00:00Z",
        ),
        (
            "Stop 1: Brooklyn",
            40.6782,
            -73.9442,
            "2024-03-20T08:45:00Z",
        ),
        ("Stop 2: Queens", 40.7282, -73.7949, "2024-03-20T09:30:00Z"),
        ("Stop 3: Bronx", 40.8448, -73.8648, "2024-03-20T10:15:00Z"),
        (
            "Lunch break: Manhattan",
            40.7580,
            -73.9855,
            "2024-03-20T12:00:00Z",
        ),
        (
            "Stop 4: Staten Island",
            40.5795,
            -74.1502,
            "2024-03-20T14:00:00Z",
        ),
        (
            "Stop 5: Jersey City",
            40.7178,
            -74.0431,
            "2024-03-20T15:30:00Z",
        ),
        (
            "Return to warehouse",
            40.7128,
            -74.0060,
            "2024-03-20T17:00:00Z",
        ),
    ];

    for (desc, lat, lon, ts) in &route {
        let location = Location::new(*lat, *lon);
        let timestamp = Timestamp::parse(ts).unwrap();
        index.insert(desc.to_string(), &location, &timestamp);
    }

    println!("Indexed {} delivery stops", index.len());

    // Query: Stops in Manhattan area during morning
    let manhattan_bounds = GeoBounds::new(40.7, -74.02, 40.8, -73.9);
    let morning = TimeRange::new(
        Timestamp::parse("2024-03-20T08:00:00Z").unwrap(),
        Timestamp::parse("2024-03-20T12:00:00Z").unwrap(),
    );

    let morning_manhattan = index.query(&manhattan_bounds, &morning);
    println!("\nMorning stops in Manhattan area:");
    for stop in morning_manhattan {
        println!("  - {}", stop);
    }

    // Spatial-only query
    let brooklyn_queens = GeoBounds::new(40.6, -74.0, 40.75, -73.7);
    let bk_queens_stops = index.query_spatial(&brooklyn_queens);
    println!("\nAll stops in Brooklyn/Queens area:");
    for stop in bk_queens_stops {
        println!("  - {}", stop);
    }

    // Temporal-only query
    let afternoon = TimeRange::new(
        Timestamp::parse("2024-03-20T13:00:00Z").unwrap(),
        Timestamp::parse("2024-03-20T18:00:00Z").unwrap(),
    );
    let afternoon_stops = index.query_temporal(&afternoon);
    println!("\nAfternoon stops:");
    for stop in afternoon_stops {
        println!("  - {}", stop);
    }
}

fn demonstrate_heatmap() {
    // Create an index with many events to visualize density
    let mut index: SpatiotemporalIndex<i32> = SpatiotemporalIndex::new();

    // Simulate event clusters in different areas
    let clusters = vec![
        // High-density cluster in Manhattan (20 events)
        (40.7580, -73.9855, 20),
        // Medium-density in Brooklyn (10 events)
        (40.6782, -73.9442, 10),
        // Low-density in Queens (5 events)
        (40.7282, -73.7949, 5),
        // Very low in Bronx (2 events)
        (40.8448, -73.8648, 2),
    ];

    let mut event_id = 0;
    let base_ts = Timestamp::parse("2024-03-20T12:00:00Z").unwrap();

    for (lat, lon, count) in clusters {
        for i in 0..count {
            // Add slight variation to locations
            let lat_offset = (i as f64 * 0.001) - (count as f64 * 0.0005);
            let lon_offset = (i as f64 * 0.0015) - (count as f64 * 0.00075);

            index.insert(
                event_id,
                &Location::new(lat + lat_offset, lon + lon_offset),
                &base_ts,
            );
            event_id += 1;
        }
    }

    println!("Indexed {} events for heatmap", index.len());

    // Generate heatmap
    let bounds = GeoBounds::new(40.5, -74.2, 40.9, -73.7);
    let grid = GridSpec::new(bounds, 5, 5);
    let heatmap = index.heatmap(grid);

    println!("\nHeatmap visualization (5x5 grid):");
    println!("Legend: . = 0, + = 1-3, * = 4-7, # = 8+\n");

    let grid_2d = heatmap.to_grid();
    for row in grid_2d.iter().rev() {
        print!("  ");
        for &count in row {
            let symbol = match count {
                0 => '.',
                1..=3 => '+',
                4..=7 => '*',
                _ => '#',
            };
            print!("{} ", symbol);
        }
        println!();
    }

    println!("\nMax events in a cell: {}", heatmap.max_count);
    println!(
        "Hotspot normalized value: {:.2}",
        heatmap.get_normalized(2, 2)
    );
}
