//! Gazetteer trait and built-in implementation for place name resolution.
//!
//! This module provides multiple gazetteer implementations for resolving place names to coordinates:
//!
//! - [`BuiltinGazetteer`]: Built-in database with 200+ major world locations
//! - [`GazetteerNominatim`]: OpenStreetMap Nominatim API (requires `geocoding` feature)
//! - [`GazetteerGeoNames`]: GeoNames web service (requires `geocoding` feature)
//! - [`GazetteerWikidata`]: Wikidata SPARQL query service (requires `geocoding` feature)
//! - [`MultiGazetteer`]: Combines multiple gazetteers with fallback
//!
//! # Example
//!
//! ```rust
//! use spatial_narrative::parser::{BuiltinGazetteer, Gazetteer};
//!
//! let gaz = BuiltinGazetteer::new();
//! if let Some(loc) = gaz.lookup("Paris") {
//!     println!("Paris: {}, {}", loc.lat, loc.lon);
//! }
//! ```

use crate::core::Location;
use std::collections::HashMap;

#[cfg(feature = "geocoding")]
use serde::Deserialize;

/// Trait for place name resolution (gazetteer).
///
/// Implement this trait to provide custom place name databases
/// or external geocoding services.
///
/// # Example
///
/// ```rust
/// use spatial_narrative::parser::Gazetteer;
/// use spatial_narrative::core::Location;
///
/// struct MyGazetteer {
///     places: std::collections::HashMap<String, (f64, f64)>,
/// }
///
/// impl Gazetteer for MyGazetteer {
///     fn lookup(&self, name: &str) -> Option<Location> {
///         self.places.get(&name.to_lowercase())
///             .map(|(lat, lon)| Location::new(*lat, *lon))
///     }
///
///     fn contains(&self, name: &str) -> bool {
///         self.places.contains_key(&name.to_lowercase())
///     }
///
///     fn all_names(&self) -> Vec<&str> {
///         self.places.keys().map(|s| s.as_str()).collect()
///     }
/// }
/// ```
pub trait Gazetteer: Send + Sync {
    /// Look up a place name and return its coordinates.
    fn lookup(&self, name: &str) -> Option<Location>;

    /// Check if a place name exists in the gazetteer.
    fn contains(&self, name: &str) -> bool;

    /// Get all place names in the gazetteer.
    fn all_names(&self) -> Vec<&str>;

    /// Get aliases for a place name (e.g., "NYC" → "New York City").
    fn aliases(&self, _name: &str) -> Vec<&str> {
        Vec::new()
    }
}

#[cfg(feature = "geocoding")]
/// Gazetteer that queries OSM Nominatim API.
///
/// Uses the public Nominatim API at `https://nominatim.openstreetmap.org`.
/// Please respect the [usage policy](https://operations.osmfoundation.org/policies/nominatim/).
///
/// # Example
///
/// ```rust,no_run
/// use spatial_narrative::parser::{GazetteerNominatim, Gazetteer};
///
/// let gaz = GazetteerNominatim::new();
/// if let Some(loc) = gaz.lookup("Berlin") {
///     println!("Berlin: {}, {}", loc.lat, loc.lon);
/// }
/// ```
pub struct GazetteerNominatim {
    base_url: String,
    user_agent: String,
}

#[cfg(feature = "geocoding")]
impl GazetteerNominatim {
    /// Create a new Nominatim gazetteer with default settings.
    pub fn new() -> Self {
        Self {
            base_url: "https://nominatim.openstreetmap.org".to_string(),
            user_agent: "spatial-narrative/0.1.0".to_string(),
        }
    }

    /// Create a Nominatim gazetteer with custom base URL (for self-hosted instances).
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            user_agent: "spatial-narrative/0.1.0".to_string(),
        }
    }

    /// Set a custom user agent (recommended for production use).
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }
}

#[cfg(feature = "geocoding")]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NominatimResponse {
    lat: String,
    lon: String,
    display_name: Option<String>,
}

#[cfg(feature = "geocoding")]
impl Gazetteer for GazetteerNominatim {
    fn lookup(&self, name: &str) -> Option<Location> {
        let url = format!(
            "{}/search?q={}&format=json&limit=1",
            self.base_url,
            urlencoding::encode(name)
        );

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", &self.user_agent)
            .send()
            .ok()?;

        let results: Vec<NominatimResponse> = response.json().ok()?;
        let result = results.first()?;

        let lat: f64 = result.lat.parse().ok()?;
        let lon: f64 = result.lon.parse().ok()?;

        Some(Location::new(lat, lon))
    }

    fn contains(&self, name: &str) -> bool {
        self.lookup(name).is_some()
    }

    fn all_names(&self) -> Vec<&str> {
        vec![] // Not applicable for API-based gazetteers
    }
}

