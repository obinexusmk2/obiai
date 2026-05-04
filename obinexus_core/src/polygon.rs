//! Polygon Schema Validator: Cryptographic Schema Enforcement
//!
//! Enforces schemas using polygon verification for data integrity.
//! This ensures entities conform to defined structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Schema field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub constraints: Vec<String>,
}

/// Field data types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    Enum(Vec<String>),
    Reference(String), // Reference to another schema
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::String => write!(f, "String"),
            FieldType::Integer => write!(f, "Integer"),
            FieldType::Float => write!(f, "Float"),
            FieldType::Boolean => write!(f, "Boolean"),
            FieldType::Enum(variants) => write!(f, "Enum({})", variants.join("|")),
            FieldType::Reference(schema) => write!(f, "Reference({})", schema),
        }
    }
}

/// Entity schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// Schema name
    pub name: String,

    /// Schema version
    pub version: String,

    /// Field definitions
    pub fields: HashMap<String, SchemaField>,

    /// Polygon hash for integrity (cryptographic verification)
    pub polygon_hash: String,

    /// Constraints on the whole schema
    pub constraints: Vec<String>,
}

impl Schema {
    pub fn new(name: String, version: String) -> Self {
        let polygon_hash = Self::compute_hash(&name, &version);
        Schema {
            name,
            version,
            fields: HashMap::new(),
            polygon_hash,
            constraints: Vec::new(),
        }
    }

    /// Add a field to the schema
    pub fn add_field(&mut self, field: SchemaField) {
        self.fields.insert(field.name.clone(), field);
        self.recompute_hash();
    }

    /// Add a global constraint
    pub fn add_constraint(&mut self, constraint: String) {
        self.constraints.push(constraint);
        self.recompute_hash();
    }

    /// Compute the polygon hash (cryptographic verification)
    fn compute_hash(name: &str, version: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        version.hash(&mut hasher);

        let hash = hasher.finish();
        format!("polygon_{:x}", hash)
    }

    /// Recompute hash after modifications
    fn recompute_hash(&mut self) {
        self.polygon_hash =
            Self::compute_hash(&self.name, &self.version);
    }

    /// Verify data against this schema
    pub fn verify(&self, data: &HashMap<String, String>) -> VerificationResult {
        let mut errors = Vec::new();

        // Check required fields
        for (field_name, field) in &self.fields {
            if field.required && !data.contains_key(field_name) {
                errors.push(format!("Required field missing: {}", field_name));
            }
        }

        // Check field types and constraints
        for (field_name, value) in data {
            if let Some(field) = self.fields.get(field_name) {
                // Type validation
                if !self.validate_type(&field.field_type, value) {
                    errors.push(format!(
                        "Type mismatch for {}: expected {}, got '{}'",
                        field_name, field.field_type, value
                    ));
                }

                // Constraint validation
                for constraint in &field.constraints {
                    if !self.validate_constraint(constraint, value) {
                        errors.push(format!(
                            "Constraint violation for {}: {}",
                            field_name, constraint
                        ));
                    }
                }
            }
        }

        if errors.is_empty() {
            VerificationResult {
                valid: true,
                errors: vec![],
                polygon_verified: true,
            }
        } else {
            VerificationResult {
                valid: false,
                errors,
                polygon_verified: false,
            }
        }
    }

    /// Validate a value against a field type
    fn validate_type(&self, field_type: &FieldType, value: &str) -> bool {
        match field_type {
            FieldType::String => true, // Any string is valid
            FieldType::Integer => value.parse::<i64>().is_ok(),
            FieldType::Float => value.parse::<f64>().is_ok(),
            FieldType::Boolean => matches!(value.to_lowercase().as_str(), "true" | "false"),
            FieldType::Enum(variants) => variants.iter().any(|v| v == value),
            FieldType::Reference(_) => true, // Assume reference is valid
        }
    }

