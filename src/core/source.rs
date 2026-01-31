//! Source reference types for tracking data provenance.

use crate::core::Timestamp;
use serde::{Deserialize, Serialize};

/// Type of source material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    /// News article or blog post.
    Article,
    /// Official report or document.
    Report,
    /// Witness account or testimony.
    Witness,
    /// Sensor or automated data.
    Sensor,
    /// Social media post.
    Social,
    /// Academic paper or research.
    Academic,
    /// Government or official record.
    Government,
    /// Archive or historical document.
    Archive,
    /// Other/unknown source type.
    Other,
}

impl Default for SourceType {
    fn default() -> Self {
        Self::Other
    }
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceType::Article => write!(f, "article"),
            SourceType::Report => write!(f, "report"),
            SourceType::Witness => write!(f, "witness"),
            SourceType::Sensor => write!(f, "sensor"),
            SourceType::Social => write!(f, "social"),
            SourceType::Academic => write!(f, "academic"),
            SourceType::Government => write!(f, "government"),
            SourceType::Archive => write!(f, "archive"),
            SourceType::Other => write!(f, "other"),
        }
    }
}

/// Reference to source material.
///
/// Sources track the provenance of event data, enabling
/// verification and attribution.
///
/// # Examples
///
/// ```
/// use spatial_narrative::core::{SourceRef, SourceType};
///
/// let source = SourceRef::article("https://news.example.com/story");
///
/// let detailed = SourceRef::builder()
///     .source_type(SourceType::Report)
///     .url("https://gov.example.com/report.pdf")
///     .title("Official Incident Report")
///     .author("Department of Safety")
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRef {
    /// Type of source.
    pub source_type: SourceType,
    /// URL to the source (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Title of the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Author or creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Publication or access date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,
    /// Additional notes about the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl SourceRef {
    /// Creates a new source reference with the given type.
    pub fn new(source_type: SourceType) -> Self {
        Self {
            source_type,
            url: None,
            title: None,
            author: None,
            date: None,
            notes: None,
        }
    }

    /// Creates an article source with a URL.
    pub fn article(url: impl Into<String>) -> Self {
        Self {
            source_type: SourceType::Article,
            url: Some(url.into()),
            title: None,
            author: None,
            date: None,
            notes: None,
        }
    }

    /// Creates a report source with a URL.
    pub fn report(url: impl Into<String>) -> Self {
        Self {
            source_type: SourceType::Report,
            url: Some(url.into()),
            title: None,
            author: None,
            date: None,
            notes: None,
        }
    }

    /// Creates a witness source with optional notes.
    pub fn witness(notes: Option<String>) -> Self {
        Self {
            source_type: SourceType::Witness,
            url: None,
            title: None,
            author: None,
            date: None,
            notes,
        }
    }

    /// Creates a sensor source with a URL.
    pub fn sensor(url: impl Into<String>) -> Self {
        Self {
            source_type: SourceType::Sensor,
            url: Some(url.into()),
            title: None,
            author: None,
            date: None,
            notes: None,
        }
    }

    /// Creates a builder for constructing a SourceRef.
    pub fn builder() -> SourceRefBuilder {
        SourceRefBuilder::new()
    }

    /// Sets the URL.
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the author.
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Sets the date.
    pub fn with_date(mut self, date: Timestamp) -> Self {
        self.date = Some(date);
        self
    }
}

impl Default for SourceRef {
    fn default() -> Self {
        Self::new(SourceType::Other)
    }
}

/// Builder for constructing [`SourceRef`] instances.
#[derive(Debug, Default)]
pub struct SourceRefBuilder {
    source_type: SourceType,
    url: Option<String>,
    title: Option<String>,
    author: Option<String>,
    date: Option<Timestamp>,
    notes: Option<String>,
}

impl SourceRefBuilder {
    /// Creates a new SourceRefBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the source type.
    pub fn source_type(mut self, source_type: SourceType) -> Self {
        self.source_type = source_type;
        self
    }

    /// Sets the URL.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the author.
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Sets the date.
    pub fn date(mut self, date: Timestamp) -> Self {
        self.date = Some(date);
        self
    }

    /// Sets the notes.
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Builds the SourceRef.
    pub fn build(self) -> SourceRef {
        SourceRef {
            source_type: self.source_type,
            url: self.url,
            title: self.title,
            author: self.author,
            date: self.date,
            notes: self.notes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_article() {
        let source = SourceRef::article("https://example.com/news");
        assert_eq!(source.source_type, SourceType::Article);
        assert_eq!(source.url, Some("https://example.com/news".to_string()));
    }

    #[test]
    fn test_source_builder() {
        let source = SourceRef::builder()
            .source_type(SourceType::Report)
            .url("https://example.com/report.pdf")
            .title("Annual Report")
            .author("Research Team")
            .build();

        assert_eq!(source.source_type, SourceType::Report);
        assert_eq!(source.title, Some("Annual Report".to_string()));
        assert_eq!(source.author, Some("Research Team".to_string()));
    }

    #[test]
    fn test_source_with_methods() {
        let source = SourceRef::new(SourceType::Academic)
            .with_url("https://journal.example.com/paper")
            .with_title("Research Paper")
            .with_author("Dr. Smith");

        assert_eq!(source.source_type, SourceType::Academic);
        assert!(source.url.is_some());
        assert!(source.title.is_some());
    }

    #[test]
    fn test_source_serialization() {
        let source = SourceRef::article("https://example.com");
        let json = serde_json::to_string(&source).unwrap();
        let parsed: SourceRef = serde_json::from_str(&json).unwrap();
        assert_eq!(source, parsed);
    }
}
