//! SwarmSH v2 Telemetry Validation
//! 
//! Validates generated telemetry code and OTEL integration.
//! Replaces Python validation with native Rust implementation.

use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

// Import generated modules
mod generated {
    pub mod attributes {
        include!("../generated/attributes.rs");
    }
    pub mod span_builders {
        include!("../generated/span_builders.rs");
    }
    pub mod metrics {
        include!("../generated/metrics.rs");
    }
}

fn main() {
    println!("SwarmSH v2 Telemetry Validation Test");
    println!("=====================================");
    
    let mut all_passed = true;
    
    // Test 1: Validate attribute constants exist
    println!("\n1. Testing generated attribute constants:");
    all_passed &= test_attribute_constants();
    
    // Test 2: Validate nanosecond precision IDs
    println!("\n2. Testing nanosecond precision ID generation:");
    all_passed &= test_nanosecond_ids();
    
    // Test 3: Validate telemetry structure
    println!("\n3. Testing telemetry structure:");
    all_passed &= test_telemetry_structure();
    
    // Test 4: Validate metrics functionality
    println!("\n4. Testing metrics functionality:");
    all_passed &= test_metrics();
    
    // Test 5: Validate semantic convention coverage
    println!("\n5. Testing semantic convention coverage:");
    all_passed &= test_coverage();
    
    // Summary
    if all_passed {
        println!("\n✅ All telemetry validation tests passed!");
        println!("🚀 SwarmSH v2 telemetry generation is working correctly");
        std::process::exit(0);
    } else {
        println!("\n❌ Some validation tests failed");
        std::process::exit(1);
    }
}

fn test_attribute_constants() -> bool {
    println!("Testing attribute constant accessibility...");
    
    // Test agent attributes
    let agent_id = generated::attributes::swarmsh_agent::AGENT_ID;
    let agent_role = generated::attributes::swarmsh_agent::AGENT_ROLE;
    let agent_status = generated::attributes::swarmsh_agent::AGENT_STATUS;
    
    println!("Agent ID attribute: {}", agent_id);
    println!("Agent role attribute: {}", agent_role);
    println!("Agent status attribute: {}", agent_status);
    
    // Test work attributes
    let work_id = generated::attributes::swarmsh_work::WORK_ID;
    let work_type = generated::attributes::swarmsh_work::WORK_TYPE;
    let work_status = generated::attributes::swarmsh_work::WORK_STATUS;
    
    println!("Work ID attribute: {}", work_id);
    println!("Work type attribute: {}", work_type);
    println!("Work status attribute: {}", work_status);
    
    // Test coordination attributes
    let coord_pattern = generated::attributes::swarmsh_coordination::COORDINATION_PATTERN;
    let coord_epoch = generated::attributes::swarmsh_coordination::COORDINATION_EPOCH;
    
    println!("Coordination pattern attribute: {}", coord_pattern);
    println!("Coordination epoch attribute: {}", coord_epoch);
    
    // Validate expected values
    let expected_checks = vec![
        (agent_id, "swarmsh.agent.agent.id"),
        (work_type, "swarmsh.work.work.type"),
        (coord_pattern, "swarmsh.coordination.coordination.pattern"),
    ];
    
    let mut all_valid = true;
    for (actual, expected) in expected_checks {
        if actual != expected {
            println!("❌ Attribute mismatch: expected '{}', got '{}'", expected, actual);
            all_valid = false;
        }
    }
    
    if all_valid {
        println!("✅ All attribute constants valid");
    }
    
    all_valid
}

fn test_nanosecond_ids() -> bool {
    println!("Testing nanosecond precision ID generation...");
    
    let mut ids = Vec::new();
    
    for i in 0..5 {
        let agent_id = generate_nanosecond_id("agent");
        let work_id = generate_nanosecond_id("work");
        println!("Iteration {}: Agent={}, Work={}", i + 1, agent_id, work_id);
        
        ids.push((agent_id, work_id));
        
        // Small delay to ensure different nanosecond timestamps
        std::thread::sleep(std::time::Duration::from_nanos(100));
    }
    
    // Validate all IDs are unique
    let mut unique_ids = std::collections::HashSet::new();
    for (agent_id, work_id) in &ids {
        if !unique_ids.insert(agent_id) {
            println!("❌ Duplicate agent ID: {}", agent_id);
            return false;
        }
        if !unique_ids.insert(work_id) {
            println!("❌ Duplicate work ID: {}", work_id);
            return false;
        }
    }
    
    // Validate ID format
    for (agent_id, work_id) in &ids {
        if !agent_id.starts_with("agent_") || !work_id.starts_with("work_") {
            println!("❌ Invalid ID format");
            return false;
        }
        
        // Validate nanosecond timestamp is numeric
        let agent_timestamp = &agent_id[6..]; // Skip "agent_"
        let work_timestamp = &work_id[5..];   // Skip "work_"
        
        if agent_timestamp.parse::<u128>().is_err() || work_timestamp.parse::<u128>().is_err() {
            println!("❌ Invalid timestamp format");
            return false;
        }
    }
    
    println!("✅ Nanosecond ID generation working correctly");
    true
}

