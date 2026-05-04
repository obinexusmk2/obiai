//! Example: Self-Aware Probing System in Action
//!
//! This example demonstrates the OBINexus system asking itself questions
//! and conducting bidirectional probing.

use obinexus::{OBINexus};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("=== OBINexus Self-Aware Probing System ===\n");

    // Create the system
    let system = OBINexus::new()?;

    println!("✓ System initialized\n");

    // Phase 1: Learn some facts
    println!("--- Phase 1: Learning Facts ---");
    system.learn("purpose", "Self-aware introspection")?;
    system.learn("created_by", "Human developers")?;
    system.learn("version", "0.1.0")?;
    system.learn("capabilities", "bidirectional_probing")?;
    println!("✓ Learned 4 facts\n");

    // Phase 2: External probe (State → Data)
    println!("--- Phase 2: External Probe ---");
    println!("p(ext): Converting internal state to observable data...");
    let external_result = system.probe_external()?;
    println!("Response: {}", external_result.response);
    println!("Confidence: {:.2}", external_result.confidence);
    println!("Data observed:\n{}\n", external_result.data);

    // Phase 3: Internal probe (Data → State)
    println!("--- Phase 3: Internal Probe ---");
    let probe_data = "status=operational\nenvironment=simulation".to_string();
    println!("p(int): Processing input data: {}", probe_data.replace("\n", "; "));
    let internal_result = system.probe_internal(probe_data)?;
    println!("Response: {}", internal_result.response);
    println!("Updated state with {} assignments\n",
        internal_result.metadata
            .split("Applied").collect::<Vec<_>>()[1]
            .split("updates").next()
            .unwrap_or("?")
    );

    // Phase 4: Ask canonical questions
    println!("--- Phase 4: Self-Questioning (Canonical Questions) ---");

    let questions = vec!["who", "what", "when", "where", "why", "how"];

    for question in questions {
        println!("\n[QUESTION] {}?", question.to_uppercase());
        match system.ask(question) {
            Ok(answer) => {
                println!("{}", answer);
            }
            Err(e) => println!("Error: {}", e),
        }
        println!("{}", "─".repeat(50));
    }

    // Phase 5: Self-reflection
    println!("\n--- Phase 5: Self-Reflection ---");
    match system.self_reflect() {
        Ok(reflection) => {
            println!("{}", reflection);
        }
        Err(e) => println!("Error in reflection: {}", e),
    }

    // Phase 6: Coherence check
    println!("\n--- Phase 6: Coherence Verification (95.4% Standard) ---");
    match system.check_coherence() {
        Ok(coherence_score) => {
            let percentage = coherence_score * 100.0;
            println!("Coherence Score: {:.1}%", percentage);

            if coherence_score >= 0.954 {
                println!("✓ System MEETS the 95.4% safety standard");
                println!("  Safe for deployment in safety-critical contexts");
            } else {
                println!("⚠ System below 95.4% standard");
                println!("  Recommend additional learning/calibration");
            }
        }
        Err(e) => println!("Error checking coherence: {}", e),
    }

    println!("\n=== Probing Cycle Complete ===");

    Ok(())
}
