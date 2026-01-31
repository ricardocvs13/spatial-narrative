# Location

The `Location` type represents a geographic point on Earth.

## Creating Locations

### Simple Constructor

```rust
use spatial_narrative::core::Location;

// Latitude, Longitude (WGS84)
let nyc = Location::new(40.7128, -74.0060);
let tokyo = Location::new(35.6762, 139.6503);
let sydney = Location::new(-33.8688, 151.2093);
```

### Builder Pattern

For locations with additional attributes:

```rust
let location = Location::builder()
    .lat(40.7484)
    .lon(-73.9857)
    .elevation(443.0)           // meters above sea level
    .uncertainty_meters(10.0)   // GPS accuracy
    .name("Empire State Building")
    .build()
    .unwrap();
```

### From Tuple

```rust
let loc: Location = (40.7128, -74.0060).into();
```

## Properties

| Property | Type | Description |
|----------|------|-------------|
| `lat` | `f64` | Latitude (-90 to 90) |
| `lon` | `f64` | Longitude (-180 to 180) |
| `elevation` | `Option<f64>` | Meters above sea level |
| `uncertainty_meters` | `Option<f64>` | Location accuracy |
| `name` | `Option<String>` | Human-readable name |

## Methods

### `distance_to`

Calculate great-circle distance to another location:

```rust
let nyc = Location::new(40.7128, -74.0060);
let london = Location::new(51.5074, -0.1278);

let distance_km = nyc.distance_to(&london);
println!("NYC to London: {:.0} km", distance_km);  // ~5570 km
```

### `to_geo_point`

Convert to `geo` crate's Point type:

```rust
use geo::Point;

let location = Location::new(40.7128, -74.0060);
let point: Point<f64> = location.to_geo_point();
```

## Validation

Coordinates are validated on construction:

```rust
// Valid
Location::new(0.0, 0.0);       // Null Island
Location::new(90.0, 180.0);    // North Pole, Date Line
Location::new(-90.0, -180.0);  // South Pole, Date Line

// Invalid - will return Error
Location::builder().lat(91.0).lon(0.0).build();   // Lat out of range
Location::builder().lat(0.0).lon(181.0).build();  // Lon out of range
```

## Serialization

```rust
use serde_json;

let loc = Location::new(40.7128, -74.0060);

// To JSON
let json = serde_json::to_string(&loc)?;
// {"lat":40.7128,"lon":-74.006,"elevation":null,"uncertainty_meters":null,"name":null}

// From JSON
let loc: Location = serde_json::from_str(&json)?;
```

## Examples

### City Locations

```rust
let cities = vec![
    Location::builder().lat(40.7128).lon(-74.0060).name("New York").build()?,
    Location::builder().lat(51.5074).lon(-0.1278).name("London").build()?,
    Location::builder().lat(35.6762).lon(139.6503).name("Tokyo").build()?,
    Location::builder().lat(-33.8688).lon(151.2093).name("Sydney").build()?,
];
```

### With Elevation

```rust
// Mountain peaks
let everest = Location::builder()
    .lat(27.9881)
    .lon(86.9250)
    .elevation(8848.86)
    .name("Mount Everest")
    .build()?;

let denali = Location::builder()
    .lat(63.0692)
    .lon(-151.0070)
    .elevation(6190.5)
    .name("Denali")
    .build()?;
```

### GPS Tracking

```rust
// GPS readings with uncertainty
let readings = vec![
    Location::builder()
        .lat(40.7128).lon(-74.0060)
        .uncertainty_meters(5.0)
        .build()?,
    Location::builder()
        .lat(40.7130).lon(-74.0058)
        .uncertainty_meters(3.0)
        .build()?,
];
```
