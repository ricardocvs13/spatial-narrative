//! Basic usage example demonstrating core types and operations.
//!
//! Run with: `cargo run --example basic_usage`

use spatial_narrative::core::{
    EventBuilder, GeoBounds, Location, NarrativeBuilder, SourceRef, SourceType, TimeRange,
    Timestamp,
};

fn main() {
    println!("=== Spatial Narrative - Basic Usage ===\n");

    // 1. Create locations
    let paris = Location::builder()
        .lat(48.8566)
        .lon(2.3522)
        .name("Paris, France")
        .build()
        .unwrap();

    let berlin = Location::builder()
        .lat(52.5200)
        .lon(13.4050)
        .name("Berlin, Germany")
        .build()
        .unwrap();

    let rome = Location::builder()
        .lat(41.9028)
        .lon(12.4964)
        .name("Rome, Italy")
        .elevation(21.0)
        .build()
        .unwrap();

    println!("Created locations:");
    println!("  - {:?}", paris);
    println!("  - {:?}", berlin);
    println!("  - {:?}", rome);
    println!();

    // 2. Create timestamps
    let ts1 = Timestamp::parse("1914-06-28T10:45:00Z").unwrap();
    let ts2 = Timestamp::parse("1918-11-11T11:00:00Z").unwrap();
    let ts3 = Timestamp::parse("1919-06-28T00:00:00Z").unwrap();

    println!("Created timestamps:");
    println!("  - {:?}", ts1);
    println!("  - {:?}", ts2);
    println!("  - {:?}", ts3);
    println!();

    // 3. Create source references
    let source = SourceRef::builder()
        .title("A History of World War I")
        .author("Historical Society")
        .source_type(SourceType::Archive)
        .url("https://example.com/ww1-history")
        .build();

    println!("Created source reference:");
    println!("  - {:?}", source);
    println!();

    // 4. Create events with the builder pattern
    let event1 = EventBuilder::new()
        .location(paris.clone())
        .timestamp(ts1.clone())
        .text("Significant historical event begins")
        .tag("history")
        .tag("europe")
        .source(source.clone())
        .build();

    let event2 = EventBuilder::new()
        .location(berlin.clone())
        .timestamp(ts2.clone())
        .text("Event concludes in Berlin")
        .tag("history")
        .build();

    let event3 = EventBuilder::new()
        .location(paris.clone())
        .timestamp(ts3.clone())
        .text("Treaty signed at Versailles")
        .tag("treaty")
        .tag("peace")
        .build();

    println!("Created events:");
    println!(
        "  - Event 1: {} @ {}",
        event1.text,
        event1.location.name.as_deref().unwrap_or("Unknown")
    );
    println!(
        "  - Event 2: {} @ {}",
        event2.text,
        event2.location.name.as_deref().unwrap_or("Unknown")
    );
    println!(
        "  - Event 3: {} @ {}",
        event3.text,
        event3.location.name.as_deref().unwrap_or("Unknown")
    );
    println!();

    // 5. Create a narrative
    let narrative = NarrativeBuilder::new()
        .title("World War I Overview")
        .description("A timeline of major WWI events")
        .author("Historical Research Team")
        .tag("history")
        .tag("world-war")
        .events(vec![event1.clone(), event2.clone(), event3.clone()])
        .build();

    println!("Created narrative: {}", narrative.title);
    println!("  - Events: {}", narrative.events.len());
    println!("  - Tags: {:?}", narrative.tags);
    println!();

    // 6. Query the narrative
    println!("=== Querying the Narrative ===\n");

    // Get chronological events
    let chronological = narrative.events_chronological();
    println!("Events in chronological order:");
    for event in chronological {
        println!(
            "  - {} | {}",
            event.timestamp.format_with_precision(),
            event.text
        );
    }
    println!();

    // Get time range
    if let Some(range) = narrative.time_range() {
        println!(
            "Time span: {} to {}",
            range.start.format_with_precision(),
            range.end.format_with_precision()
        );

        let duration = range.duration();
        let days = duration.num_days();
        let years = days / 365;
        println!("Duration: {} days ({} years)", days, years);
    }
    println!();

    // Get geographic bounds
    if let Some(bounds) = narrative.bounds() {
        println!("Geographic bounds:");
        println!("  - Lat: {:.4}° to {:.4}°", bounds.min_lat, bounds.max_lat);
        println!("  - Lon: {:.4}° to {:.4}°", bounds.min_lon, bounds.max_lon);
        let center = bounds.center();
        println!("  - Center: ({:.4}°, {:.4}°)", center.lat, center.lon);
    }
    println!();

    // 7. Spatial filtering
    println!("=== Spatial Filtering ===\n");

    // Filter to only Paris area
    let paris_bounds = GeoBounds::new(48.0, 2.0, 49.0, 3.0);
    let paris_events = narrative.filter_spatial(&paris_bounds);
    println!("Events in Paris area: {}", paris_events.len());
    for event in paris_events {
        println!("  - {}", event.text);
    }
    println!();

    // 8. Temporal filtering
    println!("=== Temporal Filtering ===\n");

    let range_1918 = TimeRange::year(1918);
    let events_1918 = narrative.filter_temporal(&range_1918);
    println!("Events in 1918: {}", events_1918.len());
    for event in events_1918 {
        println!(
            "  - {} | {}",
            event.timestamp.format_with_precision(),
            event.text
        );
    }
    println!();

    println!("=== Example Complete ===");
}