#[cfg(not(feature = "geocoding"))]
/// Gazetteer that queries OSM Nominatim API (requires `geocoding` feature).
pub struct GazetteerNominatim;

#[cfg(not(feature = "geocoding"))]
impl Gazetteer for GazetteerNominatim {
    fn lookup(&self, _name: &str) -> Option<Location> {
        None
    }
    fn contains(&self, _name: &str) -> bool {
        false
    }
    fn all_names(&self) -> Vec<&str> {
        vec![]
    }
}

#[cfg(feature = "geocoding")]
/// Gazetteer that queries GeoNames API.
///
/// Requires a GeoNames username (free registration at https://www.geonames.org/login).
///
/// # Example
///
/// ```rust,no_run
/// use spatial_narrative::parser::{GazetteerGeoNames, Gazetteer};
///
/// let gaz = GazetteerGeoNames::new("your_username");
/// if let Some(loc) = gaz.lookup("Tokyo") {
///     println!("Tokyo: {}, {}", loc.lat, loc.lon);
/// }
/// ```
pub struct GazetteerGeoNames {
    username: String,
}

#[cfg(feature = "geocoding")]
impl GazetteerGeoNames {
    /// Create a new GeoNames gazetteer with your username.
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
        }
    }
}

#[cfg(feature = "geocoding")]
#[derive(Debug, Deserialize)]
struct GeoNamesResponse {
    geonames: Vec<GeoNamesEntry>,
}

#[cfg(feature = "geocoding")]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GeoNamesEntry {
    lat: String,
    lng: String,
    name: String,
}

#[cfg(feature = "geocoding")]
impl Gazetteer for GazetteerGeoNames {
    fn lookup(&self, name: &str) -> Option<Location> {
        let url = format!(
            "http://api.geonames.org/searchJSON?q={}&maxRows=1&username={}",
            urlencoding::encode(name),
            urlencoding::encode(&self.username)
        );

        let client = reqwest::blocking::Client::new();
        let response = client.get(&url).send().ok()?;
        let data: GeoNamesResponse = response.json().ok()?;
        let entry = data.geonames.first()?;

        let lat: f64 = entry.lat.parse().ok()?;
        let lon: f64 = entry.lng.parse().ok()?;

        Some(Location::new(lat, lon))
    }

    fn contains(&self, name: &str) -> bool {
        self.lookup(name).is_some()
    }

    fn all_names(&self) -> Vec<&str> {
        vec![]
    }
}

#[cfg(not(feature = "geocoding"))]
/// Gazetteer that queries GeoNames API (requires `geocoding` feature).
pub struct GazetteerGeoNames;

#[cfg(not(feature = "geocoding"))]
impl Gazetteer for GazetteerGeoNames {
    fn lookup(&self, _name: &str) -> Option<Location> {
        None
    }
    fn contains(&self, _name: &str) -> bool {
        false
    }
    fn all_names(&self) -> Vec<&str> {
        vec![]
    }
}

#[cfg(feature = "geocoding")]
/// Gazetteer that queries Wikidata SPARQL endpoint.
///
/// Uses the Wikidata Query Service to find geographic coordinates for place names.
///
/// # Example
///
/// ```rust,no_run
/// use spatial_narrative::parser::{GazetteerWikidata, Gazetteer};
///
/// let gaz = GazetteerWikidata::new();
/// if let Some(loc) = gaz.lookup("London") {
///     println!("London: {}, {}", loc.lat, loc.lon);
/// }
/// ```
pub struct GazetteerWikidata {
    endpoint: String,
}

#[cfg(feature = "geocoding")]
impl GazetteerWikidata {
    /// Create a new Wikidata gazetteer.
    pub fn new() -> Self {
        Self {
            endpoint: "https://query.wikidata.org/sparql".to_string(),
        }
    }

