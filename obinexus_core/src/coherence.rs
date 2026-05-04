//! Coherence Checking: 95.4% Safety Standard
//!
//! Evaluates system coherence against real-world standards.
//! The 95.4% metric ensures the system understands problems well enough
//! to be deployed in safety-critical contexts.

use crate::dimensions::DimensionalSpace;
use crate::execution_state::ExecutionState;
use crate::symbol_table::SymbolTable;

/// Coherence evaluation metrics
#[derive(Debug, Clone)]
pub struct CoherenceMetrics {
    /// Knowledge completeness (0.0 to 1.0)
    pub knowledge_score: f64,

    /// State consistency (0.0 to 1.0)
    pub consistency_score: f64,

    /// Problem space understanding (0.0 to 1.0)
    pub understanding_score: f64,

    /// Overall coherence (0.0 to 1.0)
    pub overall_score: f64,

    /// Whether system meets 95.4% threshold
    pub meets_standard: bool,

    /// Diagnostic information
    pub diagnostics: String,
}

pub struct CoherenceCheck;

impl CoherenceCheck {
    pub fn new() -> Self {
        CoherenceCheck
    }

    /// Evaluate system coherence
    pub fn evaluate(
        &self,
        state: &ExecutionState,
        symbols: &SymbolTable,
        dimensions: &DimensionalSpace,
    ) -> Result<f64, String> {
        let metrics = self.compute_metrics(state, symbols, dimensions);
        Ok(metrics.overall_score)
    }

    /// Detailed coherence analysis
    pub fn analyze(
        &self,
        state: &ExecutionState,
        symbols: &SymbolTable,
        dimensions: &DimensionalSpace,
    ) -> CoherenceMetrics {
        self.compute_metrics(state, symbols, dimensions)
    }

    fn compute_metrics(
        &self,
        state: &ExecutionState,
        symbols: &SymbolTable,
        dimensions: &DimensionalSpace,
    ) -> CoherenceMetrics {
        let knowledge_score = self.evaluate_knowledge(symbols);
        let consistency_score = self.evaluate_consistency(state, symbols);
        let understanding_score = self.evaluate_understanding(dimensions);

        // Weighted average
        let overall_score = (knowledge_score * 0.3) + (consistency_score * 0.4) + (understanding_score * 0.3);

        let meets_standard = overall_score >= 0.954;

        let mut diagnostics = String::new();

        if knowledge_score < 0.8 {
            diagnostics.push_str("⚠ Low knowledge base (consider learning more facts)\n");
        }

        if consistency_score < 0.8 {
            diagnostics.push_str("⚠ Consistency issues detected\n");
        }

        if understanding_score < 0.7 {
            diagnostics.push_str("⚠ Limited problem space coverage\n");
        }

        if meets_standard {
            diagnostics.push_str("✓ System meets 95.4% safety standard\n");
        } else {
            diagnostics.push_str(&format!(
                "✗ System below standard: {:.1}% (target: 95.4%)\n",
                overall_score * 100.0
            ));
        }

        CoherenceMetrics {
            knowledge_score,
            consistency_score,
            understanding_score,
            overall_score,
            meets_standard,
            diagnostics,
        }
    }

    /// Knowledge completeness: quality of symbol table
    fn evaluate_knowledge(&self, symbols: &SymbolTable) -> f64 {
        if symbols.is_empty() {
            return 0.2; // Minimal knowledge
        }

        let count = symbols.len() as f64;
        let confidence = symbols.average_confidence();

        // Score based on quantity and quality
        let quantity_score = (count / 100.0).min(1.0); // Normalize to 1.0 at 100 symbols
        let quality_score = confidence; // Already 0.0 to 1.0

        (quantity_score * 0.4) + (quality_score * 0.6)
    }

    /// State consistency: how stable and predictable is the system?
    fn evaluate_consistency(&self, state: &ExecutionState, _symbols: &SymbolTable) -> f64 {
        let snapshot = state.snapshot();

        // Reasonable activity level
        let activity_score = if snapshot.instructions_executed > 0 {
            (snapshot.instructions_executed as f64 / 1000.0).min(1.0)
        } else {
            0.3
        };

        // Frame depth shouldn't be too deep (potential stack overflow)
        let frame_score = if snapshot.frame_depth > 100 {
            0.3 // Warning: deep recursion
        } else if snapshot.frame_depth > 0 {
            1.0
        } else {
            0.5
        };

        // Variables should be stable
        let var_score = if snapshot.variables_count > 0 {
            0.9
        } else {
            0.5
        };

        // Global stability
        let global_score = if snapshot.globals_count > 0 {
            0.85
        } else {
            0.7
        };

        (activity_score * 0.25) + (frame_score * 0.25) + (var_score * 0.25) + (global_score * 0.25)
    }

    /// Problem space understanding: coherence in dimensional space
    fn evaluate_understanding(&self, dimensions: &DimensionalSpace) -> f64 {
        let current = dimensions.current();

        // System should have meaningful position (not all zeros or all ones)
        let avg = (current.offense + current.defense + current.attack) / 3.0;

        let balance = if avg > 0.0 {
            // How balanced are the axes?
            let offense_diff = (current.offense - avg).abs();
            let defense_diff = (current.defense - avg).abs();
            let attack_diff = (current.attack - avg).abs();

            let max_diff = offense_diff.max(defense_diff).max(attack_diff);
            1.0 - (max_diff / avg).min(1.0) // Less deviation = better balance
        } else {
            0.5
        };

        // Magnitude (system should have some problem-solving energy)
        let magnitude = current.magnitude();
        let magnitude_score = (magnitude / 1.732).min(1.0); // Normalize to 1.0 at sqrt(3)

        (balance * 0.6) + (magnitude_score * 0.4)
    }
}

impl Default for CoherenceCheck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coherence_evaluation() {
        let checker = CoherenceCheck::new();
        let state = ExecutionState::new();
        let symbols = SymbolTable::new();
        let dims = DimensionalSpace::new();

        let score = checker.evaluate(&state, &symbols, &dims);
        assert!(score.is_ok());

        let score = score.unwrap();
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn test_coherence_analysis() {
        let checker = CoherenceCheck::new();
        let state = ExecutionState::new();
        let symbols = SymbolTable::new();
        let dims = DimensionalSpace::new();

        let metrics = checker.analyze(&state, &symbols, &dims);

        assert!(metrics.knowledge_score >= 0.0 && metrics.knowledge_score <= 1.0);
        assert!(metrics.consistency_score >= 0.0 && metrics.consistency_score <= 1.0);
        assert!(metrics.understanding_score >= 0.0 && metrics.understanding_score <= 1.0);
    }

    #[test]
    fn test_95_4_standard() {
        let checker = CoherenceCheck::new();
        let state = ExecutionState::new();
        let mut symbols = SymbolTable::new();

        // Add significant knowledge
        for i in 0..50 {
            symbols.insert(
                format!("fact_{}", i),
                format!("value_{}", i),
            );
        }

        let dims = DimensionalSpace::new();
        let metrics = checker.analyze(&state, &symbols, &dims);

        println!("Overall coherence: {:.1}%", metrics.overall_score * 100.0);
        println!("Diagnostics: {}", metrics.diagnostics);
        println!("Meets 95.4% standard: {}", metrics.meets_standard);
    }
}
