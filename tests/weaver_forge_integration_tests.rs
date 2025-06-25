//! Integration tests for Weaver Forge template generation

use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

use swarmsh_v2::weaver_forge::WeaverForge;

#[tokio::test]
async fn test_weaver_forge_initialization() {
    let forge = WeaverForge::new("weaver.yaml");
    assert!(forge.is_ok(), "Failed to initialize WeaverForge");
}

#[tokio::test] 
async fn test_semantic_conventions_loading() {
    let forge = WeaverForge::new("weaver.yaml").expect("Failed to create forge");
    let conventions = forge.load_semantic_conventions();
    
    assert!(conventions.is_ok(), "Failed to load semantic conventions");
    let data = conventions.unwrap();
    assert!(data.is_array(), "Semantic conventions should be an array");
}

#[tokio::test]
async fn test_template_validation() {
    // Check that all templates referenced in weaver.yaml exist
    let config_content = fs::read_to_string("weaver.yaml")
        .expect("Failed to read weaver.yaml");
    
    // Parse YAML and check template files exist
    let config: serde_yaml::Value = serde_yaml::from_str(&config_content)
        .expect("Failed to parse weaver.yaml");
    
    if let Some(templates) = config.get("templates").and_then(|t| t.as_sequence()) {
        for template in templates {
            if let Some(template_path) = template.get("template").and_then(|p| p.as_str()) {
                let full_path = format!("templates/registry/{}", template_path);
                assert!(
                    fs::metadata(&full_path).is_ok(),
                    "Template file not found: {}", full_path
                );
            }
        }
    }
}

#[test]
fn test_filter_functions() {
    use swarmsh_v2::weaver_forge::*;
    
    // Test case conversion filters
    assert_eq!(snake_case_filter("TestValue"), "test_value");
    assert_eq!(pascal_case_filter("test_value"), "TestValue"); 
    assert_eq!(screaming_snake_case_filter("test.value"), "TEST_VALUE");
    
    // Test comment filter
    assert_eq!(comment_filter("test", "rust"), "// test");
    assert_eq!(comment_filter("line1\nline2", "rust"), "// line1\n// line2");
}

#[tokio::test]
async fn test_template_context_building() {
    let forge = WeaverForge::new("weaver.yaml").expect("Failed to create forge");
    let conventions = forge.load_semantic_conventions().expect("Failed to load conventions");
    
    // Verify conventions have expected structure
    if let Some(array) = conventions.as_array() {
        assert!(!array.is_empty(), "Should have semantic conventions");
        
        // Check for SwarmSH-specific conventions
        let has_swarmsh = array.iter().any(|conv| {
            conv.get("groups")
                .and_then(|g| g.as_array())
                .map(|groups| {
                    groups.iter().any(|group| {
                        group.get("id")
                            .and_then(|id| id.as_str())
                            .map(|s| s.starts_with("swarmsh"))
                            .unwrap_or(false)
                    })
                })
                .unwrap_or(false)
        });
        
        assert!(has_swarmsh, "Should contain SwarmSH semantic conventions");
    }
}

#[tokio::test] 
async fn test_code_generation_coverage() {
    // Verify we have comprehensive template coverage
    let rust_templates = fs::read_dir("templates/registry/rust")
        .expect("Failed to read rust templates directory")
        .count();
    
    let shell_templates = fs::read_dir("templates/registry/shell")
        .expect("Failed to read shell templates directory")
        .count();
    
    // Should have core templates for comprehensive coverage
    assert!(rust_templates >= 8, "Should have at least 8 Rust templates");
    assert!(shell_templates >= 3, "Should have at least 3 shell templates");
}

#[tokio::test]
async fn test_shell_export_compatibility() {
    // Verify shell templates follow correct patterns
    let shell_templates = [
        "templates/registry/shell/telemetry_export.sh.j2",
        "templates/registry/shell/span_tracking.sh.j2", 
        "templates/registry/shell/metric_collection.sh.j2",
    ];
    
    for template_path in &shell_templates {
        let content = fs::read_to_string(template_path)
            .expect(&format!("Failed to read {}", template_path));
        
        // Check for shell script markers
        assert!(content.starts_with("#!/bin/bash"), "Shell template should start with shebang");
        assert!(content.contains("set -euo pipefail"), "Shell template should use strict error handling");
        
        // Check for Jinja2 template syntax
        assert!(content.contains("{{") && content.contains("}}"), "Should contain Jinja2 variable syntax");
        assert!(content.contains("{%") && content.contains("%}"), "Should contain Jinja2 control syntax");
    }
}

#[tokio::test]
async fn test_comprehensive_attribute_coverage() {
    let forge = WeaverForge::new("weaver.yaml").expect("Failed to create forge");
    let conventions = forge.load_semantic_conventions().expect("Failed to load conventions");
    
    let mut total_attributes = 0;
    let mut total_required = 0;
    let mut total_metrics = 0;
    
    if let Some(array) = conventions.as_array() {
        for convention in array {
            if let Some(groups) = convention.get("groups").and_then(|g| g.as_array()) {
                for group in groups {
                    if let Some(attributes) = group.get("attributes").and_then(|a| a.as_array()) {
                        total_attributes += attributes.len();
                        
                        for attr in attributes {
                            if let Some(req_level) = attr.get("requirement_level").and_then(|r| r.as_str()) {
                                if req_level == "required" {
                                    total_required += 1;
                                }
                            }
                        }
                    }
                    
                    if let Some(metrics) = group.get("metrics").and_then(|m| m.as_array()) {
                        total_metrics += metrics.len();
                    }
                }
            }
        }
    }
    
    // Verify comprehensive coverage
    assert!(total_attributes > 20, "Should have substantial attribute coverage: {}", total_attributes);
    assert!(total_required > 5, "Should have required attributes: {}", total_required);
    
    println!("Coverage stats: {} attributes, {} required, {} metrics", 
             total_attributes, total_required, total_metrics);
}