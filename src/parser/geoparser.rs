//! GeoParser for extracting geographic locations from text.

use crate::core::Location;
use once_cell::sync::Lazy;
use regex::Regex;

use super::gazetteer::Gazetteer;
use super::mention::{LocationMention, LocationPattern, MentionType};

// Compile regex patterns once
static DECIMAL_DEGREES: Lazy<Regex> = Lazy::new(|| {
    // Match decimal degrees like "40.7128, -74.0060" or "40.7128 -74.0060"
    Regex::new(r"(-?\d{1,3}\.\d+)\s*[,\s]\s*(-?\d{1,3}\.\d+)").unwrap()
});

static DEGREES_WITH_SYMBOLS: Lazy<Regex> = Lazy::new(|| {
    // Match degrees with direction: "40.7128°N, 74.0060°W" or "40.7128N 74.0060W"
    Regex::new(r"(?i)(\d{1,3}\.?\d*)\s*°?\s*([NS])\s*[,\s]*(\d{1,3}\.?\d*)\s*°?\s*([EW])").unwrap()
});

static DMS_FORMAT: Lazy<Regex> = Lazy::new(|| {
    // Match DMS: 40°42'46"N, 74°0'22"W (with various quote styles)
    Regex::new(r#"(?i)(\d{1,3})\s*°\s*(\d{1,2})\s*['\u{2032}]\s*(\d{1,2}(?:\.\d+)?)\s*["\u{2033}]?\s*([NS])\s*[,\s]*(\d{1,3})\s*°\s*(\d{1,2})\s*['\u{2032}]\s*(\d{1,2}(?:\.\d+)?)\s*["\u{2033}]?\s*([EW])"#).unwrap()
});

/// Main geoparser for extracting locations from text.
///
/// # Example
///
/// ```rust
/// use spatial_narrative::parser::GeoParser;
///
/// let parser = GeoParser::new();
/// let mentions = parser.extract("Meeting at 40.7128, -74.0060");
///
/// assert_eq!(mentions.len(), 1);
/// assert!(mentions[0].location.is_some());
/// ```
pub struct GeoParser {
    pattern: LocationPattern,
    gazetteer: Option<Box<dyn Gazetteer>>,
}

impl GeoParser {
    /// Create a new geoparser with default settings.
    pub fn new() -> Self {
        Self {
            pattern: LocationPattern::default(),
            gazetteer: None,
        }
    }

    /// Create a geoparser with a gazetteer for place name resolution.
    pub fn with_gazetteer(gazetteer: Box<dyn Gazetteer>) -> Self {
        Self {
            pattern: LocationPattern::default(),
            gazetteer: Some(gazetteer),
        }
    }

    /// Create a geoparser with custom pattern configuration.
    pub fn with_pattern(pattern: LocationPattern) -> Self {
        Self {
            pattern,
            gazetteer: None,
        }
    }

    /// Set the gazetteer for place name resolution.
    pub fn set_gazetteer(&mut self, gazetteer: Box<dyn Gazetteer>) {
        self.gazetteer = Some(gazetteer);
    }

    /// Set the pattern configuration.
    pub fn set_pattern(&mut self, pattern: LocationPattern) {
        self.pattern = pattern;
    }

    /// Extract all location mentions from text.
    pub fn extract(&self, text: &str) -> Vec<LocationMention> {
        let mut mentions = Vec::new();

        // Extract coordinate patterns
        if self.pattern.detect_decimal {
            self.extract_decimal_degrees(text, &mut mentions);
        }
        if self.pattern.detect_symbols {
            self.extract_degrees_with_symbols(text, &mut mentions);
        }
        if self.pattern.detect_dms {
            self.extract_dms(text, &mut mentions);
        }

        // Extract place names from gazetteer
        if self.pattern.detect_places {
            if let Some(ref gazetteer) = self.gazetteer {
                self.extract_place_names(text, gazetteer.as_ref(), &mut mentions);
            }
        }

        // Filter by confidence
        mentions.retain(|m| m.confidence >= self.pattern.min_confidence);

        // Sort by position in text
        mentions.sort_by_key(|m| m.start);

        // Remove overlapping mentions (keep higher confidence)
        self.remove_overlaps(&mut mentions);

        mentions
    }

    /// Extract coordinates from text without resolving place names.
    pub fn extract_coordinates(&self, text: &str) -> Vec<LocationMention> {
        let mut mentions = Vec::new();

        self.extract_decimal_degrees(text, &mut mentions);
        self.extract_degrees_with_symbols(text, &mut mentions);
        self.extract_dms(text, &mut mentions);

        mentions.sort_by_key(|m| m.start);
        self.remove_overlaps(&mut mentions);

        mentions
    }

    /// Geocode a place name using the configured gazetteer.
    pub fn geocode(&self, name: &str) -> Option<Location> {
        self.gazetteer.as_ref().and_then(|g| g.lookup(name))
    }

    fn extract_decimal_degrees(&self, text: &str, mentions: &mut Vec<LocationMention>) {
        for cap in DECIMAL_DEGREES.captures_iter(text) {
            let full_match = cap.get(0).unwrap();
            let lat_str = cap.get(1).unwrap().as_str();
            let lon_str = cap.get(2).unwrap().as_str();

            if let (Ok(lat), Ok(lon)) = (lat_str.parse::<f64>(), lon_str.parse::<f64>()) {
                // Validate coordinate ranges
                if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon) {
                    let mut mention = LocationMention::new(
                        full_match.as_str(),
                        full_match.start(),
                        full_match.end(),
                        MentionType::DecimalDegrees,
                    );
                    mention.location = Some(Location::new(lat, lon));
                    mention.confidence = 0.95;
                    mentions.push(mention);
                }
            }
        }
    }

    fn extract_degrees_with_symbols(&self, text: &str, mentions: &mut Vec<LocationMention>) {
        for cap in DEGREES_WITH_SYMBOLS.captures_iter(text) {
            let full_match = cap.get(0).unwrap();
            let lat_val: f64 = cap.get(1).unwrap().as_str().parse().unwrap_or(0.0);
            let lat_dir = cap.get(2).unwrap().as_str().to_uppercase();
            let lon_val: f64 = cap.get(3).unwrap().as_str().parse().unwrap_or(0.0);
            let lon_dir = cap.get(4).unwrap().as_str().to_uppercase();

            let lat = if lat_dir == "S" { -lat_val } else { lat_val };
            let lon = if lon_dir == "W" { -lon_val } else { lon_val };

            if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon) {
                let mut mention = LocationMention::new(
                    full_match.as_str(),
                    full_match.start(),
                    full_match.end(),
                    MentionType::DegreesWithSymbols,
                );
                mention.location = Some(Location::new(lat, lon));
                mention.confidence = 0.98;
                mentions.push(mention);
            }
        }
    }

    fn extract_dms(&self, text: &str, mentions: &mut Vec<LocationMention>) {
        for cap in DMS_FORMAT.captures_iter(text) {
            let full_match = cap.get(0).unwrap();

            let lat_deg: f64 = cap.get(1).unwrap().as_str().parse().unwrap_or(0.0);
            let lat_min: f64 = cap.get(2).unwrap().as_str().parse().unwrap_or(0.0);
            let lat_sec: f64 = cap.get(3).unwrap().as_str().parse().unwrap_or(0.0);
            let lat_dir = cap.get(4).unwrap().as_str().to_uppercase();

            let lon_deg: f64 = cap.get(5).unwrap().as_str().parse().unwrap_or(0.0);
            let lon_min: f64 = cap.get(6).unwrap().as_str().parse().unwrap_or(0.0);
            let lon_sec: f64 = cap.get(7).unwrap().as_str().parse().unwrap_or(0.0);
            let lon_dir = cap.get(8).unwrap().as_str().to_uppercase();

            let lat = dms_to_decimal(lat_deg, lat_min, lat_sec, &lat_dir);
            let lon = dms_to_decimal(lon_deg, lon_min, lon_sec, &lon_dir);

            if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon) {
                let mut mention = LocationMention::new(
                    full_match.as_str(),
                    full_match.start(),
                    full_match.end(),
                    MentionType::DMS,
                );
                mention.location = Some(Location::new(lat, lon));
                mention.confidence = 0.99;
                mentions.push(mention);
            }
        }
    }

    fn extract_place_names(
        &self,
        text: &str,
        gazetteer: &dyn Gazetteer,
        mentions: &mut Vec<LocationMention>,
    ) {
        // Get all place names and sort by length (longest first for greedy matching)
        let mut names: Vec<_> = gazetteer.all_names().to_vec();
        names.sort_by_key(|b| std::cmp::Reverse(b.len()));

        let text_lower = text.to_lowercase();

        for name in names {
            let name_lower = name.to_lowercase();

            // Find all occurrences of this name in the text
            let mut start = 0;
            while let Some(pos) = text_lower[start..].find(&name_lower) {
                let abs_pos = start + pos;
                let end_pos = abs_pos + name.len();

                // Check word boundaries
                let valid_start =
                    abs_pos == 0 || !text.chars().nth(abs_pos - 1).unwrap().is_alphanumeric();
                let valid_end =
                    end_pos >= text.len() || !text.chars().nth(end_pos).unwrap().is_alphanumeric();

                if valid_start && valid_end {
                    // Check for overlap with existing mentions
                    let overlaps = mentions
                        .iter()
                        .any(|m| !(end_pos <= m.start || abs_pos >= m.end));

                    if !overlaps {
                        if let Some(location) = gazetteer.lookup(name) {
                            let original_text = &text[abs_pos..end_pos];
                            let mut mention = LocationMention::new(
                                original_text,
                                abs_pos,
                                end_pos,
                                MentionType::PlaceName,
                            );
                            mention.location = Some(location);
                            mention.confidence = 0.85;
                            mentions.push(mention);
                        }
                    }
                }

                start = abs_pos + 1;
            }
        }
    }

    fn remove_overlaps(&self, mentions: &mut Vec<LocationMention>) {
        if mentions.len() <= 1 {
            return;
        }

        // Sort by start position, then by confidence (descending)
        mentions.sort_by(|a, b| {
            a.start
                .cmp(&b.start)
                .then(b.confidence.partial_cmp(&a.confidence).unwrap())
        });

        let mut result = Vec::new();
        let mut last_end = 0;

        for mention in mentions.drain(..) {
            if mention.start >= last_end {
                last_end = mention.end;
                result.push(mention);
            }
        }

        *mentions = result;
    }
}

