//! AEGIS Cost Verification: Accountability & Cost Tracking
//!
//! Tracks resource usage, costs, and provides verification for accountability.
//! AEGIS = Auditable Execution & Guaranteed Integrity System

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Resource types tracked by AEGIS
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResourceType {
    /// CPU cycles or computation time
    Computation,
    /// Memory usage in bytes
    Memory,
    /// Network bandwidth
    Network,
    /// Storage operations
    Storage,
    /// Query/reasoning operations
    Reasoning,
    /// Probing operations
    Probing,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::Computation => write!(f, "Computation"),
            ResourceType::Memory => write!(f, "Memory"),
            ResourceType::Network => write!(f, "Network"),
            ResourceType::Storage => write!(f, "Storage"),
            ResourceType::Reasoning => write!(f, "Reasoning"),
            ResourceType::Probing => write!(f, "Probing"),
        }
    }
}

/// Cost entry for an operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEntry {
    /// Operation identifier
    pub operation_id: String,

    /// Resource type
    pub resource: ResourceType,

    /// Amount used
    pub amount: f64,

    /// Unit (bytes, cycles, etc)
    pub unit: String,

    /// Cost in arbitrary units
    pub cost: f64,

    /// Timestamp
    pub timestamp: u64,

    /// Whether this was verified
    pub verified: bool,
}

impl CostEntry {
    pub fn new(
        operation_id: String,
        resource: ResourceType,
        amount: f64,
        unit: String,
        cost: f64,
    ) -> Self {
        CostEntry {
            operation_id,
            resource,
            amount,
            unit,
            cost,
            timestamp: Self::current_timestamp(),
            verified: false,
        }
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

/// AEGIS Accountability Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountabilityRecord {
    /// Unique record ID
    pub id: String,

    /// Operation description
    pub operation: String,

    /// All costs for this operation
    pub costs: Vec<CostEntry>,

    /// Total cost
    pub total_cost: f64,

    /// Budget limit (if any)
    pub budget_limit: Option<f64>,

    /// Whether operation exceeded budget
    pub over_budget: bool,

    /// Verification status
    pub verified: bool,
}

impl AccountabilityRecord {
    pub fn new(id: String, operation: String) -> Self {
        AccountabilityRecord {
            id,
            operation,
            costs: Vec::new(),
            total_cost: 0.0,
            budget_limit: None,
            over_budget: false,
            verified: false,
        }
    }

    /// Add a cost entry
    pub fn add_cost(&mut self, entry: CostEntry) {
        self.total_cost += entry.cost;
        self.over_budget = self.budget_limit.map(|b| self.total_cost > b).unwrap_or(false);
        self.costs.push(entry);
    }

    /// Set budget limit
    pub fn set_budget(&mut self, limit: f64) {
        self.budget_limit = Some(limit);
        self.over_budget = self.total_cost > limit;
    }

    /// Verify the record
    pub fn verify(&mut self) -> bool {
        self.verified = !self.over_budget && !self.costs.is_empty();
        self.verified
    }
}

/// AEGIS Verification System
pub struct AegisVerifier {
    /// All accountability records
    records: HashMap<String, AccountabilityRecord>,

    /// Total system cost
    total_system_cost: f64,

    /// Cost limits per resource type
    resource_limits: HashMap<ResourceType, f64>,

    /// Verification log
    verification_log: Vec<String>,
}

impl AegisVerifier {
    pub fn new() -> Self {
        let mut limits = HashMap::new();
        limits.insert(ResourceType::Computation, 1_000_000.0);
        limits.insert(ResourceType::Memory, 1_000_000.0);
        limits.insert(ResourceType::Network, 100_000.0);
        limits.insert(ResourceType::Storage, 500_000.0);
        limits.insert(ResourceType::Reasoning, 100_000.0);
        limits.insert(ResourceType::Probing, 50_000.0);

        AegisVerifier {
            records: HashMap::new(),
            total_system_cost: 0.0,
            resource_limits: limits,
            verification_log: Vec::new(),
        }
    }

    /// Start tracking an operation
    pub fn start_operation(&mut self, id: String, operation: String) {
        self.records.insert(id.clone(), AccountabilityRecord::new(id, operation));
    }

    /// Record a cost for an operation
    pub fn record_cost(
        &mut self,
        operation_id: &str,
        resource: ResourceType,
        amount: f64,
        unit: String,
    ) -> Result<(), String> {
        let cost = self.calculate_cost(resource, amount);

        let entry = CostEntry::new(operation_id.to_string(), resource, amount, unit, cost);

        if let Some(record) = self.records.get_mut(operation_id) {
            record.add_cost(entry);
            self.total_system_cost += cost;

            // Check against resource limits
            if let Some(&limit) = self.resource_limits.get(&resource) {
                if self.total_system_cost > limit {
                    self.verification_log
                        .push(format!("⚠ Resource limit exceeded for {}", resource));
                }
            }

            Ok(())
        } else {
            Err(format!("Operation {} not found", operation_id))
        }
    }

