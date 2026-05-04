//! OBINexus: Self-Aware Probing System
//!
//! A system that can introspect on itself through bidirectional probes:
//! - p(ext) : State → Data  [External Probe]
//! - p(int) : Data → State  [Internal Probe]

pub mod symbol_table;
pub mod execution_state;
pub mod probing;
pub mod dimensions;
pub mod temporal;
pub mod query_engine;
pub mod coherence;
pub mod ontology;
pub mod polygon;
pub mod aegis;

pub use symbol_table::{SymbolTable, Symbol, Binding};
pub use execution_state::ExecutionState;
pub use probing::{Probe, ProbeResponse, ProbeResult};
pub use dimensions::DimensionalSpace;
pub use temporal::TemporalHistory;
pub use query_engine::QueryEngine;
pub use coherence::CoherenceCheck;
pub use ontology::{OntologicalReasoner, OntologicalEntity, EntityType};
pub use polygon::{Schema, PolygonRegistry, SchemaField, FieldType};
pub use aegis::{AegisVerifier, ResourceType};

use std::sync::{Arc, Mutex};

/// OBINexus System - The core self-aware probing entity
pub struct OBINexus {
    /// Knowledge base: facts, patterns, learned associations
    symbol_table: Arc<Mutex<SymbolTable>>,

    /// Current execution state
    state: Arc<Mutex<ExecutionState>>,

    /// Problem space with O/D/A dimensions
    dimensions: Arc<Mutex<DimensionalSpace>>,

    /// Temporal history of state transitions and insights
    history: Arc<Mutex<TemporalHistory>>,

    /// Self-query engine (who, what, when, where, why, how)
    query_engine: QueryEngine,

    /// Coherence verification (95.4% standard for real-world systems)
    coherence: CoherenceCheck,

    /// Ontological reasoning system (entity classification & semantic understanding)
    ontology: Arc<Mutex<OntologicalReasoner>>,
}

impl OBINexus {
    /// Create a new OBINexus system
    pub fn new() -> Result<Self, String> {
        Ok(OBINexus {
            symbol_table: Arc::new(Mutex::new(SymbolTable::new())),
            state: Arc::new(Mutex::new(ExecutionState::new())),
            dimensions: Arc::new(Mutex::new(DimensionalSpace::new())),
            history: Arc::new(Mutex::new(TemporalHistory::new())),
            query_engine: QueryEngine::new(),
            coherence: CoherenceCheck::new(),
            ontology: Arc::new(Mutex::new(OntologicalReasoner::new())),
        })
    }

    /// External probe: State → Data
    /// Observes the current state and converts it to representable data
    pub fn probe_external(&self) -> Result<ProbeResult, String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        let symbols = self.symbol_table.lock().map_err(|e| e.to_string())?;

        Probe::external(&state, &symbols)
    }

    /// Internal probe: Data → State
    /// Processes data and updates the internal state
    pub fn probe_internal(&self, data: String) -> Result<ProbeResult, String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        let symbols = self.symbol_table.lock().map_err(|e| e.to_string())?;

        Probe::internal(data, &mut state, &symbols)
    }

    /// Ask a canonical question (who, what, when, where, why, how)
    pub fn ask(&self, question: &str) -> Result<String, String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        let symbols = self.symbol_table.lock().map_err(|e| e.to_string())?;
        let dims = self.dimensions.lock().map_err(|e| e.to_string())?;
        let hist = self.history.lock().map_err(|e| e.to_string())?;

        self.query_engine.answer(
            question,
            &state,
            &symbols,
            &dims,
            &hist,
        )
    }

    /// Learn a new fact into the knowledge base
    pub fn learn(&self, fact: &str, value: &str) -> Result<(), String> {
        let mut symbols = self.symbol_table.lock().map_err(|e| e.to_string())?;
        symbols.insert(fact.to_string(), value.to_string());

        // Record this in history
        if let Ok(mut hist) = self.history.lock() {
            hist.record_learning(fact.to_string(), value.to_string());
        }

        Ok(())
    }

    /// Get insight about why the system is in its current state
    pub fn self_reflect(&self) -> Result<String, String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        let hist = self.history.lock().map_err(|e| e.to_string())?;

        hist.reflection(&state)
    }

    /// Check system coherence (95.4% standard)
    pub fn check_coherence(&self) -> Result<f64, String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        let symbols = self.symbol_table.lock().map_err(|e| e.to_string())?;
        let dims = self.dimensions.lock().map_err(|e| e.to_string())?;

        self.coherence.evaluate(&state, &symbols, &dims)
    }

    /// Classify and register an entity in the ontology
    pub fn classify_entity(
        &self,
        id: String,
        name: String,
        properties: std::collections::HashMap<String, String>,
    ) -> Result<String, String> {
        let mut ontology = self.ontology.lock().map_err(|e| e.to_string())?;

        // Classify the entity
        let (entity_type, confidence) = ontology.classify(&name, &properties);

        // Create and register the entity
        let mut entity = OntologicalEntity::new(id.clone(), name, entity_type);
        entity.confidence = confidence;

        for (key, value) in properties {
            entity.add_property(key, value);
        }

        ontology.register_entity(entity)?;

        Ok(format!(
            "Classified as {} with {:.1}% confidence",
            entity_type,
            confidence * 100.0
        ))
    }

    /// Get ontological reasoning about an entity
    pub fn reason_about(&self, entity_id: &str) -> Result<String, String> {
        let ontology = self.ontology.lock().map_err(|e| e.to_string())?;

        if let Some(entity) = ontology.get_entity(entity_id) {
            let mut result = entity.describe();

            let conclusions = ontology.reason(entity_id);
            if !conclusions.is_empty() {
                result.push_str("Derived conclusions:\n");
                for conclusion in conclusions {
                    result.push_str(&format!("  - {}\n", conclusion));
                }
            }

            Ok(result)
        } else {
            Err(format!("Entity {} not found in ontology", entity_id))
        }
    }

    /// Export the ontological structure
    pub fn export_ontology(&self) -> Result<String, String> {
        let ontology = self.ontology.lock().map_err(|e| e.to_string())?;
        Ok(ontology.export())
    }
}

impl Default for OBINexus {
    fn default() -> Self {
        Self::new().expect("Failed to create OBINexus system")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_creation() {
        let system = OBINexus::new();
        assert!(system.is_ok());
    }

    #[test]
    fn test_learn_fact() {
        let system = OBINexus::new().unwrap();
        let result = system.learn("color", "blue");
        assert!(result.is_ok());
    }
}
