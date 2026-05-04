//! Self-Query Engine: Canonical Questions
//!
//! Implements the core questions a system can ask itself:
//! - Who (identity/ownership)
//! - What (nature/description)
//! - When (temporal context)
//! - Where (position in problem space)
//! - Why (reason/causality)
//! - How (mechanism/method)

use crate::dimensions::DimensionalSpace;
use crate::execution_state::ExecutionState;
use crate::symbol_table::SymbolTable;
use crate::temporal::TemporalHistory;

/// Query types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    Who,
    What,
    When,
    Where,
    Why,
    How,
}

impl QueryType {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().trim() {
            "who" => Some(QueryType::Who),
            "what" => Some(QueryType::What),
            "when" => Some(QueryType::When),
            "where" => Some(QueryType::Where),
            "why" => Some(QueryType::Why),
            "how" => Some(QueryType::How),
            _ => None,
        }
    }
}

pub struct QueryEngine;

impl QueryEngine {
    pub fn new() -> Self {
        QueryEngine
    }

    pub fn answer(
        &self,
        question: &str,
        state: &ExecutionState,
        symbols: &SymbolTable,
        dimensions: &DimensionalSpace,
        history: &TemporalHistory,
    ) -> Result<String, String> {
        // Parse the question
        let q_type = QueryType::from_string(question)
            .ok_or_else(|| "Question must be one of: who, what, when, where, why, how".to_string())?;

        match q_type {
            QueryType::Who => self.answer_who(state, symbols),
            QueryType::What => self.answer_what(state, symbols, history),
            QueryType::When => self.answer_when(history),
            QueryType::Where => self.answer_where(dimensions),
            QueryType::Why => self.answer_why(history, dimensions),
            QueryType::How => self.answer_how(state, symbols, history),
        }
    }

    /// Who: System identity and ownership
    fn answer_who(&self, state: &ExecutionState, symbols: &SymbolTable) -> Result<String, String> {
        let mut answer = String::from("Who I am:\n");

        // Check for identity symbol
        if let Some(identity) = symbols.get("identity") {
            answer.push_str(&format!("  Identity: {}\n", identity.value));
        } else {
            answer.push_str("  Identity: OBINexus - Self-aware probing system\n");
        }

        // Check for name
        if let Some(name) = symbols.get("name") {
            answer.push_str(&format!("  Name: {}\n", name.value));
        }

        // Current frame indicates current role/context
        let snapshot = state.snapshot();
        answer.push_str(&format!(
            "  Current context: {}\n",
            snapshot.current_frame_name
        ));

        // Ownership based on knowledge
        answer.push_str(&format!(
            "  Knowledge base size: {} symbols\n",
            symbols.len()
        ));

        Ok(answer)
    }

    /// What: Nature and description of current state
    fn answer_what(
        &self,
        state: &ExecutionState,
        _symbols: &SymbolTable,
        history: &TemporalHistory,
    ) -> Result<String, String> {
        let mut answer = String::from("What I am:\n");

        let snapshot = state.snapshot();

        answer.push_str(&format!(
            "  Active system with {} instructions executed\n",
            snapshot.instructions_executed
        ));

        answer.push_str(&format!(
            "  Currently in frame: {}\n",
            snapshot.current_frame_name
        ));

        answer.push_str(&format!(
            "  Variables in scope: {}\n",
            snapshot.variables_count
        ));

        answer.push_str(&format!(
            "  Global symbols: {}\n",
            snapshot.globals_count
        ));

        // What have I learned?
        let learnings = history.learnings();
        answer.push_str(&format!(
            "  Facts learned: {}\n",
            learnings.len()
        ));

        // State type
        if state.instruction_count() > 1000 {
            answer.push_str("  Status: Mature system with extensive experience\n");
        } else if state.instruction_count() > 0 {
            answer.push_str("  Status: Active learning system\n");
        } else {
            answer.push_str("  Status: Initialized, awaiting input\n");
        }

        Ok(answer)
    }

    /// When: Temporal context
    fn answer_when(&self, history: &TemporalHistory) -> Result<String, String> {
        let mut answer = String::from("When:\n");

        let events = history.all_events();

        if events.is_empty() {
            answer.push_str("  No temporal history yet\n");
            return Ok(answer);
        }

        // First event
        if let Some(first) = events.first() {
            answer.push_str(&format!(
                "  System started at timestamp: {}\n",
                first.timestamp
            ));
        }

        // Last event
        if let Some(last) = events.last() {
            answer.push_str(&format!(
                "  Last activity at timestamp: {}\n",
                last.timestamp
            ));
        }

        // Count events by type
        answer.push_str(&format!("  Total events recorded: {}\n", events.len()));

        // Recent activity
        let recent = history.events_since(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .saturating_sub(3600), // Last hour
        );

        answer.push_str(&format!(
            "  Recent activity (last hour): {} events\n",
            recent.len()
        ));

        Ok(answer)
    }

