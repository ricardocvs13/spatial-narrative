# Installation

## Requirements

- **Rust**: 1.70 or later
- **Cargo**: Included with Rust

## Adding to Your Project

Add `spatial-narrative` to your `Cargo.toml`:

```toml
[dependencies]
spatial-narrative = "1.0"
```

Or use cargo add:

```bash
cargo add spatial-narrative
```

## Features

The library comes with sensible defaults. All core features are included by default.

### Default Features

```toml
[dependencies]
spatial-narrative = "1.0"  # Includes all standard features
```

### Optional Features

```toml
[dependencies]
spatial-narrative = { version = "1.0", features = ["parallel"] }
```

| Feature | Description | Default |
|---------|-------------|---------|
| `parallel` | Enable parallel processing with rayon | ❌ |
| `serde` | Serialization support | ✅ |

## Verifying Installation

Create a simple test file:

```rust
// src/main.rs
use spatial_narrative::core::{Location, Timestamp, Event};

fn main() {
    let location = Location::new(40.7128, -74.0060);
    let timestamp = Timestamp::now();
    let event = Event::new(location, timestamp, "Hello, spatial-narrative!");
    
    println!("Created event: {}", event.text);
    println!("Location: ({}, {})", event.location.lat, event.location.lon);
}
```

Run it:

```bash
cargo run
```

You should see:

```
Created event: Hello, spatial-narrative!
Location: (40.7128, -74.006)
```

## Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/spatial-narrative.git
cd spatial-narrative

# Build
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

## Minimum Supported Rust Version (MSRV)

The current MSRV is **Rust 1.70**.

This version is tested in CI and will be maintained according to our [compatibility policy](../reference/faq.md#compatibility).

## Next Steps

- [Quick Start](./quick-start.md) - Get up and running quickly
- [Concepts](./concepts.md) - Understand the core ideas