impl Default for GeoParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert degrees, minutes, seconds to decimal degrees.
fn dms_to_decimal(degrees: f64, minutes: f64, seconds: f64, direction: &str) -> f64 {
    let decimal = degrees + minutes / 60.0 + seconds / 3600.0;
    if direction == "S" || direction == "W" {
        -decimal
    } else {
        decimal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::BuiltinGazetteer;

    #[test]
    fn test_decimal_degrees_extraction() {
        let parser = GeoParser::new();
        let text = "The location is 40.7128, -74.0060 near the city.";

        let mentions = parser.extract(text);

        assert_eq!(mentions.len(), 1);
        assert!(matches!(
            mentions[0].mention_type,
            MentionType::DecimalDegrees
        ));
        assert!(mentions[0].location.is_some());

        let loc = mentions[0].location.as_ref().unwrap();
        assert!((loc.lat - 40.7128).abs() < 0.0001);
        assert!((loc.lon - (-74.0060)).abs() < 0.0001);
    }

    #[test]
    fn test_degrees_with_symbols_extraction() {
        let parser = GeoParser::new();
        let text = "Position: 40.7128°N, 74.0060°W";

        let mentions = parser.extract(text);

        assert_eq!(mentions.len(), 1);
        assert!(matches!(
            mentions[0].mention_type,
            MentionType::DegreesWithSymbols
        ));

        let loc = mentions[0].location.as_ref().unwrap();
        assert!((loc.lat - 40.7128).abs() < 0.0001);
        assert!((loc.lon - (-74.0060)).abs() < 0.0001);
    }

    #[test]
    fn test_dms_extraction() {
        let parser = GeoParser::new();
        let text = "Coordinates: 40°42'46\"N, 74°0'22\"W";

        let mentions = parser.extract(text);

        assert_eq!(mentions.len(), 1);
        assert!(matches!(mentions[0].mention_type, MentionType::DMS));

        let loc = mentions[0].location.as_ref().unwrap();
        // 40°42'46"N = 40 + 42/60 + 46/3600 ≈ 40.7128
        assert!((loc.lat - 40.7128).abs() < 0.001);
        // 74°0'22"W = -(74 + 0/60 + 22/3600) ≈ -74.0061
        assert!((loc.lon - (-74.0061)).abs() < 0.001);
    }

    #[test]
    fn test_place_name_extraction() {
        let gazetteer = BuiltinGazetteer::new();
        let parser = GeoParser::with_gazetteer(Box::new(gazetteer));

        let text = "The conference was held in Tokyo and participants came from London.";
        let mentions = parser.extract(text);

        assert_eq!(mentions.len(), 2);
        assert!(mentions.iter().any(|m| m.text == "Tokyo"));
        assert!(mentions.iter().any(|m| m.text == "London"));
    }

    #[test]
    fn test_mixed_extraction() {
        let gazetteer = BuiltinGazetteer::new();
        let parser = GeoParser::with_gazetteer(Box::new(gazetteer));

        let text = "Meeting point: 40.7128, -74.0060 (near New York City)";
        let mentions = parser.extract(text);

        // Should find both the coordinates and the place name
        assert!(!mentions.is_empty());
        assert!(mentions
            .iter()
            .any(|m| matches!(m.mention_type, MentionType::DecimalDegrees)));
    }

    #[test]
    fn test_invalid_coordinates_ignored() {
        let parser = GeoParser::new();

        // Latitude out of range
        let text = "Invalid: 100.0, -74.0";
        let mentions = parser.extract(text);
        assert!(mentions.is_empty());

        // Longitude out of range
        let text = "Invalid: 40.0, -200.0";
        let mentions = parser.extract(text);
        assert!(mentions.is_empty());
    }

    #[test]
    fn test_multiple_coordinates() {
        let parser = GeoParser::new();
        let text = "From 40.7128, -74.0060 to 34.0522, -118.2437";

        let mentions = parser.extract(text);

        assert_eq!(mentions.len(), 2);

        let loc1 = mentions[0].location.as_ref().unwrap();
        assert!((loc1.lat - 40.7128).abs() < 0.0001);

        let loc2 = mentions[1].location.as_ref().unwrap();
        assert!((loc2.lat - 34.0522).abs() < 0.0001);
    }

    #[test]
    fn test_coordinates_only_pattern() {
        let gazetteer = BuiltinGazetteer::new();
        let mut parser = GeoParser::with_gazetteer(Box::new(gazetteer));
        parser.set_pattern(LocationPattern::coordinates_only());

        let text = "Meeting in London at 51.5074, -0.1278";
        let mentions = parser.extract(text);

        // Should only find coordinates, not "London"
        assert_eq!(mentions.len(), 1);
        assert!(matches!(
            mentions[0].mention_type,
            MentionType::DecimalDegrees
        ));
    }

    #[test]
    fn test_geocode() {
        let gazetteer = BuiltinGazetteer::new();
        let parser = GeoParser::with_gazetteer(Box::new(gazetteer));

        let loc = parser.geocode("Paris").unwrap();
        // Built-in gazetteer has Paris at (48.8534, 2.3488)
        assert!((loc.lat - 48.8534).abs() < 0.01);
        assert!((loc.lon - 2.3488).abs() < 0.01);

        assert!(parser.geocode("NonexistentPlace").is_none());
    }

    #[test]
    fn test_dms_to_decimal() {
        // New York City: 40°42'46"N, 74°0'22"W
        let lat = dms_to_decimal(40.0, 42.0, 46.0, "N");
        let lon = dms_to_decimal(74.0, 0.0, 22.0, "W");

        assert!((lat - 40.7128).abs() < 0.001);
        assert!((lon - (-74.0061)).abs() < 0.001);

        // Southern hemisphere
        let lat_s = dms_to_decimal(33.0, 52.0, 10.0, "S");
        assert!(lat_s < 0.0);
    }
}
