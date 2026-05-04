//! Example: Ontological Reasoning with Schema Validation & Cost Tracking
//!
//! Demonstrates the enhanced OBINexus system with:
//! - Entity classification and ontological reasoning
//! - Polygon schema validation
//! - AEGIS cost verification

use obinexus::{
    OntologicalEntity, EntityType, OntologicalReasoner,
    Schema, PolygonRegistry, SchemaField, FieldType,
    AegisVerifier, ResourceType,
};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== OBINexus Ontological Reasoning System ===\n");

    // ========== Part 1: Ontological Reasoning ==========
    println!("--- Part 1: Entity Classification & Reasoning ---\n");

    let mut reasoner = OntologicalReasoner::new();

    // Create entities
    let mut user = OntologicalEntity::new(
        "user_001".to_string(),
        "Alice Smith".to_string(),
        EntityType::Agent,
    );
    user.add_property("role".to_string(), "developer".to_string());
    user.add_property("organization".to_string(), "OBINexus".to_string());
    user.set_semiotic_symbol("👤".to_string());

    let mut system = OntologicalEntity::new(
        "sys_001".to_string(),
        "OBINexus Core".to_string(),
        EntityType::Object,
    );
    system.add_property("type".to_string(), "AI System".to_string());
    system.add_property("status".to_string(), "operational".to_string());
    system.set_semiotic_symbol("⚙️".to_string());

    // Register entities
    reasoner.register_entity(user)?;
    reasoner.register_entity(system)?;

    // Add reasoning rules
    reasoner.add_rule(
        "Agent".to_string(),
        "Can interact with systems".to_string(),
    );
    reasoner.add_rule(
        "Object".to_string(),
        "Responds to interactions".to_string(),
    );

    // Perform reasoning
    println!("Entity: Alice Smith");
    if let Some(entity) = reasoner.get_entity("user_001") {
        println!("{}", entity.describe());

        let conclusions = reasoner.reason("user_001");
        println!("Derived conclusions:");
        for conclusion in conclusions {
            println!("  • {}", conclusion);
        }
    }

    println!();

    // ========== Part 2: Polygon Schema Validation ==========
    println!("--- Part 2: Polygon-enforced Schema Validation ---\n");

    let mut registry = PolygonRegistry::new();

    // Define User schema
    let mut user_schema = Schema::new("User".to_string(), "1.0".to_string());

    user_schema.add_field(SchemaField {
        name: "username".to_string(),
        field_type: FieldType::String,
        required: true,
        constraints: vec!["length>3".to_string(), "non-empty".to_string()],
    });

    user_schema.add_field(SchemaField {
        name: "email".to_string(),
        field_type: FieldType::String,
        required: true,
        constraints: vec!["matches:@".to_string()],
    });

    user_schema.add_field(SchemaField {
        name: "age".to_string(),
        field_type: FieldType::Integer,
        required: false,
        constraints: vec![],
    });

    user_schema.add_field(SchemaField {
        name: "status".to_string(),
        field_type: FieldType::Enum(vec![
            "active".to_string(),
            "inactive".to_string(),
            "suspended".to_string(),
        ]),
        required: true,
        constraints: vec![],
    });

    // Register schema
    registry.register(user_schema)?;
    println!("✓ User schema registered with Polygon enforcement\n");

    // Validate valid data
    let mut valid_data = HashMap::new();
    valid_data.insert("username".to_string(), "alice_smith".to_string());
    valid_data.insert("email".to_string(), "alice@example.com".to_string());
    valid_data.insert("age".to_string(), "30".to_string());
    valid_data.insert("status".to_string(), "active".to_string());

    let result = registry.verify("User", &valid_data);
    println!("Valid data verification:");
    println!("  Result: {}", result);
    println!("  Polygon verified: {}\n", result.polygon_verified);

    // Validate invalid data
    let mut invalid_data = HashMap::new();
    invalid_data.insert("username".to_string(), "bob".to_string());
    invalid_data.insert("email".to_string(), "not_an_email".to_string()); // Missing @
    invalid_data.insert("status".to_string(), "unknown".to_string()); // Invalid enum

    let result = registry.verify("User", &invalid_data);
    println!("Invalid data verification:");
    println!("  Result: {}", result);
    if !result.valid {
        println!("  Errors:");
        for error in &result.errors {
            println!("    - {}", error);
        }
    }

    println!();

    // ========== Part 3: AEGIS Cost Verification ==========
    println!("--- Part 3: AEGIS Cost Verification ---\n");

    let mut verifier = AegisVerifier::new();

    // Operation 1: External Probe
    println!("Tracking operation: External Probe");
    verifier.start_operation("probe_ext_1".to_string(), "External probe p(ext)".to_string());

    verifier.record_cost(
        "probe_ext_1",
        ResourceType::Computation,
        1000.0,
        "cycles".to_string(),
    )?;

    verifier.record_cost(
        "probe_ext_1",
        ResourceType::Memory,
        2048.0,
        "bytes".to_string(),
    )?;

    verifier.record_cost(
        "probe_ext_1",
        ResourceType::Probing,
        1.0,
        "operation".to_string(),
    )?;

    let verified_ext = verifier.complete_operation("probe_ext_1")?;
    println!("  ✓ External probe verified: {}\n", verified_ext);

    // Operation 2: Internal Probe
    println!("Tracking operation: Internal Probe");
    verifier.start_operation("probe_int_1".to_string(), "Internal probe p(int)".to_string());

    verifier.record_cost(
        "probe_int_1",
        ResourceType::Reasoning,
        500.0,
        "operations".to_string(),
    )?;

    verifier.record_cost(
        "probe_int_1",
        ResourceType::Computation,
        2000.0,
        "cycles".to_string(),
    )?;

    verifier.record_cost(
        "probe_int_1",
        ResourceType::Probing,
        1.0,
        "operation".to_string(),
    )?;

    let verified_int = verifier.complete_operation("probe_int_1")?;
    println!("  ✓ Internal probe verified: {}\n", verified_int);

    // Operation 3: Ontological Reasoning
    println!("Tracking operation: Ontological Reasoning");
    verifier.start_operation("reasoning_1".to_string(), "Entity classification".to_string());

    verifier.record_cost(
        "reasoning_1",
        ResourceType::Reasoning,
        100.0,
        "classifications".to_string(),
    )?;

    verifier.record_cost(
        "reasoning_1",
        ResourceType::Computation,
        500.0,
        "cycles".to_string(),
    )?;

    let verified_reasoning = verifier.complete_operation("reasoning_1")?;
    println!("  ✓ Reasoning verified: {}\n", verified_reasoning);

    // Export AEGIS report
    println!("=== AEGIS Accountability Report ===\n");
    println!("{}", verifier.export_report());

    // Cost breakdown
    println!("\nCost Breakdown by Resource Type:");
    let breakdown = verifier.cost_breakdown();
    for (resource, cost) in breakdown {
        println!("  {}: {:.2}", resource, cost);
    }

    println!();

    // ========== Summary ==========
    println!("=== System Summary ===\n");
    println!("✓ Ontological Reasoning: 2 entities classified and reasoned about");
    println!("✓ Polygon Schema Validation: User schema enforced with cryptographic hashes");
    println!("✓ AEGIS Cost Verification: {} operations tracked", 3);
    println!("✓ System Status: Operational with comprehensive accountability\n");

    println!("=== OBINexus OBIAI Advantages ===");
    println!("✓ Entity-aware reasoning");
    println!("✓ Schema-enforced data integrity");
    println!("✓ Transparent resource accountability");
    println!("✓ Semiotic symbolic understanding\n");

    Ok(())
}