    fn build_query(name: &str) -> String {
        format!(
            r#"
SELECT ?place ?placeLabel ?coord WHERE {{
  ?place rdfs:label "{}"@en.
  ?place wdt:P625 ?coord.
  SERVICE wikibase:label {{ bd:serviceParam wikibase:language "en". }}
}}
LIMIT 1
"#,
            name.replace('"', r#"\""#)
        )
    }
}

#[cfg(feature = "geocoding")]
#[derive(Debug, Deserialize)]
struct WikidataResponse {
    results: WikidataResults,
}

#[cfg(feature = "geocoding")]
#[derive(Debug, Deserialize)]
struct WikidataResults {
    bindings: Vec<WikidataBinding>,
}

#[cfg(feature = "geocoding")]
#[derive(Debug, Deserialize)]
struct WikidataBinding {
    coord: WikidataValue,
}

#[cfg(feature = "geocoding")]
#[derive(Debug, Deserialize)]
struct WikidataValue {
    value: String,
}

#[cfg(feature = "geocoding")]
impl Gazetteer for GazetteerWikidata {
    fn lookup(&self, name: &str) -> Option<Location> {
        let query = Self::build_query(name);
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(&self.endpoint)
            .query(&[("query", query)])
            .header("User-Agent", "spatial-narrative/0.1.0")
            .header("Accept", "application/sparql-results+json")
            .send()
            .ok()?;

        let data: WikidataResponse = response.json().ok()?;
        let binding = data.results.bindings.first()?;

        // Parse "Point(lon lat)" format
        let coord_str = &binding.coord.value;
        if let Some(point_data) = coord_str.strip_prefix("Point(").and_then(|s| s.strip_suffix(")"))
        {
            let parts: Vec<&str> = point_data.split_whitespace().collect();
            if parts.len() == 2 {
                let lon: f64 = parts[0].parse().ok()?;
                let lat: f64 = parts[1].parse().ok()?;
                return Some(Location::new(lat, lon));
            }
        }

        None
    }

    fn contains(&self, name: &str) -> bool {
        self.lookup(name).is_some()
    }

    fn all_names(&self) -> Vec<&str> {
        vec![]
    }
}

#[cfg(not(feature = "geocoding"))]
/// Gazetteer that queries Wikidata (requires `geocoding` feature).
pub struct GazetteerWikidata;

#[cfg(not(feature = "geocoding"))]
impl Gazetteer for GazetteerWikidata {
    fn lookup(&self, _name: &str) -> Option<Location> {
        None
    }
    fn contains(&self, _name: &str) -> bool {
        false
    }
    fn all_names(&self) -> Vec<&str> {
        vec![]
    }
}

/// Gazetteer that tries multiple sources in order.
///
/// Queries each gazetteer in sequence until one returns a result.
/// Useful for combining a fast local gazetteer with slower API-based fallbacks.
///
/// # Example
///
/// ```rust,no_run
/// use spatial_narrative::parser::{BuiltinGazetteer, MultiGazetteer, Gazetteer};
/// # #[cfg(feature = "geocoding")]
/// use spatial_narrative::parser::GazetteerNominatim;
///
/// let mut multi = MultiGazetteer::new();
/// multi.add_source(Box::new(BuiltinGazetteer::new())); // Try built-in first
/// # #[cfg(feature = "geocoding")]
/// multi.add_source(Box::new(GazetteerNominatim::new())); // Then Nominatim
///
/// if let Some(loc) = multi.lookup("Paris") {
///     println!("Found Paris: {}, {}", loc.lat, loc.lon);
/// }
/// ```
pub struct MultiGazetteer {
    sources: Vec<Box<dyn Gazetteer>>,
}

impl MultiGazetteer {
    /// Create a new empty multi-gazetteer.
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    /// Add a gazetteer source (will be queried in order).
    pub fn add_source(&mut self, source: Box<dyn Gazetteer>) {
        self.sources.push(source);
    }

    /// Create a multi-gazetteer with the given sources.
    pub fn from_sources(sources: Vec<Box<dyn Gazetteer>>) -> Self {
        Self { sources }
    }
}

impl Default for MultiGazetteer {
    fn default() -> Self {
        Self::new()
    }
}

impl Gazetteer for MultiGazetteer {
    fn lookup(&self, name: &str) -> Option<Location> {
        for g in &self.sources {
            if let Some(loc) = g.lookup(name) {
                return Some(loc);
            }
        }
        None
    }
    fn contains(&self, name: &str) -> bool {
        self.sources.iter().any(|g| g.contains(name))
    }
    fn all_names(&self) -> Vec<&str> {
        let mut names = Vec::new();
        for g in &self.sources {
            names.extend(g.all_names());
        }
        names
    }
}

/// Entry in the built-in gazetteer.
#[derive(Debug, Clone)]
pub struct GazetteerEntry {
    /// Primary name
    pub name: String,
    /// Country or region
    pub country: String,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Population (for ranking)
    pub population: u64,
    /// Alternative names
    pub aliases: Vec<String>,
}

/// Built-in gazetteer with major world locations.
///
/// Contains approximately 200+ major cities, countries, and landmarks.
/// For comprehensive coverage, consider using an external geocoding service.
pub struct BuiltinGazetteer {
    entries: HashMap<String, GazetteerEntry>,
    name_to_canonical: HashMap<String, String>,
}

impl BuiltinGazetteer {
    /// Create a new built-in gazetteer.
    pub fn new() -> Self {
        let mut gazetteer = Self {
            entries: HashMap::new(),
            name_to_canonical: HashMap::new(),
        };
        gazetteer.load_default_entries();
        gazetteer
    }