    /// Complete and verify an operation
    pub fn complete_operation(&mut self, operation_id: &str) -> Result<bool, String> {
        if let Some(record) = self.records.get_mut(operation_id) {
            let verified = record.verify();

            if verified {
                self.verification_log
                    .push(format!("✓ Operation {} verified", operation_id));
            } else {
                self.verification_log
                    .push(format!("✗ Operation {} failed verification", operation_id));
            }

            Ok(verified)
        } else {
            Err(format!("Operation {} not found", operation_id))
        }
    }

    /// Calculate cost for a resource
    fn calculate_cost(&self, resource: ResourceType, amount: f64) -> f64 {
        match resource {
            ResourceType::Computation => amount * 0.001,      // 0.001 per cycle
            ResourceType::Memory => amount * 0.0001,          // 0.0001 per byte
            ResourceType::Network => amount * 0.01,           // 0.01 per unit
            ResourceType::Storage => amount * 0.0005,         // 0.0005 per operation
            ResourceType::Reasoning => amount * 0.01,         // 0.01 per operation
            ResourceType::Probing => amount * 0.02,           // 0.02 per operation
        }
    }

    /// Get operation record
    pub fn get_record(&self, operation_id: &str) -> Option<&AccountabilityRecord> {
        self.records.get(operation_id)
    }

    /// Get total cost of all operations
    pub fn total_cost(&self) -> f64 {
        self.total_system_cost
    }

    /// Get cost breakdown by resource type
    pub fn cost_breakdown(&self) -> HashMap<ResourceType, f64> {
        let mut breakdown = HashMap::new();

        for record in self.records.values() {
            for cost_entry in &record.costs {
                let total = breakdown.entry(cost_entry.resource).or_insert(0.0);
                *total += cost_entry.cost;
            }
        }

        breakdown
    }

    /// Get verification status
    pub fn verification_status(&self) -> String {
        let total_records = self.records.len();
        let verified_records = self.records.values().filter(|r| r.verified).count();

        let percentage = if total_records > 0 {
            (verified_records as f64 / total_records as f64) * 100.0
        } else {
            0.0
        };

        format!(
            "AEGIS Status: {}/{} operations verified ({:.1}%)\nTotal Cost: {:.2}",
            verified_records, total_records, percentage, self.total_system_cost
        )
    }

    /// Export accountability report
    pub fn export_report(&self) -> String {
        let mut report = String::from("=== AEGIS Accountability Report ===\n\n");

        report.push_str(&format!("Total System Cost: {:.2}\n\n", self.total_system_cost));

        report.push_str("Cost Breakdown by Resource:\n");
        let breakdown = self.cost_breakdown();
        for (resource, cost) in &breakdown {
            report.push_str(&format!("  {}: {:.2}\n", resource, cost));
        }

        report.push_str("\nOperations:\n");
        for (id, record) in &self.records {
            report.push_str(&format!("  [{}] {}\n", id, record.operation));
            report.push_str(&format!("    Cost: {:.2}", record.total_cost));

            if let Some(limit) = record.budget_limit {
                report.push_str(&format!(" / {:.2}", limit));
            }

            report.push_str(&format!("    Status: {}\n", if record.verified { "✓" } else { "✗" }));
        }

        report.push_str("\nVerification Log:\n");
        for entry in &self.verification_log {
            report.push_str(&format!("  {}\n", entry));
        }

        report
    }
}

impl Default for AegisVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_calculation() {
        let verifier = AegisVerifier::new();
        let cost = verifier.calculate_cost(ResourceType::Computation, 1000.0);
        assert_eq!(cost, 1.0); // 1000 * 0.001
    }

    #[test]
    fn test_operation_tracking() {
        let mut verifier = AegisVerifier::new();
        verifier.start_operation("op1".to_string(), "Test operation".to_string());

        assert!(verifier.record_cost("op1", ResourceType::Computation, 1000.0, "cycles".to_string()).is_ok());
        assert!(verifier.get_record("op1").is_some());
    }

    #[test]
    fn test_verification() {
        let mut verifier = AegisVerifier::new();
        verifier.start_operation("op1".to_string(), "Test".to_string());
        verifier.record_cost("op1", ResourceType::Memory, 500.0, "bytes".to_string()).unwrap();

        let verified = verifier.complete_operation("op1");
        assert!(verified.is_ok());
        assert!(verified.unwrap());
    }
}
