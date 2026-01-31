# Sources

The `SourceRef` type provides source attribution for events.

## Creating Source References

```rust
use spatial_narrative::core::{SourceRef, SourceType};

let source = SourceRef::builder()
    .title("The New York Times")
    .source_type(SourceType::Article)
    .url("https://nytimes.com/article/123")
    .author("Jane Reporter")
    .date("2024-01-15")
    .build();
```

## Source Types

| Type | Description |
|------|-------------|
| `Article` | News article or blog post |
| `Report` | Official report or document |
| `Witness` | Eyewitness account |
| `Sensor` | Automated sensor data |
| `Archive` | Historical archive |
| `Other` | Other source type |

## Properties

| Property | Type | Description |
|----------|------|-------------|
| `source_type` | `SourceType` | Category of source |
| `title` | `Option<String>` | Source title |
| `url` | `Option<String>` | URL reference |
| `author` | `Option<String>` | Author/creator |
| `date` | `Option<String>` | Publication date |
| `notes` | `Option<String>` | Additional notes |

## Examples

### News Article

```rust
let source = SourceRef::builder()
    .source_type(SourceType::Article)
    .title("Breaking: Event Occurs")
    .url("https://news.example.com/article")
    .author("John Journalist")
    .date("2024-01-15")
    .build();
```

### Sensor Data

```rust
let source = SourceRef::builder()
    .source_type(SourceType::Sensor)
    .title("Weather Station #42")
    .notes("Automated reading every 5 minutes")
    .build();
```

### Historical Archive

```rust
let source = SourceRef::builder()
    .source_type(SourceType::Archive)
    .title("National Archives Collection")
    .url("https://archives.gov/document/123")
    .notes("Declassified 2020")
    .build();
```
