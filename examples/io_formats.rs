//! Example demonstrating I/O operations with different formats.
//!
//! Run with: `cargo run --example io_formats`

use spatial_narrative::core::{
    EventBuilder, Location, Narrative, NarrativeBuilder, Timestamp,
};
use spatial_narrative::io::{
    Format, GeoJsonFormat, GeoJsonOptions, 
    CsvFormat, CsvOptions,
    JsonFormat,
};

fn main() {
    println!("=== Spatial Narrative - I/O Formats ===\n");

    // Create a sample narrative
    let narrative = create_sample_narrative();
    println!("Created narrative with {} events\n", narrative.events.len());

    // 1. JSON Format
    println!("--- JSON Format ---\n");
    demonstrate_json_format(&narrative);

    // 2. GeoJSON Format  
    println!("\n--- GeoJSON Format ---\n");
    demonstrate_geojson_format(&narrative);

    // 3. CSV Format
    println!("\n--- CSV Format ---\n");
    demonstrate_csv_format(&narrative);

    println!("\n=== I/O Example Complete ===");
}

fn create_sample_narrative() -> Narrative {
    let events = vec![
        EventBuilder::new()
            .location(Location::new(51.5074, -0.1278))
            .timestamp(Timestamp::parse("2024-03-15T09:00:00Z").unwrap())
            .text("Morning conference in London")
            .tag("meeting")
            .build(),
        EventBuilder::new()
            .location(Location::new(48.8566, 2.3522))
            .timestamp(Timestamp::parse("2024-03-15T14:00:00Z").unwrap())
            .text("Afternoon workshop in Paris")
            .tag("workshop")
            .build(),
        EventBuilder::new()
            .location(Location::new(52.5200, 13.4050))
            .timestamp(Timestamp::parse("2024-03-16T10:00:00Z").unwrap())
            .text("Final presentation in Berlin")
            .tag("presentation")
            .build(),
    ];

    NarrativeBuilder::new()
        .title("European Conference Tour")
        .author("Research Team")
        .events(events)
        .build()
}

fn demonstrate_json_format(narrative: &Narrative) {
    // Export with pretty printing
    let json_format = JsonFormat::pretty();

    let mut output = Vec::new();
    match json_format.export(narrative, &mut output) {
        Ok(_) => {
            let json_str = String::from_utf8_lossy(&output);
            println!("JSON export (first 500 chars):");
            println!("{}", &json_str.chars().take(500).collect::<String>());
            if json_str.len() > 500 {
                println!("... [truncated]");
            }
            println!("\nTotal JSON size: {} bytes", output.len());
        }
        Err(e) => println!("JSON export error: {}", e),
    }

    // Demonstrate round-trip
    let reimported: Result<Narrative, _> = json_format.import(&mut output.as_slice());
    match reimported {
        Ok(n) => println!("Round-trip successful! Events: {}", n.events.len()),
        Err(e) => println!("Round-trip failed: {}", e),
    }
}

fn demonstrate_geojson_format(narrative: &Narrative) {
    let geojson_format = GeoJsonFormat::with_options(GeoJsonOptions {
        include_ids: true,
        include_tags: true,
        include_sources: true,
        timestamp_property: "timestamp".to_string(),
        text_property: "description".to_string(),
    });

    let mut output = Vec::new();
    match geojson_format.export(narrative, &mut output) {
        Ok(_) => {
            let json_str = String::from_utf8_lossy(&output);
            println!("GeoJSON export:");
            println!("{}", &json_str.chars().take(600).collect::<String>());
            if json_str.len() > 600 {
                println!("... [truncated]");
            }
            println!("\nGeoJSON is compatible with mapping tools like:");
            println!("  - Leaflet.js");
            println!("  - Mapbox GL JS");
            println!("  - QGIS");
            println!("  - Google Earth");
        }
        Err(e) => println!("GeoJSON export error: {}", e),
    }
}

fn demonstrate_csv_format(narrative: &Narrative) {
    // Default CSV format
    let csv_format = CsvFormat::new();

    let mut output = Vec::new();
    match csv_format.export(narrative, &mut output) {
        Ok(_) => {
            let csv_str = String::from_utf8_lossy(&output);
            println!("CSV export:");
            println!("{}", csv_str);
            println!("CSV is ideal for:");
            println!("  - Spreadsheet analysis (Excel, Google Sheets)");
            println!("  - Data science workflows (pandas, R)");
            println!("  - Database imports");
        }
        Err(e) => println!("CSV export error: {}", e),
    }

    // Custom delimiter (TSV)
    println!("\nTab-separated (TSV) format:");
    let tsv_format = CsvFormat::with_options(CsvOptions {
        delimiter: b'\t',
        ..Default::default()
    });

    let mut tsv_output = Vec::new();
    if tsv_format.export(narrative, &mut tsv_output).is_ok() {
        let tsv_str = String::from_utf8_lossy(&tsv_output);
        for line in tsv_str.lines().take(4) {
            println!("{}", line);
        }
    }
}
