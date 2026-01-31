//! Gazetteer trait and built-in implementation for place name resolution.

use crate::core::Location;
use std::collections::HashMap;

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
            ("New York City", "United States", 40.7128, -74.0060, 8_336_817, vec!["NYC", "New York", "NY"]),
            ("Los Angeles", "United States", 34.0522, -118.2437, 3_979_576, vec!["LA", "L.A."]),
            ("Chicago", "United States", 41.8781, -87.6298, 2_693_976, vec![]),
            ("Houston", "United States", 29.7604, -95.3698, 2_320_268, vec![]),
            ("Phoenix", "United States", 33.4484, -112.0740, 1_680_992, vec![]),
            ("Philadelphia", "United States", 39.9526, -75.1652, 1_584_064, vec!["Philly"]),
            ("San Antonio", "United States", 29.4241, -98.4936, 1_547_253, vec![]),
            ("San Diego", "United States", 32.7157, -117.1611, 1_423_851, vec![]),
            ("Dallas", "United States", 32.7767, -96.7970, 1_343_573, vec![]),
            ("San Francisco", "United States", 37.7749, -122.4194, 883_305, vec!["SF"]),
            ("Seattle", "United States", 47.6062, -122.3321, 753_675, vec![]),
            ("Boston", "United States", 42.3601, -71.0589, 692_600, vec![]),
            ("Washington", "United States", 38.9072, -77.0369, 689_545, vec!["Washington D.C.", "D.C.", "DC"]),
            ("Miami", "United States", 25.7617, -80.1918, 467_963, vec![]),
            ("Denver", "United States", 39.7392, -104.9903, 727_211, vec![]),
            ("Atlanta", "United States", 33.7490, -84.3880, 498_715, vec!["ATL"]),
            ("Toronto", "Canada", 43.6532, -79.3832, 2_930_000, vec![]),
            ("Vancouver", "Canada", 49.2827, -123.1207, 675_218, vec![]),
            ("Montreal", "Canada", 45.5017, -73.5673, 1_780_000, vec![]),
            ("Mexico City", "Mexico", 19.4326, -99.1332, 8_918_653, vec!["CDMX", "Ciudad de México"]),
            
            // Europe
            ("London", "United Kingdom", 51.5074, -0.1278, 8_982_000, vec![]),
            ("Paris", "France", 48.8566, 2.3522, 2_161_000, vec![]),
            ("Berlin", "Germany", 52.5200, 13.4050, 3_769_495, vec![]),
            ("Madrid", "Spain", 40.4168, -3.7038, 3_223_334, vec![]),
            ("Rome", "Italy", 41.9028, 12.4964, 2_872_800, vec!["Roma"]),
            ("Amsterdam", "Netherlands", 52.3676, 4.9041, 872_680, vec![]),
            ("Vienna", "Austria", 48.2082, 16.3738, 1_911_191, vec!["Wien"]),
            ("Brussels", "Belgium", 50.8503, 4.3517, 1_208_542, vec!["Bruxelles", "Brussel"]),
            ("Stockholm", "Sweden", 59.3293, 18.0686, 975_904, vec![]),
            ("Oslo", "Norway", 59.9139, 10.7522, 693_494, vec![]),
            ("Copenhagen", "Denmark", 55.6761, 12.5683, 794_128, vec!["København"]),
            ("Helsinki", "Finland", 60.1699, 24.9384, 656_229, vec![]),
            ("Dublin", "Ireland", 53.3498, -6.2603, 554_554, vec![]),
            ("Lisbon", "Portugal", 38.7223, -9.1393, 504_718, vec!["Lisboa"]),
            ("Athens", "Greece", 37.9838, 23.7275, 664_046, vec!["Αθήνα"]),
            ("Warsaw", "Poland", 52.2297, 21.0122, 1_790_658, vec!["Warszawa"]),
            ("Prague", "Czech Republic", 50.0755, 14.4378, 1_309_000, vec!["Praha"]),
            ("Budapest", "Hungary", 47.4979, 19.0402, 1_756_000, vec![]),
            ("Bucharest", "Romania", 44.4268, 26.1025, 1_883_425, vec!["București"]),
            ("Kyiv", "Ukraine", 50.4501, 30.5234, 2_952_301, vec!["Kiev"]),
            ("Moscow", "Russia", 55.7558, 37.6173, 12_615_279, vec!["Москва"]),
            ("Saint Petersburg", "Russia", 59.9311, 30.3609, 5_383_890, vec!["St. Petersburg", "St Petersburg"]),
            ("Istanbul", "Turkey", 41.0082, 28.9784, 15_462_452, vec!["Constantinople"]),
            ("Zurich", "Switzerland", 47.3769, 8.5417, 415_367, vec!["Zürich"]),
            ("Geneva", "Switzerland", 46.2044, 6.1432, 201_818, vec!["Genève"]),
            ("Munich", "Germany", 48.1351, 11.5820, 1_471_508, vec!["München"]),
            ("Milan", "Italy", 45.4642, 9.1900, 1_378_689, vec!["Milano"]),
            ("Barcelona", "Spain", 41.3851, 2.1734, 1_620_343, vec![]),
            
            // Asia
            ("Tokyo", "Japan", 35.6762, 139.6503, 13_960_000, vec!["東京"]),
            ("Beijing", "China", 39.9042, 116.4074, 21_540_000, vec!["Peking", "北京"]),
            ("Shanghai", "China", 31.2304, 121.4737, 26_320_000, vec!["上海"]),
            ("Hong Kong", "China", 22.3193, 114.1694, 7_496_981, vec!["HK"]),
            ("Singapore", "Singapore", 1.3521, 103.8198, 5_850_342, vec![]),
            ("Seoul", "South Korea", 37.5665, 126.9780, 9_733_509, vec!["서울"]),
            ("Mumbai", "India", 19.0760, 72.8777, 20_411_274, vec!["Bombay"]),
            ("Delhi", "India", 28.6139, 77.2090, 16_787_941, vec!["New Delhi"]),
            ("Bangkok", "Thailand", 13.7563, 100.5018, 8_281_099, vec!["กรุงเทพ"]),
            ("Taipei", "Taiwan", 25.0330, 121.5654, 2_646_204, vec!["臺北"]),
            ("Jakarta", "Indonesia", 6.2088, 106.8456, 10_562_088, vec![]),
            ("Manila", "Philippines", 14.5995, 120.9842, 1_846_513, vec![]),
            ("Kuala Lumpur", "Malaysia", 3.1390, 101.6869, 1_808_000, vec!["KL"]),
            ("Dubai", "United Arab Emirates", 25.2048, 55.2708, 3_400_800, vec![]),
            ("Tel Aviv", "Israel", 32.0853, 34.7818, 460_613, vec!["תל אביב"]),
            ("Jerusalem", "Israel", 31.7683, 35.2137, 936_425, vec!["ירושלים"]),
            ("Riyadh", "Saudi Arabia", 24.7136, 46.6753, 7_676_654, vec![]),
            ("Tehran", "Iran", 35.6892, 51.3890, 8_693_706, vec!["تهران"]),
            ("Doha", "Qatar", 25.2854, 51.5310, 2_382_000, vec![]),
            
            // Africa
            ("Cairo", "Egypt", 30.0444, 31.2357, 10_230_350, vec!["القاهرة"]),
            ("Lagos", "Nigeria", 6.5244, 3.3792, 14_368_000, vec![]),
            ("Johannesburg", "South Africa", 26.2041, 28.0473, 5_635_127, vec!["Joburg"]),
            ("Cape Town", "South Africa", -33.9249, 18.4241, 4_618_000, vec![]),
            ("Nairobi", "Kenya", -1.2921, 36.8219, 4_397_073, vec![]),
            ("Addis Ababa", "Ethiopia", 8.9806, 38.7578, 3_352_000, vec![]),
            ("Casablanca", "Morocco", 33.5731, -7.5898, 3_359_818, vec![]),
            ("Algiers", "Algeria", 36.7538, 3.0588, 2_988_145, vec!["Alger"]),
            ("Tunis", "Tunisia", 36.8065, 10.1815, 1_056_247, vec![]),
            ("Accra", "Ghana", 5.6037, -0.1870, 2_291_352, vec![]),
            
            // South America
            ("São Paulo", "Brazil", -23.5505, -46.6333, 12_325_232, vec!["Sao Paulo"]),
            ("Rio de Janeiro", "Brazil", -22.9068, -43.1729, 6_748_000, vec!["Rio"]),
            ("Buenos Aires", "Argentina", -34.6037, -58.3816, 2_891_082, vec![]),
            ("Lima", "Peru", -12.0464, -77.0428, 9_751_717, vec![]),
            ("Bogotá", "Colombia", 4.7110, -74.0721, 7_412_566, vec!["Bogota"]),
            ("Santiago", "Chile", -33.4489, -70.6693, 5_614_000, vec![]),
            ("Caracas", "Venezuela", 10.4806, -66.9036, 2_934_000, vec![]),
            ("Quito", "Ecuador", -0.1807, -78.4678, 2_011_388, vec![]),
            ("Montevideo", "Uruguay", -34.9011, -56.1645, 1_947_604, vec![]),
            
            // Oceania
            ("Sydney", "Australia", -33.8688, 151.2093, 5_312_163, vec![]),
            ("Melbourne", "Australia", -37.8136, 144.9631, 5_078_193, vec![]),
            ("Brisbane", "Australia", -27.4698, 153.0251, 2_560_720, vec![]),
            ("Perth", "Australia", -31.9505, 115.8605, 2_085_973, vec![]),
            ("Auckland", "New Zealand", -36.8509, 174.7645, 1_657_000, vec![]),
            ("Wellington", "New Zealand", -41.2865, 174.7762, 215_400, vec![]),
            
            // Countries (centroids)
            ("United States", "Country", 37.0902, -95.7129, 331_002_651, vec!["USA", "US", "America"]),
            ("Canada", "Country", 56.1304, -106.3468, 38_005_238, vec![]),
            ("Mexico", "Country", 23.6345, -102.5528, 128_932_753, vec![]),
            ("United Kingdom", "Country", 55.3781, -3.4360, 67_886_011, vec!["UK", "Britain", "Great Britain"]),
            ("France", "Country", 46.2276, 2.2137, 65_273_511, vec![]),
            ("Germany", "Country", 51.1657, 10.4515, 83_783_942, vec!["Deutschland"]),
            ("Italy", "Country", 41.8719, 12.5674, 60_461_826, vec!["Italia"]),
            ("Spain", "Country", 40.4637, -3.7492, 46_754_778, vec!["España"]),
            ("China", "Country", 35.8617, 104.1954, 1_439_323_776, vec!["PRC"]),
            ("Japan", "Country", 36.2048, 138.2529, 126_476_461, vec![]),
            ("India", "Country", 20.5937, 78.9629, 1_380_004_385, vec![]),
            ("Brazil", "Country", -14.2350, -51.9253, 212_559_417, vec!["Brasil"]),
            ("Australia", "Country", -25.2744, 133.7751, 25_499_884, vec!["Oz"]),
            ("Russia", "Country", 61.5240, 105.3188, 145_912_025, vec!["Russian Federation"]),
            ("South Korea", "Country", 35.9078, 127.7669, 51_269_185, vec!["Korea", "ROK"]),
            ("Ukraine", "Country", 48.3794, 31.1656, 43_733_762, vec![]),
            ("Poland", "Country", 51.9194, 19.1451, 37_846_611, vec!["Polska"]),
            ("Netherlands", "Country", 52.1326, 5.2913, 17_134_872, vec!["Holland"]),
            ("Belgium", "Country", 50.5039, 4.4699, 11_589_623, vec![]),
            ("Sweden", "Country", 60.1282, 18.6435, 10_099_265, vec!["Sverige"]),
            ("Norway", "Country", 60.4720, 8.4689, 5_421_241, vec!["Norge"]),
            ("Denmark", "Country", 56.2639, 9.5018, 5_792_202, vec!["Danmark"]),
            ("Finland", "Country", 61.9241, 25.7482, 5_540_720, vec!["Suomi"]),
            ("Switzerland", "Country", 46.8182, 8.2275, 8_654_622, vec!["Schweiz", "Suisse"]),
            ("Austria", "Country", 47.5162, 14.5501, 9_006_398, vec!["Österreich"]),
            ("Portugal", "Country", 39.3999, -8.2245, 10_196_709, vec![]),
            ("Greece", "Country", 39.0742, 21.8243, 10_423_054, vec!["Hellas", "Ελλάδα"]),
            ("Turkey", "Country", 38.9637, 35.2433, 84_339_067, vec!["Türkiye"]),
            ("Israel", "Country", 31.0461, 34.8516, 8_655_535, vec![]),
            ("Egypt", "Country", 26.8206, 30.8025, 102_334_404, vec!["مصر"]),
            ("South Africa", "Country", -30.5595, 22.9375, 59_308_690, vec!["RSA", "SA"]),
            ("Nigeria", "Country", 9.0820, 8.6753, 206_139_589, vec![]),
            ("Kenya", "Country", -0.0236, 37.9062, 53_771_296, vec![]),
            ("Argentina", "Country", -38.4161, -63.6167, 45_195_774, vec![]),
            ("Chile", "Country", -35.6751, -71.5430, 19_116_201, vec![]),
            ("Colombia", "Country", 4.5709, -74.2973, 50_882_891, vec![]),
            ("Peru", "Country", -9.1900, -75.0152, 32_971_854, vec![]),
            ("Venezuela", "Country", 6.4238, -66.5897, 28_435_940, vec![]),
            ("New Zealand", "Country", -40.9006, 174.8860, 4_822_233, vec!["NZ", "Aotearoa"]),
            ("Indonesia", "Country", -0.7893, 113.9213, 273_523_615, vec![]),
            ("Philippines", "Country", 12.8797, 121.7740, 109_581_078, vec![]),
            ("Thailand", "Country", 15.8700, 100.9925, 69_799_978, vec![]),
            ("Vietnam", "Country", 14.0583, 108.2772, 97_338_579, vec!["Việt Nam"]),
            ("Malaysia", "Country", 4.2105, 101.9758, 32_365_999, vec![]),
            ("Singapore", "Country", 1.3521, 103.8198, 5_850_342, vec![]),
            ("Taiwan", "Country", 23.6978, 120.9605, 23_816_775, vec!["ROC"]),
            ("Iran", "Country", 32.4279, 53.6880, 83_992_949, vec!["Persia"]),
            ("Saudi Arabia", "Country", 23.8859, 45.0792, 34_813_871, vec!["KSA"]),
            ("United Arab Emirates", "Country", 23.4241, 53.8478, 9_890_402, vec!["UAE", "Emirates"]),
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
}