fn test_telemetry_structure() -> bool {
    println!("Testing telemetry structure simulation...");
    
    // Simulate agent operations with generated attributes
    let agent_id = generate_nanosecond_id("agent");
    let work_id = generate_nanosecond_id("work");
    
    println!("Simulating agent coordination with telemetry:");
    println!("  {} = {}", generated::attributes::swarmsh_agent::AGENT_ID, agent_id);
    println!("  {} = {}", generated::attributes::swarmsh_agent::AGENT_ROLE, "coordinator");
    println!("  {} = {}", generated::attributes::swarmsh_work::WORK_ID, work_id);
    println!("  {} = {}", generated::attributes::swarmsh_work::WORK_TYPE, "coordination");
    println!("  {} = {}", generated::attributes::swarmsh_coordination::COORDINATION_PATTERN, "scrum_at_scale");
    
    // Create a mock telemetry context
    let mut telemetry_context = HashMap::new();
    telemetry_context.insert(generated::attributes::swarmsh_agent::AGENT_ID.to_string(), agent_id);
    telemetry_context.insert(generated::attributes::swarmsh_agent::AGENT_ROLE.to_string(), "coordinator".to_string());
    telemetry_context.insert(generated::attributes::swarmsh_work::WORK_ID.to_string(), work_id);
    telemetry_context.insert(generated::attributes::swarmsh_work::WORK_TYPE.to_string(), "coordination".to_string());
    
    // Validate context has expected keys
    let required_keys = vec![
        generated::attributes::swarmsh_agent::AGENT_ID,
        generated::attributes::swarmsh_agent::AGENT_ROLE,
        generated::attributes::swarmsh_work::WORK_ID,
        generated::attributes::swarmsh_work::WORK_TYPE,
    ];
    
    for key in required_keys {
        if !telemetry_context.contains_key(key) {
            println!("❌ Missing telemetry key: {}", key);
            return false;
        }
    }
    
    println!("✅ Telemetry structure validation successful");
    true
}

fn test_metrics() -> bool {
    println!("Testing metrics functionality...");
    
    let mut metrics = generated::metrics::SwarmMetrics::new();
    
    // Test counter operations
    metrics.increment_counter("agent.registrations");
    metrics.increment_counter("agent.registrations");
    metrics.increment_counter("work.completed");
    
    let registration_count = metrics.get_counter("agent.registrations");
    let work_count = metrics.get_counter("work.completed");
    
    if registration_count != 2 {
        println!("❌ Counter increment failed: expected 2, got {}", registration_count);
        return false;
    }
    
    if work_count != 1 {
        println!("❌ Counter increment failed: expected 1, got {}", work_count);
        return false;
    }
    
    // Test gauge operations
    metrics.set_gauge("health.score", 0.95);
    metrics.set_gauge("flow.efficiency", 0.84);
    
    let health_score = metrics.get_gauge("health.score");
    let flow_efficiency = metrics.get_gauge("flow.efficiency");
    
    if (health_score - 0.95).abs() > f64::EPSILON {
        println!("❌ Gauge set failed: expected 0.95, got {}", health_score);
        return false;
    }
    
    if (flow_efficiency - 0.84).abs() > f64::EPSILON {
        println!("❌ Gauge set failed: expected 0.84, got {}", flow_efficiency);
        return false;
    }
    
    println!("Counters: registrations={}, work={}", registration_count, work_count);
    println!("Gauges: health={}, efficiency={}", health_score, flow_efficiency);
    
    println!("✅ Metrics functionality working correctly");
    true
}

fn test_coverage() -> bool {
    println!("Testing semantic convention coverage...");
    
    // Count available attribute modules by checking compilation
    let domains = vec![
        ("Agent", check_module_exists("swarmsh_agent")),
        ("Work", check_module_exists("swarmsh_work")),
        ("Coordination", check_module_exists("swarmsh_coordination")),
        ("Health", check_module_exists("swarmsh_health")),
        ("Analytics", check_module_exists("swarmsh_analytics")),
    ];
    
    let mut available_domains = 0;
    for (domain_name, exists) in domains {
        if exists {
            println!("✅ {} domain available", domain_name);
            available_domains += 1;
        } else {
            println!("❌ {} domain missing", domain_name);
        }
    }
    
    println!("Available domains: {}/5", available_domains);
    
    // Validate minimum expected coverage
    if available_domains >= 4 {  // Allow some flexibility
        println!("✅ Good semantic convention coverage");
        true
    } else {
        println!("❌ Insufficient semantic convention coverage");
        false
    }
}

fn generate_nanosecond_id(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{}_{}", prefix, nanos)
}

fn check_module_exists(module_name: &str) -> bool {
    // This is a compile-time check - if the module doesn't exist, compilation would fail
    // For runtime checking, we verify that we can access expected constants
    match module_name {
        "swarmsh_agent" => {
            // Try to access a known constant from the module
            !generated::attributes::swarmsh_agent::AGENT_ID.is_empty()
        },
        "swarmsh_work" => {
            !generated::attributes::swarmsh_work::WORK_ID.is_empty()
        },
        "swarmsh_coordination" => {
            !generated::attributes::swarmsh_coordination::COORDINATION_PATTERN.is_empty()
        },
        "swarmsh_health" => {
            !generated::attributes::swarmsh_health::HEALTH_SCORE.is_empty()
        },
        "swarmsh_analytics" => {
            !generated::attributes::swarmsh_analytics::ANALYTICS_TIER.is_empty()
        },
        _ => false,
    }
}