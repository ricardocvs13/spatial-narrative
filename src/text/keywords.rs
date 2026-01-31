//! Keyword extraction using TF-IDF-like scoring.

use std::collections::{HashMap, HashSet};

use super::analyzer::TextAnalyzer;

/// An extracted keyword with relevance information.
#[derive(Debug, Clone)]
pub struct Keyword {
    /// The keyword or phrase
    pub text: String,
    /// Relevance score (higher = more relevant)
    pub score: f64,
    /// Number of occurrences in text
    pub frequency: usize,
}

impl Keyword {
    /// Create a new keyword.
    pub fn new(text: impl Into<String>, score: f64, frequency: usize) -> Self {
        Self {
            text: text.into(),
            score,
            frequency,
        }
    }
}

/// Keyword extractor using TF-IDF-like scoring.
///
/// Extracts the most relevant keywords from text based on
/// frequency and document structure.
///
/// # Example
///
/// ```rust
/// use spatial_narrative::text::KeywordExtractor;
///
/// let extractor = KeywordExtractor::new();
/// let text = "Climate change affects global weather patterns. \
///             Rising temperatures cause extreme weather events.";
///
/// let keywords = extractor.extract(text, 5);
/// for kw in &keywords {
///     println!("{}: {:.2}", kw.text, kw.score);
/// }
/// ```
pub struct KeywordExtractor {
    /// Stop words to filter
    stop_words: HashSet<String>,
    /// Minimum word length
    min_word_length: usize,
    /// Maximum phrase length (in words)
    max_phrase_length: usize,
}

impl KeywordExtractor {
    /// Create a new keyword extractor with default settings.
    pub fn new() -> Self {
        Self {
            stop_words: TextAnalyzer::default_stop_words(),
            min_word_length: 3,
            max_phrase_length: 3,
        }
    }

    /// Set minimum word length.
    pub fn min_length(mut self, length: usize) -> Self {
        self.min_word_length = length;
        self
    }

    /// Set maximum phrase length.
    pub fn max_phrase_length(mut self, length: usize) -> Self {
        self.max_phrase_length = length;
        self
    }

    /// Extract top N keywords from text.
    pub fn extract(&self, text: &str, n: usize) -> Vec<Keyword> {
        let mut word_freq: HashMap<String, usize> = HashMap::new();

        // Tokenize and count word frequencies
        let words: Vec<String> = text
            .split(|c: char| !c.is_alphanumeric() && c != '\'')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .filter(|w| w.len() >= self.min_word_length)
            .filter(|w| !self.stop_words.contains(w))
            .collect();

        for word in &words {
            *word_freq.entry(word.clone()).or_insert(0) += 1;
        }

        // Extract n-grams (phrases)
        if self.max_phrase_length > 1 {
            self.extract_ngrams(text, &mut word_freq);
        }

        // Calculate scores
        let total_words = words.len() as f64;
        let mut keywords: Vec<Keyword> = word_freq
            .into_iter()
            .map(|(word, freq)| {
                // Simple TF score (could be enhanced with IDF if we had a corpus)
                let tf = freq as f64 / total_words;
                // Boost longer words/phrases slightly
                let length_boost = 1.0 + (word.len() as f64 / 20.0);
                // Boost phrases (contain spaces)
                let phrase_boost = if word.contains(' ') { 1.5 } else { 1.0 };
                let score = tf * length_boost * phrase_boost;

                Keyword::new(word, score, freq)
            })
            .collect();

        // Sort by score (descending)
        keywords.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        // Return top N
        keywords.truncate(n);
        keywords
    }

    /// Extract keywords with custom stop words.
    pub fn extract_with_stopwords(
        &self,
        text: &str,
        n: usize,
        additional_stopwords: &[&str],
    ) -> Vec<Keyword> {
        let mut extractor = Self {
            stop_words: self.stop_words.clone(),
            min_word_length: self.min_word_length,
            max_phrase_length: self.max_phrase_length,
        };

        for word in additional_stopwords {
            extractor.stop_words.insert(word.to_lowercase());
        }

        extractor.extract(text, n)
    }

    fn extract_ngrams(&self, text: &str, word_freq: &mut HashMap<String, usize>) {
        // Split text into words properly
        let words: Vec<&str> = text
            .split(|c: char| !c.is_alphanumeric() && c != '\'')
            .filter(|s| !s.is_empty())
            .collect();

        // Extract bigrams and trigrams
        for n in 2..=self.max_phrase_length {
            for window in words.windows(n) {
                // Skip if any word is a stop word or too short
                let valid = window.iter().all(|w| {
                    w.len() >= self.min_word_length && !self.stop_words.contains(&w.to_lowercase())
                });

                if !valid {
                    continue;
                }

                let phrase_str = window
                    .iter()
                    .map(|w| w.to_lowercase())
                    .collect::<Vec<_>>()
                    .join(" ");
                *word_freq.entry(phrase_str).or_insert(0) += 1;
            }
        }
    }
}

impl Default for KeywordExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_extraction() {
        let extractor = KeywordExtractor::new().max_phrase_length(1); // Single words only
        let text = "Climate change affects global weather patterns. \
                    Rising temperatures cause extreme weather events. \
                    Climate scientists warn of more frequent storms.";

        let keywords = extractor.extract(text, 5);

        assert!(!keywords.is_empty());
        assert!(keywords.len() <= 5);

        // "climate" should be a top keyword (appears twice)
        assert!(keywords.iter().any(|k| k.text == "climate"));
    }

    #[test]
    fn test_keyword_extraction_empty() {
        let extractor = KeywordExtractor::new();
        let text = "the a an";

        let keywords = extractor.extract(text, 5);

        // All stop words, should return empty
        assert!(keywords.is_empty());
    }

    #[test]
    fn test_keyword_extraction_with_phrases() {
        let extractor = KeywordExtractor::new().max_phrase_length(2);
        let text = "global warming global warming climate change climate change";

        let keywords = extractor.extract(text, 10);

        // Should find phrases
        assert!(keywords.iter().any(|k| k.text.contains(' ')));
    }

    #[test]
    fn test_keyword_frequency() {
        let extractor = KeywordExtractor::new();
        let text = "test test test unique word";

        let keywords = extractor.extract(text, 5);

        // "test" appears 3 times
        let test_kw = keywords.iter().find(|k| k.text == "test");
        assert!(test_kw.is_some());
        assert_eq!(test_kw.unwrap().frequency, 3);
    }

    #[test]
    fn test_custom_stopwords() {
        let extractor = KeywordExtractor::new();
        let text = "important important important custom custom";

        let keywords = extractor.extract_with_stopwords(text, 5, &["important"]);

        // "important" should be filtered
        assert!(!keywords.iter().any(|k| k.text == "important"));
        // "custom" should remain
        assert!(keywords.iter().any(|k| k.text == "custom"));
    }

    #[test]
    fn test_min_word_length() {
        let extractor = KeywordExtractor::new().min_length(5);
        let text = "big cat runs fast through forest";

        let keywords = extractor.extract(text, 10);

        // Words shorter than 5 chars should be filtered
        assert!(!keywords.iter().any(|k| k.text == "big"));
        assert!(!keywords.iter().any(|k| k.text == "cat"));
        assert!(!keywords.iter().any(|k| k.text == "runs"));
        assert!(!keywords.iter().any(|k| k.text == "fast"));

        // "forest" (6 chars) should remain if not a stop word
        assert!(keywords.iter().any(|k| k.text == "forest"));
    }
}
