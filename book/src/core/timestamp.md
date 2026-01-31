# Timestamp

The `Timestamp` type represents a point in time with timezone awareness and configurable precision.

## Creating Timestamps

### Parse from String

```rust
use spatial_narrative::core::Timestamp;

// Full ISO 8601
let ts = Timestamp::parse("2024-01-15T10:30:00Z")?;

// Date only
let ts = Timestamp::parse("2024-01-15")?;

// Year-month
let ts = Timestamp::parse("2024-01")?;

// Year only
let ts = Timestamp::parse("2024")?;
```

### Current Time

```rust
let now = Timestamp::now();
```

### From Unix Timestamp

```rust
// Seconds since epoch
let ts = Timestamp::from_unix(1705315800)?;

// Milliseconds since epoch
let ts = Timestamp::from_unix_millis(1705315800000)?;
```

## Properties

| Property | Type | Description |
|----------|------|-------------|
| `datetime` | `DateTime<Utc>` | Underlying chrono datetime |
| `precision` | `TemporalPrecision` | Year, Month, Day, Hour, Minute, Second |

## Temporal Precision

Timestamps track their precision level:

```rust
use spatial_narrative::core::TemporalPrecision;

let ts = Timestamp::parse("2024")?;
assert_eq!(ts.precision, TemporalPrecision::Year);

let ts = Timestamp::parse("2024-01-15")?;
assert_eq!(ts.precision, TemporalPrecision::Day);

let ts = Timestamp::parse("2024-01-15T10:30:00Z")?;
assert_eq!(ts.precision, TemporalPrecision::Second);
```

## Methods

### Formatting

```rust
let ts = Timestamp::parse("2024-01-15T10:30:00Z")?;

// RFC 3339 format
ts.to_rfc3339();  // "2024-01-15T10:30:00+00:00"

// Format with precision
ts.format_with_precision();  // Respects the timestamp's precision
```

### Conversion

```rust
// To Unix timestamp
let seconds = ts.to_unix();
let millis = ts.to_unix_millis();
```

### Comparison

```rust
let ts1 = Timestamp::parse("2024-01-01T00:00:00Z")?;
let ts2 = Timestamp::parse("2024-01-02T00:00:00Z")?;

if ts1 < ts2 {
    println!("ts1 is earlier");
}

// Duration between timestamps
let duration = ts2.duration_since(&ts1);
println!("Difference: {} hours", duration.num_hours());
```

## Serialization

```rust
use serde_json;

let ts = Timestamp::parse("2024-01-15T10:30:00Z")?;

// To JSON (serializes as string)
let json = serde_json::to_string(&ts)?;
// "2024-01-15T10:30:00+00:00"

// From JSON
let ts: Timestamp = serde_json::from_str(&json)?;
```

## Examples

### Historical Events

```rust
// Historical dates with appropriate precision
let ww1_start = Timestamp::parse("1914-07-28")?;       // Day precision
let moon_landing = Timestamp::parse("1969-07-20T20:17:40Z")?;  // Second precision
let renaissance = Timestamp::parse("1400")?;            // Year precision
```

### Time Ranges

```rust
use spatial_narrative::core::TimeRange;

let start = Timestamp::parse("2024-01-01T00:00:00Z")?;
let end = Timestamp::parse("2024-12-31T23:59:59Z")?;

let year_2024 = TimeRange::new(start, end);
println!("Duration: {} days", year_2024.duration().num_days());
```

### Sorting Events

```rust
let mut events = vec![event3, event1, event2];

// Sort by timestamp
events.sort_by_key(|e| e.timestamp.clone());

// Now in chronological order
for event in &events {
    println!("{}: {}", event.timestamp.to_rfc3339(), event.text);
}
```