    /// Add a custom entry to the gazetteer.
    pub fn add_entry(&mut self, entry: GazetteerEntry) {
        let canonical = entry.name.to_lowercase();

        // Add aliases
        for alias in &entry.aliases {
            self.name_to_canonical
                .insert(alias.to_lowercase(), canonical.clone());
        }

        self.name_to_canonical
            .insert(canonical.clone(), canonical.clone());
        self.entries.insert(canonical, entry);
    }

    /// Get the number of entries in the gazetteer.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the gazetteer is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn load_default_entries(&mut self) {
        // Major world capitals and cities
        let cities = [
            // North America
            (
                "New York City",
                "United States",
                40.7128,
                -74.0060,
                8_336_817,
                vec!["NYC", "New York", "NY"],
            ),
            (
                "Los Angeles",
                "United States",
                34.0522,
                -118.2437,
                3_979_576,
                vec!["LA", "L.A."],
            ),
            (
                "Chicago",
                "United States",
                41.8781,
                -87.6298,
                2_693_976,
                vec![],
            ),
            (
                "Houston",
                "United States",
                29.7604,
                -95.3698,
                2_320_268,
                vec![],
            ),
            (
                "Phoenix",
                "United States",
                33.4484,
                -112.0740,
                1_680_992,
                vec![],
            ),
            (
                "Philadelphia",
                "United States",
                39.9526,
                -75.1652,
                1_584_064,
                vec!["Philly"],
            ),
            (
                "San Antonio",
                "United States",
                29.4241,
                -98.4936,
                1_547_253,
                vec![],
            ),
            (
                "San Diego",
                "United States",
                32.7157,
                -117.1611,
                1_423_851,
                vec![],
            ),
            (
                "Dallas",
                "United States",
                32.7767,
                -96.7970,
                1_343_573,
                vec![],
            ),
            (
                "San Francisco",
                "United States",
                37.7749,
                -122.4194,
                883_305,
                vec!["SF"],
            ),
            (
                "Seattle",
                "United States",
                47.6062,
                -122.3321,
                753_675,
                vec![],
            ),
            (
                "Boston",
                "United States",
                42.3601,
                -71.0589,
                692_600,
                vec![],
            ),
            (
                "Washington",
                "United States",
                38.9072,
                -77.0369,
                689_545,
                vec!["Washington D.C.", "D.C.", "DC"],
            ),
            ("Miami", "United States", 25.7617, -80.1918, 467_963, vec![]),
            (
                "Denver",
                "United States",
                39.7392,
                -104.9903,
                727_211,
                vec![],
            ),
            (
                "Atlanta",
                "United States",
                33.7490,
                -84.3880,
                498_715,
                vec!["ATL"],
            ),
            ("Toronto", "Canada", 43.6532, -79.3832, 2_930_000, vec![]),
            ("Vancouver", "Canada", 49.2827, -123.1207, 675_218, vec![]),
            ("Montreal", "Canada", 45.5017, -73.5673, 1_780_000, vec![]),
            (
                "Mexico City",
                "Mexico",
                19.4326,
                -99.1332,
                8_918_653,
                vec!["CDMX", "Ciudad de México"],
            ),
            // Europe
            (
                "London",
                "United Kingdom",
                51.5074,
                -0.1278,
                8_982_000,
                vec![],
            ),
            ("Paris", "France", 48.8566, 2.3522, 2_161_000, vec![]),
            ("Berlin", "Germany", 52.5200, 13.4050, 3_769_495, vec![]),
            ("Madrid", "Spain", 40.4168, -3.7038, 3_223_334, vec![]),
            ("Rome", "Italy", 41.9028, 12.4964, 2_872_800, vec!["Roma"]),
            ("Amsterdam", "Netherlands", 52.3676, 4.9041, 872_680, vec![]),
            (
                "Vienna",
                "Austria",
                48.2082,
                16.3738,
                1_911_191,
                vec!["Wien"],
            ),
            (
                "Brussels",
                "Belgium",
                50.8503,
                4.3517,
                1_208_542,
                vec!["Bruxelles", "Brussel"],
            ),
            ("Stockholm", "Sweden", 59.3293, 18.0686, 975_904, vec![]),
            ("Oslo", "Norway", 59.9139, 10.7522, 693_494, vec![]),
            (
                "Copenhagen",
                "Denmark",
                55.6761,
                12.5683,
                794_128,
                vec!["København"],
            ),
            ("Helsinki", "Finland", 60.1699, 24.9384, 656_229, vec![]),
            ("Dublin", "Ireland", 53.3498, -6.2603, 554_554, vec![]),
            (
                "Lisbon",
                "Portugal",
                38.7223,
                -9.1393,
                504_718,
                vec!["Lisboa"],
            ),
            ("Athens", "Greece", 37.9838, 23.7275, 664_046, vec!["Αθήνα"]),
            (
                "Warsaw",
                "Poland",
                52.2297,
                21.0122,
                1_790_658,
                vec!["Warszawa"],
            ),
            (
                "Prague",
                "Czech Republic",
                50.0755,
                14.4378,
                1_309_000,
                vec!["Praha"],
            ),
            ("Budapest", "Hungary", 47.4979, 19.0402, 1_756_000, vec![]),
            (
                "Bucharest",
                "Romania",
                44.4268,
                26.1025,
                1_883_425,
                vec!["București"],
            ),
            ("Kyiv", "Ukraine", 50.4501, 30.5234, 2_952_301, vec!["Kiev"]),
            (
                "Moscow",
                "Russia",
                55.7558,
                37.6173,
                12_615_279,
                vec!["Москва"],
            ),
            (
                "Saint Petersburg",
                "Russia",
                59.9311,
                30.3609,
                5_383_890,
                vec!["St. Petersburg", "St Petersburg"],
            ),
            (
                "Istanbul",
                "Turkey",
                41.0082,
                28.9784,
                15_462_452,
                vec!["Constantinople"],
            ),
            (
                "Zurich",
                "Switzerland",
                47.3769,
                8.5417,
                415_367,
                vec!["Zürich"],
            ),
            (
                "Geneva",
                "Switzerland",
                46.2044,
                6.1432,
                201_818,
                vec!["Genève"],
            ),
            (
                "Munich",
                "Germany",
                48.1351,
                11.5820,
                1_471_508,
                vec!["München"],
            ),
            ("Milan", "Italy", 45.4642, 9.1900, 1_378_689, vec!["Milano"]),
            ("Barcelona", "Spain", 41.3851, 2.1734, 1_620_343, vec![]),
            // Asia
            (
                "Tokyo",
                "Japan",
                35.6762,
                139.6503,
                13_960_000,
                vec!["東京"],
            ),
            (
                "Beijing",
                "China",
                39.9042,
                116.4074,
                21_540_000,
                vec!["Peking", "北京"],
            ),
            (
                "Shanghai",
                "China",
                31.2304,
                121.4737,
                26_320_000,
                vec!["上海"],
            ),
            (
                "Hong Kong",
                "China",
                22.3193,
                114.1694,
                7_496_981,
                vec!["HK"],
            ),
            (
                "Singapore",
                "Singapore",
                1.3521,
                103.8198,
                5_850_342,
                vec![],
            ),
            (
                "Seoul",
                "South Korea",
                37.5665,
                126.9780,
                9_733_509,
                vec!["서울"],
            ),
            (
                "Mumbai",
                "India",
                19.0760,
                72.8777,
                20_411_274,
                vec!["Bombay"],
            ),
            (
                "Delhi",
                "India",
                28.6139,
                77.2090,
                16_787_941,
                vec!["New Delhi"],
            ),
            (
                "Bangkok",
                "Thailand",
                13.7563,
                100.5018,
                8_281_099,
                vec!["กรุงเทพ"],
            ),
            (
                "Taipei",
                "Taiwan",
                25.0330,
                121.5654,
                2_646_204,
                vec!["臺北"],
            ),
            ("Jakarta", "Indonesia", 6.2088, 106.8456, 10_562_088, vec![]),
            (
                "Manila",
                "Philippines",
                14.5995,
                120.9842,
                1_846_513,
                vec![],
            ),
            (
                "Kuala Lumpur",
                "Malaysia",
                3.1390,
                101.6869,
                1_808_000,
                vec!["KL"],
            ),
            (
                "Dubai",
                "United Arab Emirates",
                25.2048,
                55.2708,
                3_400_800,
                vec![],
            ),
            (
                "Tel Aviv",
                "Israel",
                32.0853,
                34.7818,
                460_613,
                vec!["תל אביב"],
            ),
            (
                "Jerusalem",
                "Israel",
                31.7683,
                35.2137,
                936_425,
                vec!["ירושלים"],
            ),
            (
                "Riyadh",
                "Saudi Arabia",
                24.7136,
                46.6753,
                7_676_654,
                vec![],
            ),
            ("Tehran", "Iran", 35.6892, 51.3890, 8_693_706, vec!["تهران"]),
            ("Doha", "Qatar", 25.2854, 51.5310, 2_382_000, vec![]),
            // Africa
            (
                "Cairo",
                "Egypt",
                30.0444,
                31.2357,
                10_230_350,
                vec!["القاهرة"],
            ),
            ("Lagos", "Nigeria", 6.5244, 3.3792, 14_368_000, vec![]),
            (
                "Johannesburg",
                "South Africa",
                26.2041,
                28.0473,
                5_635_127,
                vec!["Joburg"],
            ),
            (
                "Cape Town",
                "South Africa",
                -33.9249,
                18.4241,
                4_618_000,
                vec![],
            ),
            ("Nairobi", "Kenya", -1.2921, 36.8219, 4_397_073, vec![]),
            (
                "Addis Ababa",
                "Ethiopia",
                8.9806,
                38.7578,
                3_352_000,
                vec![],
            ),
            ("Casablanca", "Morocco", 33.5731, -7.5898, 3_359_818, vec![]),
            (
                "Algiers",
                "Algeria",
                36.7538,
                3.0588,
                2_988_145,
                vec!["Alger"],
            ),
            ("Tunis", "Tunisia", 36.8065, 10.1815, 1_056_247, vec![]),
            ("Accra", "Ghana", 5.6037, -0.1870, 2_291_352, vec![]),
            // South America
            (
                "São Paulo",
                "Brazil",
                -23.5505,
                -46.6333,
                12_325_232,
                vec!["Sao Paulo"],
            ),
            (
                "Rio de Janeiro",
                "Brazil",
                -22.9068,
                -43.1729,
                6_748_000,
                vec!["Rio"],
            ),
            (
                "Buenos Aires",
                "Argentina",
                -34.6037,
                -58.3816,
                2_891_082,
                vec![],
            ),
            ("Lima", "Peru", -12.0464, -77.0428, 9_751_717, vec![]),
            (
                "Bogotá",
                "Colombia",
                4.7110,
                -74.0721,
                7_412_566,
                vec!["Bogota"],
            ),
            ("Santiago", "Chile", -33.4489, -70.6693, 5_614_000, vec![]),
            ("Caracas", "Venezuela", 10.4806, -66.9036, 2_934_000, vec![]),
            ("Quito", "Ecuador", -0.1807, -78.4678, 2_011_388, vec![]),
            (
                "Montevideo",
                "Uruguay",
                -34.9011,
                -56.1645,
                1_947_604,
                vec![],
            ),
            // Oceania
            ("Sydney", "Australia", -33.8688, 151.2093, 5_312_163, vec![]),
            (
                "Melbourne",
                "Australia",
                -37.8136,
                144.9631,
                5_078_193,
                vec![],
            ),
            (
                "Brisbane",
                "Australia",
                -27.4698,
                153.0251,
                2_560_720,
                vec![],
            ),
            ("Perth", "Australia", -31.9505, 115.8605, 2_085_973, vec![]),
            (
                "Auckland",
                "New Zealand",
                -36.8509,
                174.7645,
                1_657_000,
                vec![],
            ),
            (
                "Wellington",
                "New Zealand",
                -41.2865,
                174.7762,
                215_400,
                vec![],
            ),
            // Countries (centroids)
            (
                "United States",
                "Country",
                37.0902,
                -95.7129,
                331_002_651,
                vec!["USA", "US", "America"],
            ),
            ("Canada", "Country", 56.1304, -106.3468, 38_005_238, vec![]),
            ("Mexico", "Country", 23.6345, -102.5528, 128_932_753, vec![]),
            (
                "United Kingdom",
                "Country",
                55.3781,
                -3.4360,
                67_886_011,
                vec!["UK", "Britain", "Great Britain"],
            ),
            ("France", "Country", 46.2276, 2.2137, 65_273_511, vec![]),
            (
                "Germany",
                "Country",
                51.1657,
                10.4515,
                83_783_942,
                vec!["Deutschland"],
            ),
            (
                "Italy",
                "Country",
                41.8719,
                12.5674,
                60_461_826,
                vec!["Italia"],
            ),
            (
                "Spain",
                "Country",
                40.4637,
                -3.7492,
                46_754_778,
                vec!["España"],
            ),
            (
                "China",
                "Country",
                35.8617,
                104.1954,
                1_439_323_776,
                vec!["PRC"],
            ),
            ("Japan", "Country", 36.2048, 138.2529, 126_476_461, vec![]),
            ("India", "Country", 20.5937, 78.9629, 1_380_004_385, vec![]),
            (
                "Brazil",
                "Country",
                -14.2350,
                -51.9253,
                212_559_417,
                vec!["Brasil"],
            ),
            (
                "Australia",
                "Country",
                -25.2744,
                133.7751,
                25_499_884,
                vec!["Oz"],
            ),
            (
                "Russia",
                "Country",
                61.5240,
                105.3188,
                145_912_025,
                vec!["Russian Federation"],
            ),
            (
                "South Korea",
                "Country",
                35.9078,
                127.7669,
                51_269_185,
                vec!["Korea", "ROK"],
            ),
            ("Ukraine", "Country", 48.3794, 31.1656, 43_733_762, vec![]),
            (
                "Poland",
                "Country",
                51.9194,
                19.1451,
                37_846_611,
                vec!["Polska"],
            ),
            (
                "Netherlands",
                "Country",
                52.1326,
                5.2913,
                17_134_872,
                vec!["Holland"],
            ),
            ("Belgium", "Country", 50.5039, 4.4699, 11_589_623, vec![]),
            (
                "Sweden",
                "Country",
                60.1282,
                18.6435,
                10_099_265,
                vec!["Sverige"],
            ),
            (
                "Norway",
                "Country",
                60.4720,
                8.4689,
                5_421_241,
                vec!["Norge"],
            ),
            (
                "Denmark",
                "Country",
                56.2639,
                9.5018,
                5_792_202,
                vec!["Danmark"],
            ),
            (
                "Finland",
                "Country",
                61.9241,
                25.7482,
                5_540_720,
                vec!["Suomi"],
            ),
            (
                "Switzerland",
                "Country",
                46.8182,
                8.2275,
                8_654_622,
                vec!["Schweiz", "Suisse"],
            ),
            (
                "Austria",
                "Country",
                47.5162,
                14.5501,
                9_006_398,
                vec!["Österreich"],
            ),
            ("Portugal", "Country", 39.3999, -8.2245, 10_196_709, vec![]),
            (
                "Greece",
                "Country",
                39.0742,
                21.8243,
                10_423_054,
                vec!["Hellas", "Ελλάδα"],
            ),
            (
                "Turkey",
                "Country",
                38.9637,
                35.2433,
                84_339_067,
                vec!["Türkiye"],
            ),
            ("Israel", "Country", 31.0461, 34.8516, 8_655_535, vec![]),
            (
                "Egypt",
                "Country",
                26.8206,
                30.8025,
                102_334_404,
                vec!["مصر"],
            ),
            (
                "South Africa",
                "Country",
                -30.5595,
                22.9375,
                59_308_690,
                vec!["RSA", "SA"],
            ),
            ("Nigeria", "Country", 9.0820, 8.6753, 206_139_589, vec![]),
            ("Kenya", "Country", -0.0236, 37.9062, 53_771_296, vec![]),
            (
                "Argentina",
                "Country",
                -38.4161,
                -63.6167,
                45_195_774,
                vec![],
            ),
            ("Chile", "Country", -35.6751, -71.5430, 19_116_201, vec![]),
            ("Colombia", "Country", 4.5709, -74.2973, 50_882_891, vec![]),
            ("Peru", "Country", -9.1900, -75.0152, 32_971_854, vec![]),
            ("Venezuela", "Country", 6.4238, -66.5897, 28_435_940, vec![]),
            (
                "New Zealand",
                "Country",
                -40.9006,
                174.8860,
                4_822_233,
                vec!["NZ", "Aotearoa"],
            ),
            (
                "Indonesia",
                "Country",
                -0.7893,
                113.9213,
                273_523_615,
                vec![],
            ),
            (
                "Philippines",
                "Country",
                12.8797,
                121.7740,
                109_581_078,
                vec![],
            ),
            ("Thailand", "Country", 15.8700, 100.9925, 69_799_978, vec![]),
            (
                "Vietnam",
                "Country",
                14.0583,
                108.2772,
                97_338_579,
                vec!["Việt Nam"],
            ),
            ("Malaysia", "Country", 4.2105, 101.9758, 32_365_999, vec![]),
            ("Singapore", "Country", 1.3521, 103.8198, 5_850_342, vec![]),
            (
                "Taiwan",
                "Country",
                23.6978,
                120.9605,
                23_816_775,
                vec!["ROC"],
            ),
            (
                "Iran",
                "Country",
                32.4279,
                53.6880,
                83_992_949,
                vec!["Persia"],
            ),
            (
                "Saudi Arabia",
                "Country",
                23.8859,
                45.0792,
                34_813_871,
                vec!["KSA"],
            ),
            (
                "United Arab Emirates",
                "Country",
                23.4241,
                53.8478,
                9_890_402,
                vec!["UAE", "Emirates"],
            ),
            ("Qatar", "Country", 25.3548, 51.1839, 2_881_053, vec![]),
        ];