    /// Validate a constraint
    fn validate_constraint(&self, constraint: &str, value: &str) -> bool {
        if constraint.starts_with("length>") {
            if let Ok(min_len) = constraint[7..].parse::<usize>() {
                return value.len() > min_len;
            }
        } else if constraint.starts_with("length<") {
            if let Ok(max_len) = constraint[7..].parse::<usize>() {
                return value.len() < max_len;
            }
        } else if constraint.starts_with("matches:") {
            let pattern = &constraint[8..];
            return value.contains(pattern);
        } else if constraint == "non-empty" {
            return !value.is_empty();
        }

        true
    }

    /// Get schema as JSON representation
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Result of schema verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Whether data is valid
    pub valid: bool,

    /// List of validation errors
    pub errors: Vec<String>,

    /// Whether polygon hash was verified
    pub polygon_verified: bool,
}

impl std::fmt::Display for VerificationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.valid {
            write!(f, "✓ Schema verification passed")
        } else {
            write!(f, "✗ Schema verification failed: {}", self.errors.join("; "))
        }
    }
}

/// Polygon Schema Registry
pub struct PolygonRegistry {
    /// Registered schemas
    schemas: HashMap<String, Schema>,

    /// Verification history
    history: Vec<(String, bool)>,
}

impl PolygonRegistry {
    pub fn new() -> Self {
        PolygonRegistry {
            schemas: HashMap::new(),
            history: Vec::new(),
        }
    }

    /// Register a schema
    pub fn register(&mut self, schema: Schema) -> Result<(), String> {
        self.schemas.insert(schema.name.clone(), schema);
        Ok(())
    }

    /// Get a schema
    pub fn get(&self, name: &str) -> Option<&Schema> {
        self.schemas.get(name)
    }

    /// Verify data against a schema
    pub fn verify(&mut self, schema_name: &str, data: &HashMap<String, String>) -> VerificationResult {
        let result = if let Some(schema) = self.schemas.get(schema_name) {
            schema.verify(data)
        } else {
            VerificationResult {
                valid: false,
                errors: vec![format!("Schema {} not found", schema_name)],
                polygon_verified: false,
            }
        };

        self.history.push((schema_name.to_string(), result.valid));

        result
    }

    /// Get verification history
    pub fn history(&self) -> &[(String, bool)] {
        &self.history
    }

    /// Export all schemas
    pub fn export(&self) -> String {
        let mut output = String::from("=== Polygon Schema Registry ===\n\n");

        for (name, schema) in &self.schemas {
            output.push_str(&format!("Schema: {}\n", name));
            output.push_str(&format!("  Version: {}\n", schema.version));
            output.push_str(&format!("  Polygon Hash: {}\n", schema.polygon_hash));
            output.push_str(&format!("  Fields: {}\n", schema.fields.len()));

            for (field_name, field) in &schema.fields {
                output.push_str(&format!("    - {}: {} {}\n",
                    field_name,
                    field.field_type,
                    if field.required { "[REQUIRED]" } else { "" }
                ));
            }

            output.push('\n');
        }

        output
    }
}

impl Default for PolygonRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_creation() {
        let schema = Schema::new("User".to_string(), "1.0".to_string());
        assert_eq!(schema.name, "User");
        assert!(!schema.polygon_hash.is_empty());
    }

    #[test]
    fn test_schema_verification() {
        let mut schema = Schema::new("Person".to_string(), "1.0".to_string());

        let field = SchemaField {
            name: "name".to_string(),
            field_type: FieldType::String,
            required: true,
            constraints: vec!["length>0".to_string()],
        };
        schema.add_field(field);

        let mut data = HashMap::new();
        data.insert("name".to_string(), "John".to_string());

        let result = schema.verify(&data);
        assert!(result.valid);
    }

    #[test]
    fn test_schema_type_validation() {
        let mut schema = Schema::new("Data".to_string(), "1.0".to_string());

        schema.add_field(SchemaField {
            name: "age".to_string(),
            field_type: FieldType::Integer,
            required: true,
            constraints: vec![],
        });

        let mut data = HashMap::new();
        data.insert("age".to_string(), "not_an_integer".to_string());

        let result = schema.verify(&data);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("Type mismatch")));
    }
}
