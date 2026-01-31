//! Named entity recognition (NER) using pattern-based detection.

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

use super::entity::{Entity, EntityType};

// Common title patterns for person detection
static TITLE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(Mr|Mrs|Ms|Dr|Prof|President|Chancellor|Prime Minister|King|Queen|Prince|Princess|Senator|Governor|Mayor|General|Admiral|Captain|Director|CEO|Chairman)\b\.?\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)").unwrap()
});

// Organization indicators
static ORG_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)\s+(Corporation|Corp|Inc|Ltd|LLC|Company|Co|Organization|Foundation|Institute|University|Agency|Department|Ministry|Commission|Council|Bank|Group|Association)\b").unwrap()
});

// Date patterns
static DATE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(\d{1,2}[/-]\d{1,2}[/-]\d{2,4}|\d{4}[/-]\d{1,2}[/-]\d{1,2}|(?:January|February|March|April|May|June|July|August|September|October|November|December)\s+\d{1,2},?\s+\d{4}|\d{1,2}\s+(?:January|February|March|April|May|June|July|August|September|October|November|December)\s+\d{4})\b").unwrap()
});

// Numeric patterns with units
static NUMERIC_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d+(?:,\d{3})*(?:\.\d+)?)\s*(km|miles?|meters?|feet|ft|pounds?|lbs?|kilograms?|kg|dollars?|\$|euros?|€|percent|%|people|casualties|deaths?|injured|wounded)\b").unwrap()
});

/// Text analyzer for named entity recognition.
///
/// This analyzer uses pattern-based recognition to identify
/// entities like people, organizations, locations, dates, etc.
///
/// # Example
///
/// ```rust
/// use spatial_narrative::text::TextAnalyzer;
///
/// let analyzer = TextAnalyzer::new();
/// let text = "Dr. Smith met with the World Health Organization in Geneva.";
///
/// let entities = analyzer.entities(text);
/// assert!(!entities.is_empty());
/// ```
pub struct TextAnalyzer {
    /// Known location names for detection
    known_locations: HashSet<String>,
    /// Stop words to filter
    stop_words: HashSet<String>,
}

impl TextAnalyzer {
    /// Create a new text analyzer with default settings.
    pub fn new() -> Self {
        Self {
            known_locations: Self::default_locations(),
            stop_words: Self::default_stop_words(),
        }
    }

    /// Create an analyzer with custom location names.
    pub fn with_locations(locations: HashSet<String>) -> Self {
        Self {
            known_locations: locations,
            stop_words: Self::default_stop_words(),
        }
    }

    /// Add a location name to recognize.
    pub fn add_location(&mut self, name: impl Into<String>) {
        self.known_locations.insert(name.into());
    }

    /// Extract named entities from text.
    pub fn entities(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();

        // Extract persons (with titles)
        self.extract_persons(text, &mut entities);

        // Extract organizations
        self.extract_organizations(text, &mut entities);

        // Extract dates
        self.extract_dates(text, &mut entities);

        // Extract numeric values
        self.extract_numerics(text, &mut entities);

        // Extract locations (from known list)
        self.extract_locations(text, &mut entities);

        // Sort by position and remove overlaps
        entities.sort_by_key(|e| e.start);
        self.remove_overlaps(&mut entities);

        entities
    }

    /// Tokenize text into words.
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        text.split(|c: char| !c.is_alphanumeric() && c != '\'')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    /// Tokenize and filter stop words.
    pub fn tokenize_filtered(&self, text: &str) -> Vec<String> {
        self.tokenize(text)
            .into_iter()
            .filter(|t| !self.stop_words.contains(&t.to_lowercase()))
            .collect()
    }