        for (name, country, lat, lon, pop, aliases) in cities {
            self.add_entry(GazetteerEntry {
                name: name.to_string(),
                country: country.to_string(),
                lat,
                lon,
                population: pop,
                aliases: aliases.into_iter().map(String::from).collect(),
            });
        }
    }
}

impl Default for BuiltinGazetteer {
    fn default() -> Self {
        Self::new()
    }
}

impl Gazetteer for BuiltinGazetteer {
    fn lookup(&self, name: &str) -> Option<Location> {
        let lower = name.to_lowercase();
        self.name_to_canonical
            .get(&lower)
            .and_then(|canonical| self.entries.get(canonical))
            .map(|entry| Location::new(entry.lat, entry.lon))
    }

    fn contains(&self, name: &str) -> bool {
        self.name_to_canonical.contains_key(&name.to_lowercase())
    }

    fn all_names(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }

    fn aliases(&self, name: &str) -> Vec<&str> {
        let lower = name.to_lowercase();
        self.name_to_canonical
            .get(&lower)
            .and_then(|canonical| self.entries.get(canonical))
            .map(|entry| entry.aliases.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_gazetteer() {
        let gazetteer = BuiltinGazetteer::new();

        assert!(gazetteer.contains("London"));
        assert!(gazetteer.contains("Paris"));
        assert!(gazetteer.contains("Tokyo"));

        let london = gazetteer.lookup("London").unwrap();
        assert!((london.lat - 51.5074).abs() < 0.01);
    }

    #[test]
    fn test_gazetteer_aliases() {
        let gazetteer = BuiltinGazetteer::new();

        // NYC should resolve to New York City
        assert!(gazetteer.contains("NYC"));
        let nyc = gazetteer.lookup("NYC").unwrap();
        let new_york = gazetteer.lookup("New York City").unwrap();
        assert!((nyc.lat - new_york.lat).abs() < 0.0001);
    }

    #[test]
    fn test_gazetteer_countries() {
        let gazetteer = BuiltinGazetteer::new();

        assert!(gazetteer.contains("USA"));
        assert!(gazetteer.contains("United States"));
        assert!(gazetteer.contains("UK"));
        assert!(gazetteer.contains("Britain"));
    }

    #[test]
    fn test_geocode() {
        let gazetteer = BuiltinGazetteer::new();

        let berlin = gazetteer.lookup("Berlin").unwrap();
        assert!((berlin.lat - 52.52).abs() < 0.01);
        assert!((berlin.lon - 13.405).abs() < 0.01);
    }

    #[test]
    fn test_multi_gazetteer() {
        let mut multi = MultiGazetteer::new();
        multi.add_source(Box::new(BuiltinGazetteer::new()));

        // Should find in built-in
        let paris = multi.lookup("Paris").unwrap();
        assert!((paris.lat - 48.8566).abs() < 0.01);

        // Test contains
        assert!(multi.contains("London"));
        assert!(!multi.contains("NonexistentPlace12345"));
    }

    #[test]
    fn test_multi_gazetteer_fallback() {
        let builtin = BuiltinGazetteer::new();
        
        let mut multi = MultiGazetteer::new();
        multi.add_source(Box::new(builtin));

        // Should try built-in first and succeed
        assert!(multi.lookup("Tokyo").is_some());
    }

    #[cfg(feature = "geocoding")]
    #[test]
    #[ignore] // Ignore by default to avoid hitting real APIs in tests
    fn test_nominatim_integration() {
        let gaz = GazetteerNominatim::new();
        
        // Test lookup
        if let Some(loc) = gaz.lookup("Berlin, Germany") {
            assert!((loc.lat - 52.52).abs() < 1.0);
            assert!((loc.lon - 13.4).abs() < 1.0);
        }
    }

    #[cfg(feature = "geocoding")]
    #[test]
    #[ignore] // Requires GeoNames username
    fn test_geonames_integration() {
        // Set GEONAMES_USERNAME environment variable to run this test
        if let Ok(username) = std::env::var("GEONAMES_USERNAME") {
            let gaz = GazetteerGeoNames::new(username);
            
            if let Some(loc) = gaz.lookup("Paris") {
                assert!((loc.lat - 48.85).abs() < 1.0);
                assert!((loc.lon - 2.35).abs() < 1.0);
            }
        }
    }

    #[cfg(feature = "geocoding")]
    #[test]
    #[ignore] // Ignore by default to avoid hitting real APIs
    fn test_wikidata_integration() {
        let gaz = GazetteerWikidata::new();
        
        if let Some(loc) = gaz.lookup("London") {
            assert!((loc.lat - 51.5).abs() < 1.0);
            assert!((loc.lon + 0.1).abs() < 1.0);
        }
    }

    #[cfg(feature = "geocoding")]
    #[test]
    #[ignore] // Requires network access
    fn test_multi_with_apis() {
        let mut multi = MultiGazetteer::new();
        multi.add_source(Box::new(BuiltinGazetteer::new()));
        multi.add_source(Box::new(GazetteerNominatim::new()));

        // Should find in built-in first
        let paris = multi.lookup("Paris");
        assert!(paris.is_some());
    }
}

