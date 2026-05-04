//! Symbol Table: Knowledge Base for Facts, Patterns, and Learned Associations
//!
//! The symbol table is the core knowledge repository where the system stores
//! everything it knows about the world and itself.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A symbol in the knowledge base
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub value: String,
    pub binding_type: BindingType,
    pub metadata: HashMap<String, String>,
}

/// How a symbol is bound in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BindingType {
    /// Learned from the world
    Fact,
    /// Derived from reasoning
    Pattern,
    /// System's own properties
    State,
    /// Relationship to other symbols
    Association,
}

/// A binding of a symbol to its value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binding {
    pub symbol: String,
    pub value: String,
    pub confidence: f64, // 0.0 to 1.0
    pub timestamp: u64,
}

/// Symbol Table - O(1) average lookup, maintains insertion order
pub struct SymbolTable {
    symbols: IndexMap<String, Binding>,
    type_registry: HashMap<String, Vec<String>>, // Maps type to symbol names
}

impl SymbolTable {
    /// Create a new empty symbol table
    pub fn new() -> Self {
        SymbolTable {
            symbols: IndexMap::new(),
            type_registry: HashMap::new(),
        }
    }

    /// Insert a symbol with high confidence
    pub fn insert(&mut self, name: String, value: String) -> Option<Binding> {
        self.insert_with_confidence(name, value, 1.0)
    }

    /// Insert a symbol with specified confidence
    pub fn insert_with_confidence(
        &mut self,
        name: String,
        value: String,
        confidence: f64,
    ) -> Option<Binding> {
        let binding = Binding {
            symbol: name.clone(),
            value: value.clone(),
            confidence: confidence.max(0.0).min(1.0), // Clamp to [0, 1]
            timestamp: Self::current_timestamp(),
        };

        self.symbols.insert(name, binding.clone())
    }

    /// Look up a symbol by name
    pub fn get(&self, name: &str) -> Option<&Binding> {
        self.symbols.get(name)
    }

    /// Check if a symbol exists
    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// Remove a symbol
    pub fn remove(&mut self, name: &str) -> Option<Binding> {
        self.symbols.shift_remove(name)
    }

    /// Get all symbols (in insertion order)
    pub fn all_symbols(&self) -> Vec<&Binding> {
        self.symbols.values().collect()
    }

    /// Query symbols by pattern (simple substring match)
    pub fn query_pattern(&self, pattern: &str) -> Vec<&Binding> {
        self.symbols
            .values()
            .filter(|b| b.symbol.contains(pattern) || b.value.contains(pattern))
            .collect()
    }

    /// Register a symbol type
    pub fn register_type(&mut self, type_name: String, symbol: String) {
        self.type_registry
            .entry(type_name)
            .or_insert_with(Vec::new)
            .push(symbol);
    }

    /// Get all symbols of a given type
    pub fn symbols_of_type(&self, type_name: &str) -> Vec<&Binding> {
        self.type_registry
            .get(type_name)
            .map(|names| {
                names
                    .iter()
                    .filter_map(|name| self.symbols.get(name))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Number of symbols in the table
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Check if table is empty
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    /// Get average confidence of all bindings
    pub fn average_confidence(&self) -> f64 {
        if self.symbols.is_empty() {
            0.0
        } else {
            let sum: f64 = self.symbols.values().map(|b| b.confidence).sum();
            sum / self.symbols.len() as f64
        }
    }

    /// Export as JSON for inspection
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.symbols.values().collect::<Vec<_>>())
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut table = SymbolTable::new();
        table.insert("color".to_string(), "blue".to_string());

        assert!(table.contains("color"));
        let binding = table.get("color").unwrap();
        assert_eq!(binding.value, "blue");
        assert_eq!(binding.confidence, 1.0);
    }

    #[test]
    fn test_query_pattern() {
        let mut table = SymbolTable::new();
        table.insert("color_primary".to_string(), "red".to_string());
        table.insert("color_secondary".to_string(), "blue".to_string());
        table.insert("size".to_string(), "large".to_string());

        let results = table.query_pattern("color");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_confidence() {
        let mut table = SymbolTable::new();
        table.insert_with_confidence("maybe".to_string(), "uncertain".to_string(), 0.5);
        table.insert_with_confidence("yes".to_string(), "certain".to_string(), 1.0);

        let avg = table.average_confidence();
        assert!((avg - 0.75).abs() < 0.001);
    }
}