    /// Split text into sentences.
    pub fn sentences<'a>(&self, text: &'a str) -> Vec<&'a str> {
        // Simple sentence splitting on . ! ?
        text.split(['.', '!', '?'])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Get the default stop words set.
    pub fn default_stop_words() -> HashSet<String> {
        [
            "a", "an", "and", "are", "as", "at", "be", "by", "for", "from", "has", "he", "in",
            "is", "it", "its", "of", "on", "that", "the", "to", "was", "were", "will", "with",
            "the", "this", "but", "they", "have", "had", "what", "when", "where", "who", "which",
            "why", "how", "all", "each", "every", "both", "few", "more", "most", "other", "some",
            "such", "no", "nor", "not", "only", "own", "same", "so", "than", "too", "very", "can",
            "just", "should", "now", "also", "into", "could", "would", "there", "their", "been",
            "being", "having", "does", "did", "doing", "about", "above", "after", "before",
            "below", "between", "during", "under", "again", "further", "then", "once", "here",
            "any", "if", "or", "because", "until", "while", "through", "over", "up", "down",
            "out", "off", "her", "him", "his", "she", "your", "you", "we", "me", "my", "our",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    fn extract_persons(&self, text: &str, entities: &mut Vec<Entity>) {
        for cap in TITLE_PATTERN.captures_iter(text) {
            let full_match = cap.get(0).unwrap();
            let name = cap.get(2).unwrap().as_str();

            entities.push(
                Entity::new(name, EntityType::Person, full_match.start(), full_match.end())
                    .with_confidence(0.9),
            );
        }
    }

    fn extract_organizations(&self, text: &str, entities: &mut Vec<Entity>) {
        for cap in ORG_PATTERN.captures_iter(text) {
            let full_match = cap.get(0).unwrap();

            entities.push(
                Entity::new(
                    full_match.as_str(),
                    EntityType::Organization,
                    full_match.start(),
                    full_match.end(),
                )
                .with_confidence(0.85),
            );
        }

        // Also look for common organization names
        let common_orgs = [
            "United Nations",
            "UN",
            "NATO",
            "European Union",
            "EU",
            "World Health Organization",
            "WHO",
            "World Bank",
            "IMF",
            "FBI",
            "CIA",
            "NSA",
            "Pentagon",
            "White House",
            "Congress",
            "Parliament",
            "Red Cross",
            "Amnesty International",
            "Greenpeace",
            "UNICEF",
            "UNESCO",
        ];

        for org in common_orgs {
            let org_lower = org.to_lowercase();
            let text_lower = text.to_lowercase();

            let mut start = 0;
            while let Some(pos) = text_lower[start..].find(&org_lower) {
                let abs_pos = start + pos;
                let end_pos = abs_pos + org.len();

                // Check word boundaries
                let valid_start =
                    abs_pos == 0 || !text.chars().nth(abs_pos - 1).unwrap().is_alphanumeric();
                let valid_end = end_pos >= text.len()
                    || !text.chars().nth(end_pos).unwrap().is_alphanumeric();

                if valid_start && valid_end {
                    let overlaps = entities
                        .iter()
                        .any(|e| !(end_pos <= e.start || abs_pos >= e.end));

                    if !overlaps {
                        entities.push(
                            Entity::new(
                                &text[abs_pos..end_pos],
                                EntityType::Organization,
                                abs_pos,
                                end_pos,
                            )
                            .with_confidence(0.95),
                        );
                    }
                }

                start = abs_pos + 1;
            }
        }
    }

    fn extract_dates(&self, text: &str, entities: &mut Vec<Entity>) {
        for cap in DATE_PATTERN.captures_iter(text) {
            let full_match = cap.get(0).unwrap();

            entities.push(
                Entity::new(
                    full_match.as_str(),
                    EntityType::DateTime,
                    full_match.start(),
                    full_match.end(),
                )
                .with_confidence(0.95),
            );
        }
    }

    fn extract_numerics(&self, text: &str, entities: &mut Vec<Entity>) {
        for cap in NUMERIC_PATTERN.captures_iter(text) {
            let full_match = cap.get(0).unwrap();

            entities.push(
                Entity::new(
                    full_match.as_str(),
                    EntityType::Numeric,
                    full_match.start(),
                    full_match.end(),
                )
                .with_confidence(0.9),
            );
        }
    }

    fn extract_locations(&self, text: &str, entities: &mut Vec<Entity>) {
        let text_lower = text.to_lowercase();

        // Sort locations by length (longest first)
        let mut locations: Vec<_> = self.known_locations.iter().collect();
        locations.sort_by_key(|b| std::cmp::Reverse(b.len()));

        for location in locations {
            let loc_lower = location.to_lowercase();

            let mut start = 0;
            while let Some(pos) = text_lower[start..].find(&loc_lower) {
                let abs_pos = start + pos;
                let end_pos = abs_pos + location.len();

                // Check word boundaries
                let valid_start =
                    abs_pos == 0 || !text.chars().nth(abs_pos - 1).unwrap().is_alphanumeric();
                let valid_end = end_pos >= text.len()
                    || !text.chars().nth(end_pos).unwrap().is_alphanumeric();

                if valid_start && valid_end {
                    let overlaps = entities
                        .iter()
                        .any(|e| !(end_pos <= e.start || abs_pos >= e.end));

                    if !overlaps {
                        entities.push(
                            Entity::new(
                                &text[abs_pos..end_pos],
                                EntityType::Location,
                                abs_pos,
                                end_pos,
                            )
                            .with_confidence(0.85),
                        );
                    }
                }

                start = abs_pos + 1;
            }
        }
    }

    fn remove_overlaps(&self, entities: &mut Vec<Entity>) {
        if entities.len() <= 1 {
            return;
        }

        entities.sort_by(|a, b| {
            a.start
                .cmp(&b.start)
                .then(b.confidence.partial_cmp(&a.confidence).unwrap())
        });

        let mut result = Vec::new();
        let mut last_end = 0;

        for entity in entities.drain(..) {
            if entity.start >= last_end {
                last_end = entity.end;
                result.push(entity);
            }
        }

        *entities = result;
    }

    fn default_locations() -> HashSet<String> {
        [
            // Major cities
            "New York",
            "Los Angeles",
            "Chicago",
            "Houston",
            "Phoenix",
            "Philadelphia",
            "San Antonio",
            "San Diego",
            "Dallas",
            "San Francisco",
            "Seattle",
            "Boston",
            "Washington",
            "Miami",
            "Denver",
            "Atlanta",
            "London",
            "Paris",
            "Berlin",
            "Madrid",
            "Rome",
            "Amsterdam",
            "Vienna",
            "Brussels",
            "Stockholm",
            "Oslo",
            "Copenhagen",
            "Helsinki",
            "Dublin",
            "Lisbon",
            "Athens",
            "Warsaw",
            "Prague",
            "Budapest",
            "Bucharest",
            "Kyiv",
            "Kiev",
            "Moscow",
            "Istanbul",
            "Tokyo",
            "Beijing",
            "Shanghai",
            "Hong Kong",
            "Singapore",
            "Seoul",
            "Mumbai",
            "Delhi",
            "Bangkok",
            "Jakarta",
            "Manila",
            "Sydney",
            "Melbourne",
            "Cairo",
            "Lagos",
            "Johannesburg",
            "Toronto",
            "Vancouver",
            "Montreal",
            "Mexico City",
            "São Paulo",
            "Rio de Janeiro",
            "Buenos Aires",
            "Lima",
            "Bogotá",
            "Santiago",
            "Geneva",
            "Zurich",
            "Munich",
            "Milan",
            "Barcelona",
            "Dubai",
            "Tel Aviv",
            "Jerusalem",
            // Countries
            "United States",
            "USA",
            "America",
            "United Kingdom",
            "UK",
            "Britain",
            "France",
            "Germany",
            "Italy",
            "Spain",
            "China",
            "Japan",
            "India",
            "Brazil",
            "Australia",
            "Russia",
            "Canada",
            "Mexico",
            "Ukraine",
            "Poland",
            "Netherlands",
            "Belgium",
            "Sweden",
            "Norway",
            "Denmark",
            "Finland",
            "Switzerland",
            "Austria",
            "Portugal",
            "Greece",
            "Turkey",
            "Israel",
            "Egypt",
            "South Africa",
            "Nigeria",
            "Kenya",
            "Argentina",
            "Colombia",
            "Peru",
            "Chile",
            "Indonesia",
            "Philippines",
            "Thailand",
            "Vietnam",
            "Malaysia",
            "Taiwan",
            "Iran",
            "Iraq",
            "Syria",
            "Afghanistan",
            "Pakistan",
            "Bangladesh",
            "Saudi Arabia",
            "UAE",
            "Qatar",
            // Regions
            "Europe",
            "Asia",
            "Africa",
            "North America",
            "South America",
            "Middle East",
            "Central Asia",
            "Southeast Asia",
            "Eastern Europe",
            "Western Europe",
            "Pacific",
            "Atlantic",
            "Mediterranean",
            "Caribbean",
            "Baltic",
            "Balkans",
            "Scandinavia",
            "Siberia",
            "Arctic",
            "Antarctic",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }
}

impl Default for TextAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_extraction_persons() {
        let analyzer = TextAnalyzer::new();
        let text = "President Biden announced new policies. Dr. Smith provided commentary.";

        let entities = analyzer.entities(text);

        assert!(entities.iter().any(|e| e.text.contains("Biden")));
        assert!(entities.iter().any(|e| e.text.contains("Smith")));
    }

    #[test]
    fn test_entity_extraction_organizations() {
        let analyzer = TextAnalyzer::new();
        let text = "The United Nations met to discuss the crisis. WHO issued guidelines.";

        let entities = analyzer.entities(text);

        assert!(entities
            .iter()
            .any(|e| e.entity_type == EntityType::Organization));
    }

    #[test]
    fn test_entity_extraction_locations() {
        let analyzer = TextAnalyzer::new();
        let text = "The summit was held in Geneva. Delegates came from Berlin and Tokyo.";

        let entities = analyzer.entities(text);

        let locations: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Location)
            .collect();

        assert!(locations.iter().any(|e| e.text == "Geneva"));
        assert!(locations.iter().any(|e| e.text == "Berlin"));
        assert!(locations.iter().any(|e| e.text == "Tokyo"));
    }

    #[test]
    fn test_entity_extraction_dates() {
        let analyzer = TextAnalyzer::new();
        let text = "The event occurred on January 15, 2024. Another event was on 2023-12-01.";

        let entities = analyzer.entities(text);

        let dates: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::DateTime)
            .collect();

        assert!(!dates.is_empty());
    }

    #[test]
    fn test_entity_extraction_numerics() {
        let analyzer = TextAnalyzer::new();
        let text = "The earthquake killed 50 people and injured 200. Damage estimated at $5 million.";

        let entities = analyzer.entities(text);

        let numerics: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Numeric)
            .collect();

        assert!(!numerics.is_empty());
    }

    #[test]
    fn test_tokenization() {
        let analyzer = TextAnalyzer::new();
        let tokens = analyzer.tokenize("Hello, world! This is a test.");

        assert!(tokens.contains(&"Hello".to_string()));
        assert!(tokens.contains(&"world".to_string()));
        assert!(tokens.contains(&"This".to_string()));
        assert!(tokens.contains(&"test".to_string()));
    }

    #[test]
    fn test_tokenization_filtered() {
        let analyzer = TextAnalyzer::new();
        let tokens = analyzer.tokenize_filtered("The quick brown fox jumps over the lazy dog.");

        // Stop words should be filtered
        assert!(!tokens.iter().any(|t| t.to_lowercase() == "the"));
        assert!(!tokens.iter().any(|t| t.to_lowercase() == "over"));

        // Content words should remain
        assert!(tokens.iter().any(|t| t.to_lowercase() == "quick"));
        assert!(tokens.iter().any(|t| t.to_lowercase() == "brown"));
    }

    #[test]
    fn test_sentences() {
        let analyzer = TextAnalyzer::new();
        let text = "First sentence. Second sentence! Third sentence?";

        let sentences = analyzer.sentences(text);

        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[0], "First sentence");
        assert_eq!(sentences[1], "Second sentence");
        assert_eq!(sentences[2], "Third sentence");
    }

    #[test]
    fn test_analyzer_add_location() {
        let mut analyzer = TextAnalyzer::new();
        analyzer.add_location("CustomCity");

        let text = "Events occurred in CustomCity today.";
        let entities = analyzer.entities(text);

        assert!(entities
            .iter()
            .any(|e| e.text == "CustomCity" && e.entity_type == EntityType::Location));
    }
}
