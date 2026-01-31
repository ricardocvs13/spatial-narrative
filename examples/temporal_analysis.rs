//! Temporal Analysis Example
//!
//! Demonstrates temporal metrics including event rates, gap detection,
//! burst detection, and time binning.
//!
//! Run with: `cargo run --example temporal_analysis`

use spatial_narrative::analysis::{detect_bursts, detect_gaps, event_rate, TemporalMetrics, TimeBin};
use spatial_narrative::core::{Event, Location, Narrative, Timestamp};

fn main() {
    println!("=== Temporal Analysis Example ===\n");

    // Create events simulating social media activity over a day
    let events = vec![
        // Morning activity (sparse)
        Event::new(
            Location::new(40.7128, -74.0060),
            Timestamp::parse("2024-01-20T08:00:00Z").unwrap(),
            "Good morning NYC!",
        ),
        Event::new(
            Location::new(40.7128, -74.0060),
            Timestamp::parse("2024-01-20T08:30:00Z").unwrap(),
            "Coffee time",
        ),
        // Gap - no activity for several hours
        // Lunch burst (rapid activity)
        Event::new(
            Location::new(40.7580, -73.9855),
            Timestamp::parse("2024-01-20T12:00:00Z").unwrap(),
            "Lunch at Times Square",
        ),
        Event::new(
            Location::new(40.7580, -73.9855),
            Timestamp::parse("2024-01-20T12:02:00Z").unwrap(),
            "The food is amazing!",
        ),
        Event::new(
            Location::new(40.7580, -73.9855),
            Timestamp::parse("2024-01-20T12:05:00Z").unwrap(),
            "Street performer!",
        ),
        Event::new(
            Location::new(40.7580, -73.9855),
            Timestamp::parse("2024-01-20T12:08:00Z").unwrap(),
            "So much energy here",
        ),
        Event::new(
            Location::new(40.7580, -73.9855),
            Timestamp::parse("2024-01-20T12:10:00Z").unwrap(),
            "Photo dump incoming",
        ),
        // Another gap
        // Evening activity
        Event::new(
            Location::new(40.7614, -73.9776),
            Timestamp::parse("2024-01-20T18:00:00Z").unwrap(),
            "Evening at the museum",
        ),
        Event::new(
            Location::new(40.7614, -73.9776),
            Timestamp::parse("2024-01-20T19:30:00Z").unwrap(),
            "Art is life",
        ),
        Event::new(
            Location::new(40.7128, -74.0060),
            Timestamp::parse("2024-01-20T22:00:00Z").unwrap(),
            "Home sweet home",
        ),
    ];

    let mut narrative = Narrative::new("A Day in NYC - Social Media");
    for event in events.clone() {
        narrative.events.push(event);
    }

    let separator = "-".repeat(40);

    // === Temporal Metrics ===
    println!("📊 Temporal Metrics");
    println!("{}", separator);

    let metrics = TemporalMetrics::from_events(&events);

    println!("Total events: {}", metrics.event_count);
    println!("Duration: {:.2} hours", metrics.duration_secs / 3600.0);

    if let Some(range) = &metrics.time_range {
        println!("First event: {}", range.start);
        println!("Last event: {}", range.end);
    }

    println!(
        "Average time between events: {:.1} minutes",
        metrics.avg_inter_event_time / 60.0
    );
    println!(
        "Min gap: {:.1} minutes",
        metrics.min_inter_event_time / 60.0
    );
    println!(
        "Max gap: {:.1} hours",
        metrics.max_inter_event_time / 3600.0
    );

    // === Event Rate (Time Binning) ===
    println!("\n⏱️ Event Rate Analysis");
    println!("{}", separator);

    // Get events per hour
    let hourly_bins = event_rate(&events, TimeBin::Hour);

    println!("Hourly event distribution:");
    for bin in &hourly_bins {
        // Extract hour from the start timestamp
        let ts_str = bin.start.to_string();
        let time_part = ts_str.split('T').nth(1).unwrap_or(&ts_str);
        let hour = &time_part[..2];
        println!("  {}:00 - {} events", hour, bin.count);
    }

    // Calculate overall rate
    let total_events = events.len();
    let total_hours = metrics.duration_secs / 3600.0;
    println!(
        "\nOverall rate: {:.2} events/hour",
        total_events as f64 / total_hours
    );

    // === Gap Detection ===
    println!("\n🕳️ Gap Detection (>1 hour gaps)");
    println!("{}", separator);

    let gaps = detect_gaps(&events, 3600.0); // gaps > 1 hour
    println!("Found {} significant gaps:", gaps.len());

    for gap in &gaps {
        // Calculate gap duration
        let start_ms = gap.start.to_unix_millis();
        let end_ms = gap.end.to_unix_millis();
        let duration_hours = (end_ms - start_ms) as f64 / 3_600_000.0;

        println!("  {} → {} ({:.1} hours)", gap.start, gap.end, duration_hours);
    }

    // === Burst Detection ===
    println!("\n💥 Burst Detection");
    println!("{}", separator);

    // Detect bursts: min 3 events within 15 minutes (900 seconds)
    let bursts = detect_bursts(&events, 900.0, 3);
    println!(
        "Found {} bursts (3+ events within 15 minutes):",
        bursts.len()
    );

    for (i, burst) in bursts.iter().enumerate() {
        // Calculate burst duration and event count
        let start_ms = burst.start.to_unix_millis();
        let end_ms = burst.end.to_unix_millis();
        let duration_mins = (end_ms - start_ms) as f64 / 60_000.0;

        // Count events in this burst
        let events_in_burst = events
            .iter()
            .filter(|e| {
                let ts = e.timestamp.to_unix_millis();
                ts >= start_ms && ts <= end_ms
            })
            .count();

        println!(
            "\nBurst {}: {} events over {:.1} minutes",
            i + 1,
            events_in_burst,
            duration_mins
        );
        println!("  Time: {} → {}", burst.start, burst.end);

        if duration_mins > 0.0 {
            println!(
                "  Rate: {:.1} events/minute",
                events_in_burst as f64 / duration_mins
            );
        }
    }

    // === Custom Time Analysis ===
    println!("\n📈 Activity Pattern Analysis");
    println!("{}", separator);

    // Analyze activity by extracting hour from timestamp strings
    let morning_count = events
        .iter()
        .filter(|e| {
            let ts_str = e.timestamp.to_string();
            let time_part = ts_str.split('T').nth(1).unwrap_or("00");
            let hour: u32 = time_part[..2].parse().unwrap_or(0);
            hour >= 6 && hour < 12
        })
        .count();
    let afternoon_count = events
        .iter()
        .filter(|e| {
            let ts_str = e.timestamp.to_string();
            let time_part = ts_str.split('T').nth(1).unwrap_or("00");
            let hour: u32 = time_part[..2].parse().unwrap_or(0);
            hour >= 12 && hour < 18
        })
        .count();
    let evening_count = events
        .iter()
        .filter(|e| {
            let ts_str = e.timestamp.to_string();
            let time_part = ts_str.split('T').nth(1).unwrap_or("00");
            let hour: u32 = time_part[..2].parse().unwrap_or(0);
            hour >= 18 || hour < 6
        })
        .count();

    println!("Morning (6am-12pm): {} events", morning_count);
    println!("Afternoon (12pm-6pm): {} events", afternoon_count);
    println!("Evening (6pm-6am): {} events", evening_count);

    let most_active = if afternoon_count >= morning_count && afternoon_count >= evening_count {
        "Afternoon"
    } else if morning_count >= evening_count {
        "Morning"
    } else {
        "Evening"
    };
    println!("\nMost active period: {}", most_active);

    println!("\n✅ Temporal analysis complete!");
}
