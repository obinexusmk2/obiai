//! Ontological Reasoning: Entity Classification & Semantic Understanding
//!
//! Implements ontological categories for semantic reasoning about entities,
//! with support for:
//! - Entity classification (Object, Agent, Abstract, Process)
//! - Property hierarchies
//! - Semantic relationships
//! - Semiotic understanding (Nsibidi-aware)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Entity types in the ontology
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityType {
    /// Physical objects (matter, things)
    Object,
    /// Agents with agency (actors, entities with goals)
    Agent,
    /// Abstract concepts (ideas, numbers, relations)
    Abstract,
    /// Processes and events (actions, changes)
    Process,
    /// States and conditions (qualities, properties)
    State,
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::Object => write!(f, "Object"),
            EntityType::Agent => write!(f, "Agent"),
            EntityType::Abstract => write!(f, "Abstract"),
            EntityType::Process => write!(f, "Process"),
            EntityType::State => write!(f, "State"),
        }
    }
}

/// Ontological classification of an entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologicalEntity {
    /// Entity identifier
    pub id: String,

    /// Human-readable name
    pub name: String,

    /// Primary entity type
    pub entity_type: EntityType,

    /// Parent category (if any)
    pub parent: Option<String>,

    /// Properties and their values
    pub properties: HashMap<String, String>,

    /// Semantic relationships to other entities
    pub relationships: HashMap<String, Vec<String>>,

    /// Confidence in classification (0.0 to 1.0)
    pub confidence: f64,

    /// Semiotic layer (symbolic meaning)
    pub semiotic_symbol: Option<String>,
}

impl OntologicalEntity {
    pub fn new(id: String, name: String, entity_type: EntityType) -> Self {
        OntologicalEntity {
            id,
            name,
            entity_type,
            parent: None,
            properties: HashMap::new(),
            relationships: HashMap::new(),
            confidence: 1.0,
            semiotic_symbol: None,
        }
    }

    /// Add a property to this entity
    pub fn add_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }

    /// Add a relationship to another entity
    pub fn add_relationship(&mut self, rel_type: String, entity_id: String) {
        self.relationships
            .entry(rel_type)
            .or_insert_with(Vec::new)
            .push(entity_id);
    }

    /// Set the semiotic symbol (Nsibidi-aware)
    pub fn set_semiotic_symbol(&mut self, symbol: String) {
        self.semiotic_symbol = Some(symbol);
    }

    /// Describe this entity
    pub fn describe(&self) -> String {
        let mut desc = format!("{} ({})\n", self.name, self.entity_type);

        if !self.properties.is_empty() {
            desc.push_str("Properties:\n");
            for (key, val) in &self.properties {
                desc.push_str(&format!("  - {}: {}\n", key, val));
            }
        }

        if !self.relationships.is_empty() {
            desc.push_str("Relationships:\n");
            for (rel_type, entities) in &self.relationships {
                desc.push_str(&format!("  {} -> {:?}\n", rel_type, entities));
            }
        }

        if let Some(symbol) = &self.semiotic_symbol {
            desc.push_str(&format!("Symbol: {}\n", symbol));
        }

        desc
    }
}

/// Ontological reasoning system
pub struct OntologicalReasoner {
    /// All entities in the ontology
    entities: HashMap<String, OntologicalEntity>,

    /// Category hierarchy
    categories: HashMap<EntityType, Vec<String>>,

    /// Semantic rules (if X then Y)
    rules: Vec<(String, String)>,

    /// Confidence threshold for reasoning
    confidence_threshold: f64,
}

impl OntologicalReasoner {
    pub fn new() -> Self {
        OntologicalReasoner {
            entities: HashMap::new(),
            categories: HashMap::new(),
            rules: Vec::new(),
            confidence_threshold: 0.7,
        }
    }

    /// Register an entity in the ontology
    pub fn register_entity(&mut self, entity: OntologicalEntity) -> Result<(), String> {
        let entity_type = entity.entity_type;
        let entity_id = entity.id.clone();

        self.entities.insert(entity_id.clone(), entity);

        self.categories
            .entry(entity_type)
            .or_insert_with(Vec::new)
            .push(entity_id);

        Ok(())
    }

    /// Get an entity by ID
    pub fn get_entity(&self, id: &str) -> Option<&OntologicalEntity> {
        self.entities.get(id)
    }