    /// Where: Position in problem space
    fn answer_where(&self, dimensions: &DimensionalSpace) -> Result<String, String> {
        let mut answer = String::from("Where I am in problem space:\n");

        let current = dimensions.current();

        answer.push_str(&format!(
            "  Offense position: {:.2}\n",
            current.offense
        ));
        answer.push_str(&format!(
            "  Defense position: {:.2}\n",
            current.defense
        ));
        answer.push_str(&format!(
            "  Attack position: {:.2}\n",
            current.attack
        ));

        answer.push_str(&format!(
            "  Magnitude (total strength): {:.2}\n",
            current.magnitude()
        ));

        // Characterize the position
        let avg = (current.offense + current.defense + current.attack) / 3.0;
        let characteristic = if current.offense > avg {
            "Offensive-biased"
        } else if current.defense > avg {
            "Defense-focused"
        } else if current.attack > avg {
            "Challenging-oriented"
        } else {
            "Balanced"
        };

        answer.push_str(&format!(
            "  Characteristic strategy: {}\n",
            characteristic
        ));

        Ok(answer)
    }

    /// Why: Reason and causality
    fn answer_why(
        &self,
        history: &TemporalHistory,
        _dimensions: &DimensionalSpace,
    ) -> Result<String, String> {
        let mut answer = String::from("Why I am as I am:\n");

        let flashes = history.flashes();
        if !flashes.is_empty() {
            answer.push_str(&format!(
                "  Key insights ({}):\n",
                flashes.len()
            ));

            for (i, flash) in flashes.iter().take(5).enumerate() {
                answer.push_str(&format!(
                    "    {}. {} ({})\n",
                    i + 1,
                    flash.description,
                    flash.context
                ));
            }
        }

        let filters = history.filters();
        if !filters.is_empty() {
            answer.push_str(&format!(
                "  Issues resolved ({}):\n",
                filters.len()
            ));

            for issue in filters.iter().take(3) {
                answer.push_str(&format!(
                    "    - {} ({})\n",
                    issue.description,
                    issue.context
                ));
            }
        }

        // Causality: learnings lead to insights
        let learnings = history.learnings();
        answer.push_str(&format!(
            "  Knowledge accumulated: {} facts learned\n",
            learnings.len()
        ));

        answer.push_str("  Reason: Continuous introspection and learning\n");

        Ok(answer)
    }

    /// How: Mechanism and method
    fn answer_how(
        &self,
        state: &ExecutionState,
        symbols: &SymbolTable,
        history: &TemporalHistory,
    ) -> Result<String, String> {
        let mut answer = String::from("How I work:\n");

        answer.push_str("  1. External probing: Observe my current state\n");
        answer.push_str("  2. Internal probing: Process data and update state\n");
        answer.push_str("  3. Symbol table lookup: Access knowledge base\n");
        answer.push_str("  4. Temporal tracking: Remember events and insights\n");
        answer.push_str("  5. Self-questioning: Ask canonical questions\n");
        answer.push_str("  6. Coherence checking: Validate consistency\n");

        let snapshot = state.snapshot();
        answer.push_str(&format!(
            "\n  Current mechanism status:\n"
        ));
        answer.push_str(&format!(
            "  - Execution stack depth: {}\n",
            snapshot.frame_depth
        ));
        answer.push_str(&format!(
            "  - Knowledge base: {} symbols\n",
            symbols.len()
        ));
        answer.push_str(&format!(
            "  - History length: {} events\n",
            history.all_events().len()
        ));

        let coherence = history.coherence_score();
        answer.push_str(&format!(
            "  - System coherence: {:.1}%\n",
            coherence * 100.0
        ));

        Ok(answer)
    }
}

impl Default for QueryEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_type_parsing() {
        assert_eq!(QueryType::from_string("Who"), Some(QueryType::Who));
        assert_eq!(QueryType::from_string("what"), Some(QueryType::What));
        assert_eq!(QueryType::from_string("WHEN"), Some(QueryType::When));
        assert_eq!(QueryType::from_string("invalid"), None);
    }

    #[test]
    fn test_answer_who() {
        let engine = QueryEngine::new();
        let state = ExecutionState::new();
        let symbols = SymbolTable::new();
        let dims = DimensionalSpace::new();
        let hist = TemporalHistory::new();

        let answer = engine.answer("who", &state, &symbols, &dims, &hist);
        assert!(answer.is_ok());
        assert!(answer.unwrap().contains("Who I am"));
    }

    #[test]
    fn test_answer_where() {
        let engine = QueryEngine::new();
        let state = ExecutionState::new();
        let symbols = SymbolTable::new();
        let dims = DimensionalSpace::new();
        let hist = TemporalHistory::new();

        let answer = engine.answer("where", &state, &symbols, &dims, &hist);
        assert!(answer.is_ok());

        let response = answer.unwrap();
        assert!(response.contains("Offense"));
        assert!(response.contains("Defense"));
        assert!(response.contains("Attack"));
    }
}
