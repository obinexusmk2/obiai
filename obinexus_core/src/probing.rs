//! Probing System: Bidirectional State ↔ Data Transformation
//!
//! Implements the core probing mechanisms:
//! - p(ext) : State → Data  [External Probe]
//! - p(int) : Data → State  [Internal Probe]

use crate::execution_state::ExecutionState;
use crate::symbol_table::SymbolTable;
use serde::{Deserialize, Serialize};

/// Response to a probe query (three poles of consensus)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProbeResponse {
    /// Definite yes - clear state match
    Yes,
    /// Definite no - clear state mismatch
    No,
    /// Uncertain - requires more data/time
    Maybe,
}

impl std::fmt::Display for ProbeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProbeResponse::Yes => write!(f, "yes"),
            ProbeResponse::No => write!(f, "no"),
            ProbeResponse::Maybe => write!(f, "maybe"),
        }
    }
}

/// Result of a probing operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResult {
    /// The response (yes/no/maybe)
    pub response: ProbeResponse,

    /// Reason for the response
    pub reason: String,

    /// Data extracted/produced
    pub data: String,

    /// Confidence in the result (0.0 to 1.0)
    pub confidence: f64,

    /// Metadata about what was observed
    pub metadata: String,
}

impl std::fmt::Display for ProbeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ProbeResult {{ response: {}, confidence: {:.2}, reason: {} }}",
            self.response, self.confidence, self.reason
        )
    }
}

/// The Probe - conducts external and internal probing operations
pub struct Probe;

impl Probe {
    /// External Probe: State → Data
    ///
    /// Observes the current execution state and symbol knowledge,
    /// converting the internal state into representable, observable data.
    pub fn external(
        state: &ExecutionState,
        symbols: &SymbolTable,
    ) -> Result<ProbeResult, String> {
        let snapshot = state.snapshot();

        // Determine response based on state coherence
        let response = if state.instruction_count() > 0 {
            ProbeResponse::Yes // System has been active
        } else {
            ProbeResponse::Maybe // Initialization phase
        };

        // Extract observable data
        let variables = state.all_variables();
        let variables_json = serde_json::to_string(&variables).unwrap_or_default();

        let data = format!(
            "ExecutionState {{ depth: {}, context: '{}', variables: {}, symbol_count: {} }}",
            snapshot.frame_depth,
            snapshot.context,
            variables_json,
            symbols.len()
        );

        let confidence = if symbols.is_empty() {
            0.5 // Low confidence with no knowledge
        } else {
            symbols.average_confidence()
        };

        let metadata = format!(
            "Instructions: {}, Frames: {}, Globals: {}, KnowledgeBase: {}",
            snapshot.instructions_executed,
            snapshot.frame_depth,
            snapshot.globals_count,
            symbols.len()
        );

        Ok(ProbeResult {
            response,
            reason: "External probe observed system state".to_string(),
            data,
            confidence,
            metadata,
        })
    }

    /// Internal Probe: Data → State
    ///
    /// Takes input data and processes it to update the internal state.
    /// This is the introspection mechanism where the system learns and adapts.
    pub fn internal(
        data: String,
        state: &mut ExecutionState,
        symbols: &SymbolTable,
    ) -> Result<ProbeResult, String> {
        // Parse the input data for simple key=value pairs
        let mut response = ProbeResponse::Yes;
        let mut updates = Vec::new();

        for line in data.lines() {
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().to_string();

                // Check if this fact contradicts known symbols
                if let Some(existing) = symbols.get(&key) {
                    if existing.value != value {
                        response = ProbeResponse::Maybe; // Contradiction found
                    }
                }

                updates.push((key, value));
            }
        }

        // Apply updates to state
        for (key, value) in &updates {
            state.set_variable(
                key.clone(),
                crate::execution_state::Value::String(value.clone()),
            );
        }

        state.record_instruction("internal_probe".to_string());

        let metadata = format!(
            "Applied {} updates, Symbol table size: {}",
            updates.len(),
            symbols.len()
        );

        Ok(ProbeResult {
            response,
            reason: format!("Processed {} data assignments", updates.len()),
            data: serde_json::to_string(&updates).unwrap_or_default(),
            confidence: 0.8,
            metadata,
        })
    }

    /// Bidirectional consistency check
    ///
    /// Verifies that p(int) ∘ p(ext) maintains consistency
    /// (though may not be perfectly reversible)
    pub fn consistency_check(
        state: &ExecutionState,
        symbols: &SymbolTable,
    ) -> Result<f64, String> {
        // Extract state
        let external = Self::external(state, symbols)?;

        // Try to reprocess it
        let mut state_copy = ExecutionState::new();
        let internal = Self::internal(external.data.clone(), &mut state_copy, symbols)?;

        // Measure consistency: if we get the same response, consistency is high
        let consistency = if external.response == internal.response {
            0.95
        } else if external.response == ProbeResponse::Maybe
            || internal.response == ProbeResponse::Maybe
        {
            0.7
        } else {
            0.4
        };

        Ok(consistency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_external_probe() {
        let state = ExecutionState::new();
        let symbols = SymbolTable::new();

        let result = Probe::external(&state, &symbols);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.data.contains("ExecutionState"));
    }

    #[test]
    fn test_internal_probe() {
        let mut state = ExecutionState::new();
        let symbols = SymbolTable::new();

        let data = "name=OBINexus\nversion=0.1.0".to_string();
        let result = Probe::internal(data, &mut state, &symbols);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().response, ProbeResponse::Yes);
        assert_eq!(state.get_variable("name"), Some(crate::execution_state::Value::String("OBINexus".to_string())));
    }

    #[test]
    fn test_consistency_check() {
        let state = ExecutionState::new();
        let symbols = SymbolTable::new();

        let consistency = Probe::consistency_check(&state, &symbols);
        assert!(consistency.is_ok());
        let consistency_value = consistency.unwrap();
        assert!(consistency_value >= 0.0 && consistency_value <= 1.0);
    }
}
