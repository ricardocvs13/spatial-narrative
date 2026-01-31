//! Location mention types for geoparsing.

use crate::core::Location;

/// Type of location mention detected in text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MentionType {
    /// Decimal degrees format: "40.7128, -74.0060"
    DecimalDegrees,
    /// Degrees with direction symbols: "40.7128°N, 74.0060°W"
    DegreesWithSymbols,
    /// Degrees, minutes, seconds: "40°42'46\"N, 74°0'22\"W"
    DMS,
    /// Named place from gazetteer
    PlaceName,
    /// Street address (detected but not geocoded)
    Address,
}

/// A location mention extracted from text.
#[derive(Debug, Clone)]
pub struct LocationMention {
    /// The original text of the mention
    pub text: String,
    /// Start position in the source text (byte offset)
    pub start: usize,
    /// End position in the source text (byte offset)
    pub end: usize,
    /// Type of mention
    pub mention_type: MentionType,
    /// Resolved location coordinates (if available)
    pub location: Option<Location>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
}

impl LocationMention {
    /// Create a new location mention.
    pub fn new(
        text: impl Into<String>,
        start: usize,
        end: usize,
        mention_type: MentionType,
    ) -> Self {
        Self {
            text: text.into(),
            start,
            end,
            mention_type,
            location: None,
            confidence: 1.0,
        }
    }

    /// Set the resolved location.
    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    /// Set the confidence score.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Returns true if this mention has resolved coordinates.
    pub fn is_resolved(&self) -> bool {
        self.location.is_some()
    }
}

/// Configuration for location pattern matching.
#[derive(Debug, Clone)]
pub struct LocationPattern {
    /// Enable decimal degrees detection
    pub detect_decimal: bool,
    /// Enable degrees with symbols detection
    pub detect_symbols: bool,
    /// Enable DMS format detection
    pub detect_dms: bool,
    /// Enable place name detection (requires gazetteer)
    pub detect_places: bool,
    /// Minimum confidence threshold for matches
    pub min_confidence: f64,
}

impl Default for LocationPattern {
    fn default() -> Self {
        Self {
            detect_decimal: true,
            detect_symbols: true,
            detect_dms: true,
            detect_places: true,
            min_confidence: 0.5,
        }
    }
}

impl LocationPattern {
    /// Create a new pattern configuration with all detection enabled.
    pub fn new() -> Self {
        Self::default()
    }

    /// Only detect coordinate patterns (no place names).
    pub fn coordinates_only() -> Self {
        Self {
            detect_decimal: true,
            detect_symbols: true,
            detect_dms: true,
            detect_places: false,
            min_confidence: 0.5,
        }
    }

    /// Only detect place names (no coordinates).
    pub fn places_only() -> Self {
        Self {
            detect_decimal: false,
            detect_symbols: false,
            detect_dms: false,
            detect_places: true,
            min_confidence: 0.5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mention_creation() {
        let mention = LocationMention::new("Berlin", 0, 6, MentionType::PlaceName);
        assert_eq!(mention.text, "Berlin");
        assert_eq!(mention.mention_type, MentionType::PlaceName);
        assert_eq!(mention.confidence, 1.0);
        assert!(!mention.is_resolved());
    }

    #[test]
    fn test_mention_with_location() {
        let mention = LocationMention::new("Berlin", 0, 6, MentionType::PlaceName)
            .with_location(Location::new(52.52, 13.405));
        assert!(mention.is_resolved());
        assert!((mention.location.unwrap().lat - 52.52).abs() < 0.001);
    }

    #[test]
    fn test_pattern_defaults() {
        let pattern = LocationPattern::default();
        assert!(pattern.detect_decimal);
        assert!(pattern.detect_places);
    }

    #[test]
    fn test_pattern_coordinates_only() {
        let pattern = LocationPattern::coordinates_only();
        assert!(pattern.detect_decimal);
        assert!(!pattern.detect_places);
    }
}
