# Spatial Narrative — Future Development Plan

> **Vision**: A practical Rust library researchers can use to analyze spatiotemporal data and produce results for publications.

---

## Table of Contents

1. [Phase 7: Publication-Ready Analysis](#phase-7-publication-ready-analysis)
2. [Phase 8: Better Geoparsing](#phase-8-better-geoparsing)
3. [Phase 9: Statistical Methods](#phase-9-statistical-methods)
4. [Phase 10: Visualization & Export](#phase-10-visualization--export)
5. [Phase 11: Real-World Data Sources](#phase-11-real-world-data-sources)
6. [Phase 12: Performance at Scale](#phase-12-performance-at-scale)
7. [Phase 13: Language Bindings](#phase-13-language-bindings)

---

## Phase 7: Publication-Ready Analysis

**Goal**: Output analysis results researchers can actually put in papers.

### Reproducible Results

- [ ] **Seed control**: Deterministic clustering and sampling
- [ ] **Method parameters export**: Dump all settings used in analysis
- [ ] **Results serialization**: Save analysis outputs for verification
- [ ] **Versioned output**: Include library version in results

### Statistical Outputs

- [ ] **Summary statistics**: Mean, median, std dev, quartiles for all metrics
- [ ] **Confidence intervals**: 95% CI for distance, duration, density measures
- [ ] **P-values**: Statistical significance for comparisons
- [ ] **Effect sizes**: Cohen's d, etc. for narrative comparisons
- [ ] **Tables**: Generate LaTeX/Markdown tables directly

### Result Formatting

- [ ] **APA/Chicago formatting**: Pre-formatted result strings
- [ ] **Significant figures**: Control decimal precision
- [ ] **Unit conversion**: km/miles, hours/days automatic
- [ ] **Copy-paste ready**: Results formatted for papers

---

## Phase 8: Better Geoparsing

**Goal**: Actually extract locations accurately from real text.

### Disambiguation

- [ ] **Context awareness**: "Paris" near "France" → Paris, France
- [ ] **Population ranking**: Default to larger cities
- [ ] **Document coherence**: Prefer locations consistent with document
- [ ] **User hints**: Allow specifying expected regions

### Coordinate Accuracy

- [ ] **Precision tracking**: Know if coordinates are city-level or street-level
- [ ] **Source attribution**: Track where each coordinate came from
- [ ] **Validation against coastlines**: Detect land/water mismatches
- [ ] **Altitude validation**: Sanity check elevation data

### More Formats

- [ ] **UTM coordinates**: Common in field research
- [ ] **MGRS**: Military grid reference
- [ ] **What3Words**: Decode W3W addresses
- [ ] **Plus Codes**: Google's open location codes

### Bigger Gazetteer

- [ ] **GeoNames integration**: 11M+ place names
- [ ] **OpenStreetMap**: POI-level detail
- [ ] **Historical names**: Old place names that changed
- [ ] **Local names**: Non-English place names

---

## Phase 9: Statistical Methods

**Goal**: Implement methods researchers actually use in papers.

### Spatial Statistics

- [ ] **Nearest neighbor analysis**: Clark-Evans R statistic
- [ ] **Ripley's K-function**: Point pattern analysis
- [ ] **Kernel density estimation**: Continuous density surfaces
- [ ] **Spatial autocorrelation**: Moran's I, Local Moran's
- [ ] **Getis-Ord Gi***: Hot spot detection

### Temporal Statistics

- [ ] **Event rate analysis**: Events per time unit with CI
- [ ] **Periodicity detection**: Find recurring patterns
- [ ] **Change point detection**: When did patterns shift?
- [ ] **Survival analysis**: Time between events
- [ ] **ARIMA modeling**: Time series forecasting

### Movement Analysis

- [ ] **Step length distribution**: Movement distance patterns
- [ ] **Turning angle analysis**: Direction change patterns
- [ ] **Home range estimation**: MCP, kernel methods
- [ ] **Path tortuosity**: Straightness index
- [ ] **First passage time**: Search behavior metrics

### Comparison Methods

- [ ] **Permutation tests**: Non-parametric significance
- [ ] **Bootstrap confidence intervals**: Robust CI estimation
- [ ] **Cross-correlation**: Temporal relationship detection
- [ ] **DTW distance**: Compare event sequences

---

## Phase 10: Visualization & Export

**Goal**: Generate figures for papers.

### Map Outputs

- [ ] **SVG export**: Vector graphics for journals
- [ ] **PDF export**: Print-ready maps
- [ ] **Configurable styles**: Journal-appropriate colors
- [ ] **Scale bars**: Proper cartographic elements
- [ ] **North arrows**: Standard map elements
- [ ] **Legends**: Auto-generated legends

### Chart Outputs

- [ ] **Timeline plots**: Events over time
- [ ] **Density plots**: KDE visualizations
- [ ] **Cluster maps**: Colored by cluster
- [ ] **Trajectory plots**: Movement paths
- [ ] **Heatmaps**: Spatial intensity

### Figure Requirements

- [ ] **300+ DPI**: Publication resolution
- [ ] **CMYK support**: Print color mode
- [ ] **Font embedding**: Portable figures
- [ ] **Accessibility**: Colorblind-safe palettes
- [ ] **Size presets**: Common journal dimensions

### Data Export

- [ ] **CSV with headers**: Analysis results as tables
- [ ] **Excel-compatible**: .xlsx for collaborators
- [ ] **R-ready**: .rds or feather format
- [ ] **SPSS-compatible**: For social science folks

---

## Phase 11: Real-World Data Sources

**Goal**: Work with data researchers actually have.

### Common Formats

- [ ] **GPX tracks**: GPS device exports
- [ ] **KML/KMZ**: Google Earth files
- [ ] **Shapefiles**: GIS standard format
- [ ] **GeoPackage**: Modern GIS format
- [ ] **Excel with coordinates**: The reality of research data

### Data Cleaning

- [ ] **Duplicate detection**: Find repeated events
- [ ] **Outlier flagging**: Impossible coordinates/times
- [ ] **Gap filling**: Interpolate missing data
- [ ] **Timezone normalization**: Handle mixed timezones
- [ ] **Coordinate system conversion**: Project between CRS

### External APIs

- [ ] **Nominatim**: Free OpenStreetMap geocoding
- [ ] **Overpass**: OSM data queries
- [ ] **GeoNames**: Place name database
- [ ] **Natural Earth**: Country/region boundaries

---

## Phase 12: Performance at Scale

**Goal**: Handle real research datasets.

### Memory Efficiency

- [ ] **Streaming import**: Don't load everything into RAM
- [ ] **Lazy loading**: Load data on demand
- [ ] **Memory limits**: Configurable memory caps
- [ ] **Disk spillover**: Use temp files for large datasets

### Speed

- [ ] **Parallel processing**: Use all CPU cores
- [ ] **SIMD distance calc**: Vectorized haversine
- [ ] **Index optimization**: Faster spatial queries
- [ ] **Batch operations**: Bulk insert/query

### Scale Targets

- [ ] **1M events**: Handle in <1GB RAM
- [ ] **10M events**: With streaming/chunking
- [ ] **Complex queries**: <1s for typical analysis

---

## Phase 13: Language Bindings

**Goal**: Let researchers use their preferred tools.

### Python (Priority)

- [ ] **PyO3 bindings**: Native Python package
- [ ] **NumPy integration**: Array interop
- [ ] **Pandas integration**: DataFrame import/export
- [ ] **GeoPandas**: GeoDataFrame support
- [ ] **Jupyter widgets**: Interactive exploration

### R

- [ ] **extendr bindings**: Native R package
- [ ] **sf compatibility**: R spatial data frames
- [ ] **tidyverse style**: Pipe-friendly API

### CLI Tools

- [ ] **sn-parse**: Extract locations from text files
- [ ] **sn-analyze**: Run analysis from command line.
- [ ] **sn-convert**: Format conversion utility
- [ ] **sn-stats**: Generate statistics

---

## Priority Order

| Priority | Feature | Why |
|----------|---------|-----|
| **P0** | Statistical outputs with CI/p-values | Researchers need this for papers |
| **P0** | SVG/PDF map export | Figures for publications |
| **P0** | Better disambiguation | Current geoparsing too naive |
| **P1** | Python bindings | Most researchers use Python |
| **P1** | GPX/KML import | Common research data formats |
| **P1** | Ripley's K, Moran's I | Standard spatial statistics |
| **P2** | GeoNames integration | Much better place coverage |
| **P2** | Streaming for large data | Real datasets are big |
| **P3** | R bindings | Secondary audience |
| **P3** | CLI tools | Convenience |

---

## Version Roadmap

| Version | Focus | Target |
|---------|-------|--------|
| **0.2** | Statistical outputs, confidence intervals, p-values | Q2 2026 |
| **0.3** | SVG/PDF export, publication-ready figures | Q3 2026 |
| **0.4** | Spatial statistics (Ripley's K, Moran's I, KDE) | Q4 2026 |
| **0.5** | Python bindings with GeoPandas support | Q1 2027 |
| **1.0** | Stable, documented, production-ready | Q2 2027 |

---

*Last updated: January 2026*
