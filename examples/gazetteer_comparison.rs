//! Gazetteer Comparison Example
//!
//! This example demonstrates using multiple gazetteer sources for place name resolution.
//!
//! Run with:
//! ```bash
//! # Basic (built-in only)
//! cargo run --example gazetteer_comparison
//!
//! # With API gazetteers (requires network access)
//! cargo run --example gazetteer_comparison --features geocoding
//!
//! # With GeoNames (requires username)
//! GEONAMES_USERNAME=your_username cargo run --example gazetteer_comparison --features geocoding
//! ```

use spatial_narrative::parser::{BuiltinGazetteer, Gazetteer, MultiGazetteer};

#[cfg(feature = "geocoding")]
use spatial_narrative::parser::{GazetteerNominatim, GazetteerWikidata, GazetteerGeoNames};

fn main() {
    println!("=== Spatial Narrative Gazetteer Comparison ===\n");

    let test_places = vec![
        "London",
        "Paris", 
        "Tokyo",
        "New York City",
        "Berlin",
        "Sydney",
    ];

    // Test 1: Built-in Gazetteer
    println!("--- Built-in Gazetteer ---");
    let builtin = BuiltinGazetteer::new();
    test_gazetteer(&builtin, &test_places);

    #[cfg(feature = "geocoding")]
    {
        // Test 2: Nominatim (OpenStreetMap)
        println!("\n--- Nominatim (OpenStreetMap) ---");
        println!("Note: Querying public API (please respect rate limits)\n");
        let nominatim = GazetteerNominatim::new();
        
        // Only test first place to avoid hitting rate limits
        if let Some(loc) = nominatim.lookup(test_places[0]) {
            println!("✓ {}: ({:.4}, {:.4})", test_places[0], loc.lat, loc.lon);
        }

        // Test 3: Wikidata
        println!("\n--- Wikidata ---");
        println!("Note: Querying Wikidata SPARQL endpoint\n");
        let wikidata = GazetteerWikidata::new();
        
        if let Some(loc) = wikidata.lookup(test_places[1]) {
            println!("✓ {}: ({:.4}, {:.4})", test_places[1], loc.lat, loc.lon);
        }

        // Test 4: GeoNames (if username provided)
        if let Ok(username) = std::env::var("GEONAMES_USERNAME") {
            println!("\n--- GeoNames ---");
            let geonames = GazetteerGeoNames::new(username);
            
            if let Some(loc) = geonames.lookup(test_places[2]) {
                println!("✓ {}: ({:.4}, {:.4})", test_places[2], loc.lat, loc.lon);
            }
        } else {
            println!("\n--- GeoNames ---");
            println!("Skipped (set GEONAMES_USERNAME environment variable to test)");
        }

        // Test 5: MultiGazetteer with fallback
        println!("\n--- MultiGazetteer (Built-in + Nominatim fallback) ---");
        let mut multi = MultiGazetteer::new();
        multi.add_source(Box::new(BuiltinGazetteer::new()));
        multi.add_source(Box::new(GazetteerNominatim::new()));

        println!("Strategy: Try built-in first, fall back to Nominatim if not found\n");
        
        // Test with a place in built-in (should use built-in)
        if let Some(loc) = multi.lookup("Paris") {
            println!("✓ Paris: ({:.4}, {:.4}) [from built-in]", loc.lat, loc.lon);
        }

        // Test with a place not in built-in (should fall back to Nominatim)
        println!("\nNote: Next lookup will query Nominatim API...");
        if let Some(loc) = multi.lookup("Seattle") {
            println!("✓ Seattle: ({:.4}, {:.4}) [from Nominatim fallback]", loc.lat, loc.lon);
        }
    }

    #[cfg(not(feature = "geocoding"))]
    {
        println!("\n--- API Gazetteers ---");
        println!("Not available (enable 'geocoding' feature to test)");
        println!("Run: cargo run --example gazetteer_comparison --features geocoding");
    }

    println!("\n=== Example Complete ===");
}

fn test_gazetteer(gaz: &dyn Gazetteer, places: &[&str]) {
    for place in places {
        if let Some(loc) = gaz.lookup(place) {
            println!("✓ {}: ({:.4}, {:.4})", place, loc.lat, loc.lon);
        } else {
            println!("✗ {}: Not found", place);
        }
    }
}
