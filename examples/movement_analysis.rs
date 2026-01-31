//! Movement Analysis Example
//!
//! Demonstrates trajectory analysis, stop detection, velocity profiles,
//! and path simplification.
//!
//! Run with: `cargo run --example movement_analysis`

use spatial_narrative::analysis::{detect_stops, MovementAnalyzer, StopThreshold, Trajectory};
use spatial_narrative::core::{Event, Location, Timestamp};

fn main() {
    println!("=== Movement Analysis Example ===\n");

    // Create events representing a delivery route with stops
    let events = vec![
        // Start at depot
        Event::new(
            Location::new(40.7128, -74.0060),
            Timestamp::parse("2024-01-20T08:00:00Z").unwrap(),
            "Left depot",
        ),
        // Moving to first stop
        Event::new(
            Location::new(40.7200, -74.0000),
            Timestamp::parse("2024-01-20T08:15:00Z").unwrap(),
            "En route",
        ),
        // First delivery stop (multiple pings at same location)
        Event::new(
            Location::new(40.7300, -73.9900),
            Timestamp::parse("2024-01-20T08:30:00Z").unwrap(),
            "Arrived at stop 1",
        ),
        Event::new(
            Location::new(40.7301, -73.9901),
            Timestamp::parse("2024-01-20T08:45:00Z").unwrap(),
            "Delivering package",
        ),
        Event::new(
            Location::new(40.7302, -73.9902),
            Timestamp::parse("2024-01-20T09:00:00Z").unwrap(),
            "Package delivered",
        ),
        // Moving to second stop
        Event::new(
            Location::new(40.7400, -73.9800),
            Timestamp::parse("2024-01-20T09:15:00Z").unwrap(),
            "En route",
        ),
        Event::new(
            Location::new(40.7500, -73.9700),
            Timestamp::parse("2024-01-20T09:30:00Z").unwrap(),
            "En route",
        ),
        // Second delivery stop
        Event::new(
            Location::new(40.7580, -73.9855),
            Timestamp::parse("2024-01-20T09:45:00Z").unwrap(),
            "Arrived at Times Square",
        ),
        Event::new(
            Location::new(40.7581, -73.9856),
            Timestamp::parse("2024-01-20T10:00:00Z").unwrap(),
            "Searching for address",
        ),
        Event::new(
            Location::new(40.7582, -73.9857),
            Timestamp::parse("2024-01-20T10:15:00Z").unwrap(),
            "Delivered at Times Square",
        ),
        Event::new(
            Location::new(40.7583, -73.9858),
            Timestamp::parse("2024-01-20T10:30:00Z").unwrap(),
            "Leaving Times Square",
        ),
        // Return to depot
        Event::new(
            Location::new(40.7400, -74.0000),
            Timestamp::parse("2024-01-20T11:00:00Z").unwrap(),
            "Heading back",
        ),
        Event::new(
            Location::new(40.7128, -74.0060),
            Timestamp::parse("2024-01-20T11:30:00Z").unwrap(),
            "Returned to depot",
        ),
    ];

    let separator = "-".repeat(40);

    // === Create Trajectory ===
    println!("🚚 Trajectory Analysis");
    println!("{}", separator);

    let trajectory = Trajectory::new("delivery_route", events.clone());

    println!("Trajectory ID: {}", trajectory.id);
    println!("Total points: {}", trajectory.len());
    println!(
        "Total distance: {:.2} km",
        trajectory.total_distance() / 1000.0
    );
    println!(
        "Duration: {:.1} hours",
        trajectory.duration_secs() / 3600.0
    );
    println!(
        "Average speed: {:.1} km/h",
        trajectory.avg_speed() * 3.6 // m/s to km/h
    );

    if let Some(bounds) = trajectory.bounds() {
        println!(
            "Coverage: ({:.4}, {:.4}) to ({:.4}, {:.4})",
            bounds.min_lat, bounds.min_lon, bounds.max_lat, bounds.max_lon
        );
    }

    // === Velocity Profile ===
    println!("\n📈 Velocity Profile");
    println!("{}", separator);

    let velocity = trajectory.velocity_profile();
    for (i, (ts, speed)) in velocity.iter().enumerate().take(5) {
        let speed_kmh = speed * 3.6;
        let time = ts.to_string();
        let time_part = time.split('T').nth(1).unwrap_or(&time);
        println!(
            "Segment {}: {} - {:.1} km/h",
            i + 1,
            &time_part[..8],
            speed_kmh
        );
    }
    if velocity.len() > 5 {
        println!("  ... and {} more segments", velocity.len() - 5);
    }

    // === Stop Detection ===
    println!("\n🛑 Stop Detection");
    println!("{}", separator);

    let threshold = StopThreshold {
        radius_m: 100.0,          // 100 meters
        min_duration_secs: 600.0, // 10 minutes
    };

    let stops = detect_stops(&trajectory, &threshold);
    println!(
        "Detection params: radius={}m, min_duration={}min",
        threshold.radius_m,
        threshold.min_duration_secs / 60.0
    );
    println!("Stops detected: {}", stops.len());

    for (i, stop) in stops.iter().enumerate() {
        println!(
            "\nStop {}: ({:.4}, {:.4})",
            i + 1,
            stop.location.lat,
            stop.location.lon
        );
        println!("  Duration: {:.0} minutes", stop.duration_secs / 60.0);
        println!("  Events during stop: {}", stop.event_count);
        println!("  Time: {} → {}", stop.start, stop.end);
    }

    // === Movement Analyzer ===
    println!("\n🔍 Movement Analyzer");
    println!("{}", separator);

    let analyzer = MovementAnalyzer::with_stop_threshold(StopThreshold {
        radius_m: 100.0,
        min_duration_secs: 600.0,
    });

    let traj = analyzer.extract_trajectory("route", events);
    let detected_stops = analyzer.detect_stops(&traj);
    let segments = analyzer.movement_segments(&traj);

    println!("Trajectory length: {} points", traj.len());
    println!("Stops found: {}", detected_stops.len());
    println!("Movement segments: {}", segments.len());

    for (i, seg) in segments.iter().enumerate() {
        println!(
            "  Segment {}: {} points, {:.2} km",
            i + 1,
            seg.len(),
            seg.total_distance() / 1000.0
        );
    }

    // === Path Simplification ===
    println!("\n✂️ Path Simplification (Douglas-Peucker)");
    println!("{}", separator);

    let original_len = trajectory.len();
    let simplified = trajectory.simplify(500.0); // 500m tolerance

    println!("Original points: {}", original_len);
    println!("Simplified points: {}", simplified.len());
    println!(
        "Reduction: {:.1}%",
        (1.0 - simplified.len() as f64 / original_len as f64) * 100.0
    );

    // Compare distances
    println!(
        "Original distance: {:.2} km",
        trajectory.total_distance() / 1000.0
    );
    println!(
        "Simplified distance: {:.2} km",
        simplified.total_distance() / 1000.0
    );

    println!("\n✅ Movement analysis complete!");
}