    /// Get all entities of a type
    pub fn entities_of_type(&self, entity_type: EntityType) -> Vec<&OntologicalEntity> {
        self.categories
            .get(&entity_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.entities.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Add a semantic rule (if premise then conclusion)
    pub fn add_rule(&mut self, premise: String, conclusion: String) {
        self.rules.push((premise, conclusion));
    }

    /// Apply ontological reasoning to derive new facts
    pub fn reason(&self, entity_id: &str) -> Vec<String> {
        let mut conclusions = Vec::new();

        if let Some(entity) = self.get_entity(entity_id) {
            for (premise, conclusion) in &self.rules {
                // Simple rule matching based on entity type
                if entity.entity_type.to_string().contains(premise)
                    || entity.properties.values().any(|v| v.contains(premise))
                {
                    if entity.confidence >= self.confidence_threshold {
                        conclusions.push(conclusion.clone());
                    }
                }
            }
        }

        conclusions
    }

    /// Find relationships between entities
    pub fn find_relationships(&self, entity_id: &str) -> HashMap<String, Vec<String>> {
        self.get_entity(entity_id)
            .map(|e| e.relationships.clone())
            .unwrap_or_default()
    }

    /// Classify an unknown entity based on properties
    pub fn classify(
        &self,
        name: &str,
        properties: &HashMap<String, String>,
    ) -> (EntityType, f64) {
        // Simple heuristic classification
        let mut scores = HashMap::new();

        // Check for agent indicators
        if properties
            .values()
            .any(|v| v.to_lowercase().contains("agent") || v.to_lowercase().contains("actor"))
        {
            *scores.entry(EntityType::Agent).or_insert(0.0) += 0.5;
        }

        // Check for object indicators
        if properties.values().any(|v| {
            v.to_lowercase().contains("physical")
                || v.to_lowercase().contains("material")
                || v.to_lowercase().contains("tangible")
        }) {
            *scores.entry(EntityType::Object).or_insert(0.0) += 0.5;
        }

        // Check for abstract indicators
        if properties
            .values()
            .any(|v| v.to_lowercase().contains("concept") || v.to_lowercase().contains("idea"))
        {
            *scores.entry(EntityType::Abstract).or_insert(0.0) += 0.5;
        }

        // Check for process indicators
        if properties.values().any(|v| {
            v.to_lowercase().contains("action")
                || v.to_lowercase().contains("process")
                || v.to_lowercase().contains("event")
        }) {
            *scores.entry(EntityType::Process).or_insert(0.0) += 0.5;
        }

        // Get highest score
        let (best_type, score): (EntityType, f64) = scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(t, s)| (*t, *s))
            .unwrap_or((EntityType::Abstract, 0.3));

        (best_type, score.min(1.0))
    }

    /// Get all entities with a specific property
    pub fn query_by_property(&self, key: &str, value: &str) -> Vec<&OntologicalEntity> {
        self.entities
            .values()
            .filter(|e| {
                e.properties
                    .get(key)
                    .map(|v| v.contains(value))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Export ontology as text
    pub fn export(&self) -> String {
        let mut output = String::from("=== Ontological Reasoning System ===\n\n");

        for (entity_type, ids) in &self.categories {
            output.push_str(&format!("{}\n", entity_type));
            output.push_str(&"─".repeat(40));
            output.push('\n');

            for id in ids {
                if let Some(entity) = self.entities.get(id) {
                    output.push_str(&entity.describe());
                    output.push('\n');
                }
            }

            output.push('\n');
        }

        if !self.rules.is_empty() {
            output.push_str("Rules:\n");
            for (premise, conclusion) in &self.rules {
                output.push_str(&format!("  {} => {}\n", premise, conclusion));
            }
        }

        output
    }
}

impl Default for OntologicalReasoner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let entity = OntologicalEntity::new(
            "e1".to_string(),
            "Human".to_string(),
            EntityType::Agent,
        );

        assert_eq!(entity.entity_type, EntityType::Agent);
        assert_eq!(entity.confidence, 1.0);
    }

    #[test]
    fn test_ontology_registration() {
        let mut reasoner = OntologicalReasoner::new();
        let entity = OntologicalEntity::new(
            "e1".to_string(),
            "Book".to_string(),
            EntityType::Object,
        );

        assert!(reasoner.register_entity(entity).is_ok());
        assert!(reasoner.get_entity("e1").is_some());
    }

    #[test]
    fn test_classification() {
        let reasoner = OntologicalReasoner::new();
        let mut props = HashMap::new();
        props.insert("type".to_string(), "physical object".to_string());

        let (entity_type, confidence) = reasoner.classify("Thing", &props);
        assert_eq!(entity_type, EntityType::Object);
        assert!(confidence > 0.0);
    }

    #[test]
    fn test_reasoning_rules() {
        let mut reasoner = OntologicalReasoner::new();
        reasoner.add_rule(
            "Agent".to_string(),
            "Can perform actions".to_string(),
        );

        let mut entity = OntologicalEntity::new(
            "e1".to_string(),
            "Person".to_string(),
            EntityType::Agent,
        );
        entity.confidence = 0.9;

        reasoner.register_entity(entity).unwrap();

        let conclusions = reasoner.reason("e1");
        assert!(!conclusions.is_empty());
    }
}
