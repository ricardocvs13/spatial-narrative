# Geoparsing

The `GeoParser` extracts geographic locations from text, detecting both coordinates and place names.

## Basic Usage

```rust
use spatial_narrative::parser::GeoParser;

let parser = GeoParser::new();
let mentions = parser.extract("Meeting at 40.7128, -74.0060");

assert_eq!(mentions.len(), 1);
let loc = mentions[0].location.as_ref().unwrap();
println!("Latitude: {}, Longitude: {}", loc.lat, loc.lon);
```

## Coordinate Formats

The parser detects three coordinate formats:

### Decimal Degrees

```text
40.7128, -74.0060
51.5074 -0.1278
-33.8688, 151.2093
```

### Degrees with Symbols

```text
40.7128°N, 74.0060°W
48.8566N 2.3522E
33.8688°S, 151.2093°E
```

### DMS (Degrees, Minutes, Seconds)

```text
40°42'46"N, 74°0'22"W
51°30'26"N, 0°7'40"W
```

## Place Name Resolution

Use a gazetteer to resolve place names to coordinates:

```rust
use spatial_narrative::parser::{GeoParser, BuiltinGazetteer, MentionType};

let gazetteer = BuiltinGazetteer::new();
let parser = GeoParser::with_gazetteer(Box::new(gazetteer));

let text = "The conference was held in Tokyo and participants came from London.";
let mentions = parser.extract(text);

for mention in &mentions {
    if matches!(mention.mention_type, MentionType::PlaceName) {
        println!("Place: {}", mention.text);
    }
}
```

## Built-in Gazetteer

The `BuiltinGazetteer` includes 2500+ world cities:

- Major world cities selected by population from [GeoNames](https://www.geonames.org)
- Cities across 150+ countries with precise coordinates
- Common aliases (NYC → New York City, SHA → Shanghai, etc.)
- Data licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)

```rust
use spatial_narrative::parser::{BuiltinGazetteer, Gazetteer};

let gazetteer = BuiltinGazetteer::new();

// Direct lookup
if let Some(loc) = gazetteer.lookup("Paris") {
    println!("Paris: {}, {}", loc.lat, loc.lon);
}

// Aliases work too
assert!(gazetteer.contains("NYC"));
assert!(gazetteer.contains("New York City"));
```

## Custom Gazetteer

Implement the `Gazetteer` trait for custom place databases:

```rust
use spatial_narrative::parser::Gazetteer;
use spatial_narrative::core::Location;
use std::collections::HashMap;

struct MyGazetteer {
    places: HashMap<String, (f64, f64)>,
}

impl Gazetteer for MyGazetteer {
    fn lookup(&self, name: &str) -> Option<Location> {
        self.places.get(&name.to_lowercase())
            .map(|(lat, lon)| Location::new(*lat, *lon))
    }

    fn contains(&self, name: &str) -> bool {
        self.places.contains_key(&name.to_lowercase())
    }

    fn all_names(&self) -> Vec<&str> {
        self.places.keys().map(|s| s.as_str()).collect()
    }
}
```

## Configuration

Use `LocationPattern` to control what the parser detects:

```rust
use spatial_narrative::parser::{GeoParser, LocationPattern, BuiltinGazetteer};

// Only detect coordinates, not place names
let mut parser = GeoParser::new();
parser.set_pattern(LocationPattern::coordinates_only());

// Or customize fully
let pattern = LocationPattern {
    detect_decimal: true,
    detect_symbols: true,
    detect_dms: false,
    detect_places: false,
    min_confidence: 0.8,
};
parser.set_pattern(pattern);
```

## LocationMention

Each detected location is returned as a `LocationMention`:

```rust
use spatial_narrative::parser::{GeoParser, MentionType};

let parser = GeoParser::new();
let mentions = parser.extract("Coordinates: 40.7128, -74.0060");

for mention in mentions {
    println!("Text: '{}'", mention.text);
    println!("Position: {}-{}", mention.start, mention.end);
    println!("Type: {:?}", mention.mention_type);
    println!("Confidence: {:.2}", mention.confidence);
    
    if let Some(loc) = mention.location {
        println!("Location: {}, {}", loc.lat, loc.lon);
    }
}
```

### MentionType

- `DecimalDegrees` - "40.7128, -74.0060"
- `DegreesWithSymbols` - "40.7128°N, 74.0060°W"
- `DMS` - "40°42'46\"N, 74°0'22\"W"
- `PlaceName` - "Paris", "New York City"

## Confidence Scores

Each mention has a confidence score (0.0 to 1.0):

| Type | Typical Confidence |
|------|-------------------|
| DMS | 0.99 |
| DegreesWithSymbols | 0.98 |
| DecimalDegrees | 0.95 |
| PlaceName | 0.85 |

Coordinate formats have higher confidence because they're unambiguous. Place names are lower due to potential ambiguity (e.g., "Paris" could be Paris, France or Paris, Texas).
