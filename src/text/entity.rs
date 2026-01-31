//! Named entity types for text analysis.

/// Type of named entity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityType {
    /// Person name
    Person,
    /// Organization or company
    Organization,
    /// Geographic location
    Location,
    /// Date or time reference
    DateTime,
    /// Numeric value or measurement
    Numeric,
    /// Event name
    Event,
    /// Unknown/other entity type
    Other,
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::Person => write!(f, "PERSON"),
            EntityType::Organization => write!(f, "ORG"),
            EntityType::Location => write!(f, "LOC"),
            EntityType::DateTime => write!(f, "DATE"),
            EntityType::Numeric => write!(f, "NUM"),
            EntityType::Event => write!(f, "EVENT"),
            EntityType::Other => write!(f, "OTHER"),
        }
    }
}

/// A named entity extracted from text.
#[derive(Debug, Clone)]
pub struct Entity {
    /// The entity text
    pub text: String,
    /// Type of entity
    pub entity_type: EntityType,
    /// Start position in source text (byte offset)
    pub start: usize,
    /// End position in source text (byte offset)
    pub end: usize,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
}

impl Entity {
    /// Create a new entity.
    pub fn new(text: impl Into<String>, entity_type: EntityType, start: usize, end: usize) -> Self {
        Self {
            text: text.into(),
            entity_type,
            start,
            end,
            confidence: 1.0,
        }
    }

    /// Set the confidence score.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_type_display() {
        assert_eq!(EntityType::Person.to_string(), "PERSON");
        assert_eq!(EntityType::Organization.to_string(), "ORG");
        assert_eq!(EntityType::Location.to_string(), "LOC");
        assert_eq!(EntityType::DateTime.to_string(), "DATE");
    }

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new("Berlin", EntityType::Location, 0, 6);
        assert_eq!(entity.text, "Berlin");
        assert_eq!(entity.entity_type, EntityType::Location);
        assert_eq!(entity.confidence, 1.0);
    }

    #[test]
    fn test_entity_with_confidence() {
        let entity = Entity::new("Berlin", EntityType::Location, 0, 6).with_confidence(0.85);
        assert_eq!(entity.confidence, 0.85);
    }

    #[test]
    fn test_entity_confidence_clamping() {
        let entity = Entity::new("Test", EntityType::Other, 0, 4).with_confidence(1.5);
        assert_eq!(entity.confidence, 1.0);

        let entity = Entity::new("Test", EntityType::Other, 0, 4).with_confidence(-0.5);
        assert_eq!(entity.confidence, 0.0);
    }
}
