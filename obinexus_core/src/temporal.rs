//! Temporal History: Filter-Flash Epistemology
//!
//! Maintains a DAG of state transitions, learning events, and insight moments ("flashes").
//! Implements the filter-flash model where the system:
//! - Filters: Sorts and processes information
//! - Flashes: Stores insights for later retrieval

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// An event in the system's history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEvent {
    pub id: String,
    pub timestamp: u64,
    pub event_type: EventType,
    pub description: String,
    pub context: String,
}

/// Types of events that can be recorded
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    /// New fact learned
    Learning,

    /// State transition
    StateChange,

    /// Sudden insight
    Flash,

    /// Error or inconsistency
    Filter,

    /// Self-reflection moment
    Reflection,

    /// Query answered
    Query,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Learning => write!(f, "Learning"),
            EventType::StateChange => write!(f, "StateChange"),
            EventType::Flash => write!(f, "Flash"),
            EventType::Filter => write!(f, "Filter"),
            EventType::Reflection => write!(f, "Reflection"),
            EventType::Query => write!(f, "Query"),
        }
    }
}

/// Temporal history with DAG structure
pub struct TemporalHistory {
    /// Events in order
    events: Vec<HistoryEvent>,

    /// Event graph: maps event ID to dependent event IDs (for future DAG analysis)
    #[allow(dead_code)]
    graph: HashMap<String, Vec<String>>,

    /// Flashes (insights) stored for retrieval
    flashes: Vec<HistoryEvent>,

    /// Filters (issues to remember)
    filters: Vec<HistoryEvent>,
}

impl TemporalHistory {
    pub fn new() -> Self {
        TemporalHistory {
            events: Vec::new(),
            graph: HashMap::new(),
            flashes: Vec::new(),
            filters: Vec::new(),
        }
    }

    /// Record a learning event
    pub fn record_learning(&mut self, fact: String, value: String) {
        let event = HistoryEvent {
            id: format!("learn_{}", self.events.len()),
            timestamp: Self::current_timestamp(),
            event_type: EventType::Learning,
            description: fact,
            context: value,
        };

        self.events.push(event);
    }

    /// Record a state change
    pub fn record_state_change(&mut self, description: String, context: String) {
        let event = HistoryEvent {
            id: format!("state_{}", self.events.len()),
            timestamp: Self::current_timestamp(),
            event_type: EventType::StateChange,
            description,
            context,
        };

        self.events.push(event);
    }

    /// Record an insight flash
    pub fn record_flash(&mut self, insight: String, context: String) {
        let event = HistoryEvent {
            id: format!("flash_{}", self.flashes.len()),
            timestamp: Self::current_timestamp(),
            event_type: EventType::Flash,
            description: insight,
            context,
        };

        self.flashes.push(event.clone());
        self.events.push(event);
    }

    /// Record a filter (issue to resolve)
    pub fn record_filter(&mut self, issue: String, context: String) {
        let event = HistoryEvent {
            id: format!("filter_{}", self.filters.len()),
            timestamp: Self::current_timestamp(),
            event_type: EventType::Filter,
            description: issue,
            context,
        };

        self.filters.push(event.clone());
        self.events.push(event);
    }

    /// Record a query with its response
    pub fn record_query(&mut self, question: String, answer: String) {
        let event = HistoryEvent {
            id: format!("query_{}", self.events.len()),
            timestamp: Self::current_timestamp(),
            event_type: EventType::Query,
            description: question,
            context: answer,
        };

        self.events.push(event);
    }

    /// Get all learning events
    pub fn learnings(&self) -> Vec<&HistoryEvent> {
        self.events
            .iter()
            .filter(|e| e.event_type == EventType::Learning)
            .collect()
    }

    /// Get all flashes (insights)
    pub fn flashes(&self) -> &[HistoryEvent] {
        &self.flashes
    }

    /// Get all filters (issues)
    pub fn filters(&self) -> &[HistoryEvent] {
        &self.filters
    }

    /// Get all events
    pub fn all_events(&self) -> &[HistoryEvent] {
        &self.events
    }

    /// Get events since a specific time
    pub fn events_since(&self, timestamp: u64) -> Vec<&HistoryEvent> {
        self.events.iter().filter(|e| e.timestamp >= timestamp).collect()
    }

    /// Perform reflection based on event history
    pub fn reflection(&self, state: &crate::execution_state::ExecutionState) -> Result<String, String> {
        let mut reflection = String::from("System Reflection:\n");

        reflection.push_str(&format!(
            "Total events: {}\n",
            self.events.len()
        ));

        reflection.push_str(&format!(
            "Learnings: {}\n",
            self.learnings().len()
        ));

        reflection.push_str(&format!(
            "Insights (flashes): {}\n",
            self.flashes.len()
        ));

        reflection.push_str(&format!(
            "Issues (filters): {}\n",
            self.filters.len()
        ));

        reflection.push_str(&format!(
            "Instructions executed: {}\n",
            state.instruction_count()
        ));

        reflection.push_str(&format!(
            "Call stack depth: {}\n",
            state.stack_depth()
        ));

        // Recent insights
        if !self.flashes.is_empty() {
            reflection.push_str("\nRecent insights:\n");
            for flash in self.flashes.iter().rev().take(3) {
                reflection.push_str(&format!(
                    "  - {} ({})\n",
                    flash.description, flash.event_type
                ));
            }
        }

        Ok(reflection)
    }

    /// Check if system is coherent based on history
    pub fn coherence_score(&self) -> f64 {
        if self.events.is_empty() {
            return 0.5; // Neutral if no history
        }

        // Score based on ratio of flashes to filters
        let flash_count = self.flashes.len() as f64;
        let filter_count = self.filters.len() as f64;
        let total = self.events.len() as f64;

        // More flashes than filters = higher coherence
        let ratio = flash_count / (filter_count + 1.0);
        let base_score = (ratio / (ratio + 1.0)).min(1.0);

        // Normalize to [0.95, 1.0] if system is generally healthy
        if filter_count <= total * 0.2 {
            0.95 + (base_score * 0.05)
        } else {
            base_score * 0.95
        }
    }

    /// Clear old history (keep last N events)
    pub fn prune_history(&mut self, keep_count: usize) {
        if self.events.len() > keep_count {
            self.events = self.events[self.events.len() - keep_count..].to_vec();
        }
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

impl Default for TemporalHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_learning() {
        let mut history = TemporalHistory::new();
        history.record_learning("color".to_string(), "blue".to_string());

        assert_eq!(history.learnings().len(), 1);
        assert_eq!(history.all_events().len(), 1);
    }

    #[test]
    fn test_record_flash() {
        let mut history = TemporalHistory::new();
        history.record_flash(
            "Realized the system can introspect".to_string(),
            "during probing".to_string(),
        );

        assert_eq!(history.flashes().len(), 1);
        assert_eq!(history.all_events()[0].event_type, EventType::Flash);
    }

    #[test]
    fn test_coherence_score() {
        let mut history = TemporalHistory::new();

        // Add some flashes
        for i in 0..3 {
            history.record_flash(format!("insight{}", i), "context".to_string());
        }

        // Add a filter
        history.record_filter("issue".to_string(), "context".to_string());

        let score = history.coherence_score();
        assert!(score > 0.5); // Should be healthy with more flashes than filters
    }

    #[test]
    fn test_events_since() {
        let mut history = TemporalHistory::new();
        let before = TemporalHistory::current_timestamp();

        history.record_learning("fact1".to_string(), "value1".to_string());

        let _after = TemporalHistory::current_timestamp();
        let recent = history.events_since(before);

        assert!(recent.len() > 0);
    }
}
