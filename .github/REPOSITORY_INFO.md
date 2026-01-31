# GitHub Repository Setup

Use this information when configuring your GitHub repository.

## Repository Description

```
🗺️ A Rust library for modeling, indexing, and analyzing spatial narratives — events anchored in space and time. Supports GeoJSON, CSV, R-tree indexes, and graph analysis.
```

## Topics (Tags)

Add these topics to your repository for discoverability:

```
rust
geospatial
gis
spatial-data
temporal-data
event-processing
narrative
timeline
r-tree
petgraph
geojson
csv
indexing
graph-analysis
location-based
chronological
data-science
rust-library
spatial-analysis
time-series
```

### Primary Topics (Most Important)
- `rust`
- `geospatial`
- `spatial-data`
- `gis`
- `event-processing`

### Secondary Topics
- `r-tree`
- `graph-analysis`
- `geojson`
- `temporal-data`
- `timeline`

## Social Preview Image

Create a social preview image (1280x640 recommended) with:
- Library name: `spatial-narrative`
- Tagline: "Events in Space and Time"
- Visual: Globe with timeline/markers
- Rust logo

## Repository Settings

### General
- [x] Issues enabled
- [x] Discussions enabled
- [x] Projects enabled
- [x] Wiki disabled (use docs site instead)

### Features
- [x] Preserve this repository (archive if needed)
- [x] Sponsorships enabled (if applicable)

### Branches
- Default branch: `main`
- Branch protection rules for `main`:
  - Require pull request reviews
  - Require status checks to pass
  - Require linear history

### Pages
- Source: `gh-pages` branch or `docs/` folder
- Custom domain: (optional)

## Labels for Issues

```yaml
labels:
  - name: "bug"
    color: "d73a4a"
    description: "Something isn't working"
  
  - name: "enhancement"
    color: "a2eeef"
    description: "New feature or request"
  
  - name: "documentation"
    color: "0075ca"
    description: "Improvements or additions to documentation"
  
  - name: "good first issue"
    color: "7057ff"
    description: "Good for newcomers"
  
  - name: "help wanted"
    color: "008672"
    description: "Extra attention is needed"
  
  - name: "core"
    color: "fbca04"
    description: "Core types and functionality"
  
  - name: "io"
    color: "d4c5f9"
    description: "Import/export formats"
  
  - name: "index"
    color: "bfdadc"
    description: "Spatial/temporal indexing"
  
  - name: "graph"
    color: "c5def5"
    description: "Graph structures and algorithms"
  
  - name: "performance"
    color: "f9d0c4"
    description: "Performance improvements"
  
  - name: "breaking-change"
    color: "b60205"
    description: "Introduces breaking API changes"
```

## Issue Templates

See `.github/ISSUE_TEMPLATE/` for:
- Bug report template
- Feature request template
- Documentation issue template

## Pull Request Template

See `.github/PULL_REQUEST_TEMPLATE.md`

---

## Quick Setup Commands

```bash
# Clone repository
git clone https://github.com/yourusername/spatial-narrative.git
cd spatial-narrative

# Build and test
cargo build
cargo test

# Generate documentation
cargo doc --open

# Run examples
cargo run --example basic_usage
cargo run --example io_formats
cargo run --example indexing
cargo run --example graph_export
```
