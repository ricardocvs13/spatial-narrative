//! Natural language processing utilities.
//!
//! This module provides text analysis tools for working with
//! narrative text, including entity extraction and keyword analysis.
//!
//! # Overview
//!
//! The text module includes:
//!
//! - [`TextAnalyzer`] - Named entity recognition for locations, organizations, etc.
//! - [`KeywordExtractor`] - Extract keywords and key phrases from text
//! - [`Entity`] - A detected named entity with type and span info
//! - [`Keyword`] - An extracted keyword with relevance score
//!
//! # Examples
//!
//! ## Extracting Named Entities
//!
//! ```rust
//! use spatial_narrative::text::{TextAnalyzer, EntityType};
//!
//! let mut analyzer = TextAnalyzer::new();
//! analyzer.add_location("Berlin");
//! let text = "The meeting was held in Berlin.";
//!
//! let entities = analyzer.entities(text);
//! assert!(entities.iter().any(|e| e.text == "Berlin"));
//! ```
//!
//! ## Extracting Keywords
//!
//! ```rust
//! use spatial_narrative::text::KeywordExtractor;
//!
//! let extractor = KeywordExtractor::new();
//! let text = "The earthquake caused significant damage to infrastructure. \
//!             Emergency responders worked through the night.";
//!
//! let keywords = extractor.extract(text, 5);
//! assert!(!keywords.is_empty());
//! ```
//!
//! ## Tokenization
//!
//! ```rust
//! use spatial_narrative::text::TextAnalyzer;
//!
//! let analyzer = TextAnalyzer::new();
//! let tokens = analyzer.tokenize("Hello, world! This is a test.");
//!
//! assert!(tokens.contains(&"Hello".to_string()));
//! assert!(tokens.contains(&"world".to_string()));
//! ```

mod analyzer;
mod entity;
mod keywords;

pub use analyzer::TextAnalyzer;
pub use entity::{Entity, EntityType};
pub use keywords::{Keyword, KeywordExtractor};
