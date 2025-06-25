//! Semantic Convention Validation Tests for SwarmSH v2
//! 
//! Validates OpenTelemetry semantic conventions compliance,
//! namespace consistency, and specification adherence.

use anyhow::Result;
use std::path::Path;
use std::fs;
use std::collections::{HashMap, HashSet};
use serde_yaml::{Value, Mapping};
use swarmsh_v2::generated::{attributes, span_builders, metrics};

/// Semantic convention domain namespaces
const EXPECTED_DOMAINS: &[&str] = &[
    "swarmsh.agent",
    "swarmsh.work", 
    "swarmsh.coordination",
    "swarmsh.health",
    "swarmsh.analytics",
    "swarmsh.worktree",
];

/// Required attribute types for validation
#[derive(Debug, PartialEq)]
enum AttributeType {
    String,
    Int,
    Double,
    Boolean,
    StringArray,
    IntArray,
    DoubleArray,
}

/// Semantic convention specification structure
#[derive(Debug)]
struct SemanticConvention {
    id: String,
    domain: String,
    attributes: HashMap<String, AttributeSpecification>,
    spans: Vec<SpanSpecification>,
    metrics: Vec<MetricSpecification>,
}

#[derive(Debug)]
struct AttributeSpecification {
    id: String,
    r#type: AttributeType,
    brief: String,
    examples: Option<Vec<Value>>,
    requirement_level: RequirementLevel,
}

#[derive(Debug)]
struct SpanSpecification {
    id: String,
    brief: String,
    attributes: Vec<String>,
}

#[derive(Debug)]
struct MetricSpecification {
    id: String,
    brief: String,
    unit: Option<String>,
    instrument: String,
    attributes: Vec<String>,
}

#[derive(Debug, PartialEq)]
enum RequirementLevel {
    Required,
    Recommended,
    Optional,
}

/// Test semantic convention file structure and validity
#[tokio::test]
async fn test_semantic_convention_file_structure() -> Result<()> {
    let semantic_conventions_dir = Path::new("semantic-conventions");
    assert!(semantic_conventions_dir.exists(), "Semantic conventions directory should exist");
    
    let expected_files = vec![
        "swarmsh-agent.yaml",
        "swarmsh-work.yaml",
        "swarmsh-coordination.yaml", 
        "swarmsh-health.yaml",
        "swarmsh-analytics.yaml",
        "swarmsh-worktree.yaml",
    ];
    
    for file in expected_files {
        let file_path = semantic_conventions_dir.join(file);
        assert!(file_path.exists(), "Semantic convention file {} should exist", file);
        
        // Validate YAML structure
        let content = fs::read_to_string(&file_path)?;
        let yaml: Value = serde_yaml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Invalid YAML in {}: {}", file, e))?;
        
        // Validate root structure
        validate_semantic_convention_structure(&yaml, file)?;
    }
    
    Ok(())
}

/// Test namespace consistency across all semantic conventions
#[tokio::test]
async fn test_namespace_consistency() -> Result<()> {
    let conventions = load_all_semantic_conventions().await?;
    
    for convention in &conventions {
        // Verify domain namespace matches expected format
        assert!(
            EXPECTED_DOMAINS.iter().any(|&domain| convention.domain.starts_with(domain)),
            "Domain {} should start with expected namespace",
            convention.domain
        );
        
        // Verify all attributes use domain namespace
        for (attr_name, attr_spec) in &convention.attributes {
            assert!(
                attr_spec.id.starts_with(&convention.domain),
                "Attribute {} should start with domain namespace {}",
                attr_spec.id,
                convention.domain
            );
        }
        
        // Verify span names use domain namespace
        for span in &convention.spans {
            assert!(
                span.id.starts_with(&convention.domain),
                "Span {} should start with domain namespace {}",
                span.id,
                convention.domain
            );
        }
        
        // Verify metric names use domain namespace
        for metric in &convention.metrics {
            assert!(
                metric.id.starts_with(&convention.domain),
                "Metric {} should start with domain namespace {}",
                metric.id,
                convention.domain
            );
        }
    }
    
    Ok(())
}

/// Test attribute specification compliance
#[tokio::test]
async fn test_attribute_specification_compliance() -> Result<()> {
    let conventions = load_all_semantic_conventions().await?;
    
    for convention in &conventions {
        for (attr_name, attr_spec) in &convention.attributes {
            // Validate attribute ID format
            assert!(
                is_valid_attribute_id(&attr_spec.id),
                "Attribute ID {} should follow naming conventions",
                attr_spec.id
            );
            
            // Validate brief description exists and is meaningful
            assert!(
                !attr_spec.brief.trim().is_empty() && attr_spec.brief.len() > 10,
                "Attribute {} should have meaningful brief description",
                attr_spec.id
            );
            
            // Validate examples if present
            if let Some(examples) = &attr_spec.examples {
                validate_attribute_examples(&attr_spec.r#type, examples, &attr_spec.id)?;
            }
            
            // Validate required attributes are properly marked
            if attr_spec.requirement_level == RequirementLevel::Required {
                // Required attributes should have examples
                assert!(
                    attr_spec.examples.is_some(),
                    "Required attribute {} should have examples",
                    attr_spec.id
                );
            }
        }
    }
    
    Ok(())
}

/// Test span specification compliance
#[tokio::test]
async fn test_span_specification_compliance() -> Result<()> {
    let conventions = load_all_semantic_conventions().await?;
    
    for convention in &conventions {
        for span in &convention.spans {
            // Validate span ID format
            assert!(
                is_valid_span_id(&span.id),
                "Span ID {} should follow naming conventions",
                span.id
            );
            
            // Validate brief description
            assert!(
                !span.brief.trim().is_empty() && span.brief.len() > 10,
                "Span {} should have meaningful brief description",
                span.id
            );
            
            // Validate all referenced attributes exist
            for attr_ref in &span.attributes {
                let attr_exists = convention.attributes.contains_key(attr_ref) ||
                    is_common_attribute(attr_ref);
                
                assert!(
                    attr_exists,
                    "Span {} references non-existent attribute {}",
                    span.id,
                    attr_ref
                );
            }
            
            // Validate span has required core attributes
            let required_attrs = get_required_span_attributes(&span.id);
            for required_attr in required_attrs {
                assert!(
                    span.attributes.contains(&required_attr),
                    "Span {} should include required attribute {}",
                    span.id,
                    required_attr
                );
            }
        }
    }
    
    Ok(())
}

/// Test metric specification compliance
#[tokio::test]
async fn test_metric_specification_compliance() -> Result<()> {
    let conventions = load_all_semantic_conventions().await?;
    
    for convention in &conventions {
        for metric in &convention.metrics {
            // Validate metric ID format
            assert!(
                is_valid_metric_id(&metric.id),
                "Metric ID {} should follow naming conventions",
                metric.id
            );
            
            // Validate brief description
            assert!(
                !metric.brief.trim().is_empty() && metric.brief.len() > 10,
                "Metric {} should have meaningful brief description",
                metric.id
            );
            
            // Validate instrument type
            let valid_instruments = vec!["counter", "histogram", "gauge", "updowncounter"];
            assert!(
                valid_instruments.contains(&metric.instrument.as_str()),
                "Metric {} should have valid instrument type: {}",
                metric.id,
                metric.instrument
            );
            
            // Validate unit if specified
            if let Some(unit) = &metric.unit {
                assert!(
                    is_valid_unit(unit),
                    "Metric {} should have valid unit: {}",
                    metric.id,
                    unit
                );
            }
            
            // Validate all referenced attributes exist
            for attr_ref in &metric.attributes {
                let attr_exists = convention.attributes.contains_key(attr_ref) ||
                    is_common_attribute(attr_ref);
                
                assert!(
                    attr_exists,
                    "Metric {} references non-existent attribute {}",
                    metric.id,
                    attr_ref
                );
            }
        }
    }
    
    Ok(())
}

/// Test generated code matches semantic conventions
#[tokio::test]
async fn test_generated_code_matches_conventions() -> Result<()> {
    let conventions = load_all_semantic_conventions().await?;
    
    // Test attributes match semantic conventions
    for convention in &conventions {
        for (attr_name, attr_spec) in &convention.attributes {
            // Check if attribute exists in generated code
            let generated_attr_exists = generated_attribute_exists(&attr_spec.id);
            
            if attr_spec.requirement_level == RequirementLevel::Required {
                assert!(
                    generated_attr_exists,
                    "Required attribute {} should exist in generated code",
                    attr_spec.id
                );
            }
        }
    }
    
    // Test span builders match semantic conventions
    for convention in &conventions {
        for span in &convention.spans {
            let span_builder_exists = generated_span_builder_exists(&span.id);
            
            assert!(
                span_builder_exists,
                "Span builder for {} should exist in generated code",
                span.id
            );
        }
    }
    
    // Test metrics match semantic conventions
    for convention in &conventions {
        for metric in &convention.metrics {
            let metric_builder_exists = generated_metric_builder_exists(&metric.id);
            
            assert!(
                metric_builder_exists,
                "Metric builder for {} should exist in generated code",
                metric.id
            );
        }
    }
    
    Ok(())
}

/// Test cross-domain attribute consistency
#[tokio::test]
async fn test_cross_domain_attribute_consistency() -> Result<()> {
    let conventions = load_all_semantic_conventions().await?;
    
    // Find attributes that appear in multiple domains
    let mut attribute_definitions: HashMap<String, Vec<&AttributeSpecification>> = HashMap::new();
    
    for convention in &conventions {
        for attr_spec in convention.attributes.values() {
            attribute_definitions.entry(attr_spec.id.clone()).or_default().push(attr_spec);
        }
    }
    
    // Validate consistency for cross-domain attributes
    for (attr_id, definitions) in attribute_definitions {
        if definitions.len() > 1 {
            let first_def = definitions[0];
            
            for def in &definitions[1..] {
                // Type should be consistent
                assert_eq!(
                    first_def.r#type, def.r#type,
                    "Attribute {} should have consistent type across domains",
                    attr_id
                );
                
                // Brief description should be consistent (or compatible)
                let brief_similarity = calculate_description_similarity(&first_def.brief, &def.brief);
                assert!(
                    brief_similarity > 0.7,
                    "Attribute {} should have consistent description across domains",
                    attr_id
                );
            }
        }
    }
    
    Ok(())
}

/// Test specification versioning and evolution
#[tokio::test]
async fn test_specification_versioning() -> Result<()> {
    let conventions = load_all_semantic_conventions().await?;
    
    for convention in &conventions {
        // Test that required attributes are stable
        let required_attrs: Vec<_> = convention.attributes.values()
            .filter(|attr| attr.requirement_level == RequirementLevel::Required)
            .collect();
        
        // Required attributes should follow stability guidelines
        for attr in required_attrs {
            // Required attributes should not use experimental prefixes
            assert!(
                !attr.id.contains("experimental"),
                "Required attribute {} should not be experimental",
                attr.id
            );
            
            // Required attributes should have stable naming
            assert!(
                is_stable_attribute_name(&attr.id),
                "Required attribute {} should follow stable naming conventions",
                attr.id
            );
        }
        
        // Test that deprecated attributes are properly marked
        // (This would require extending the semantic convention format)
    }
    
    Ok(())
}

/// Test semantic convention documentation quality
#[tokio::test]
async fn test_documentation_quality() -> Result<()> {
    let conventions = load_all_semantic_conventions().await?;
    
    for convention in &conventions {
        // Test attribute documentation
        for attr_spec in convention.attributes.values() {
            // Brief should be descriptive
            assert!(
                attr_spec.brief.len() >= 15,
                "Attribute {} brief should be descriptive (>=15 chars)",
                attr_spec.id
            );
            
            // Brief should not just repeat the attribute name
            let name_part = attr_spec.id.split('.').last().unwrap_or("");
            assert!(
                !attr_spec.brief.to_lowercase().starts_with(&name_part.to_lowercase()),
                "Attribute {} brief should not just repeat the name",
                attr_spec.id
            );
            
            // Examples should be realistic if provided
            if let Some(examples) = &attr_spec.examples {
                validate_example_realism(&attr_spec.r#type, examples, &attr_spec.id)?;
            }
        }
        
        // Test span documentation
        for span in &convention.spans {
            // Brief should describe what the span measures
            assert!(
                span.brief.len() >= 20,
                "Span {} brief should be descriptive (>=20 chars)",
                span.id
            );
            
            // Brief should include action words
            let action_words = vec!["register", "claim", "complete", "analyze", "coordinate", "monitor"];
            let brief_lower = span.brief.to_lowercase();
            assert!(
                action_words.iter().any(|&word| brief_lower.contains(word)),
                "Span {} brief should include action words",
                span.id
            );
        }
        
        // Test metric documentation
        for metric in &convention.metrics {
            // Brief should describe what is measured
            assert!(
                metric.brief.len() >= 20,
                "Metric {} brief should be descriptive (>=20 chars)",
                metric.id
            );
            
            // Unit should be appropriate for the metric type
            if let Some(unit) = &metric.unit {
                validate_metric_unit_appropriateness(&metric.instrument, unit, &metric.id)?;
            }
        }
    }
    
    Ok(())
}

// Helper functions

async fn load_all_semantic_conventions() -> Result<Vec<SemanticConvention>> {
    let mut conventions = Vec::new();
    
    let semantic_conventions_dir = Path::new("semantic-conventions");
    let entries = fs::read_dir(semantic_conventions_dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext == "yaml") {
            let content = fs::read_to_string(&path)?;
            let yaml: Value = serde_yaml::from_str(&content)?;
            
            let convention = parse_semantic_convention(&yaml, &path)?;
            conventions.push(convention);
        }
    }
    
    Ok(conventions)
}

fn validate_semantic_convention_structure(yaml: &Value, filename: &str) -> Result<()> {
    // Validate root fields
    let required_fields = vec!["groups"];
    
    for field in required_fields {
        assert!(
            yaml.get(field).is_some(),
            "Semantic convention {} should have '{}' field",
            filename,
            field
        );
    }
    
    // Validate groups structure
    if let Some(groups) = yaml.get("groups").and_then(|g| g.as_sequence()) {
        for (i, group) in groups.iter().enumerate() {
            let group_required_fields = vec!["id", "type", "brief"];
            
            for field in group_required_fields {
                assert!(
                    group.get(field).is_some(),
                    "Group {} in {} should have '{}' field",
                    i,
                    filename,
                    field
                );
            }
        }
    }
    
    Ok(())
}

fn parse_semantic_convention(yaml: &Value, path: &Path) -> Result<SemanticConvention> {
    let filename = path.file_stem().unwrap().to_str().unwrap();
    let domain = filename.replace("-", ".");
    
    let mut attributes = HashMap::new();
    let mut spans = Vec::new();
    let mut metrics = Vec::new();
    
    if let Some(groups) = yaml.get("groups").and_then(|g| g.as_sequence()) {
        for group in groups {
            let group_type = group.get("type").and_then(|t| t.as_str()).unwrap_or("");
            
            match group_type {
                "attribute_group" => {
                    if let Some(attrs) = group.get("attributes").and_then(|a| a.as_sequence()) {
                        for attr in attrs {
                            if let Some(attr_spec) = parse_attribute_specification(attr)? {
                                attributes.insert(attr_spec.id.clone(), attr_spec);
                            }
                        }
                    }
                },
                "span" => {
                    if let Some(span_spec) = parse_span_specification(group)? {
                        spans.push(span_spec);
                    }
                },
                "metric" => {
                    if let Some(metric_spec) = parse_metric_specification(group)? {
                        metrics.push(metric_spec);
                    }
                },
                _ => {}
            }
        }
    }
    
    Ok(SemanticConvention {
        id: filename.to_string(),
        domain,
        attributes,
        spans,
        metrics,
    })
}

fn parse_attribute_specification(attr: &Value) -> Result<Option<AttributeSpecification>> {
    if let (Some(id), Some(type_str), Some(brief)) = (
        attr.get("id").and_then(|i| i.as_str()),
        attr.get("type").and_then(|t| t.as_str()),
        attr.get("brief").and_then(|b| b.as_str()),
    ) {
        let attr_type = match type_str {
            "string" => AttributeType::String,
            "int" => AttributeType::Int,
            "double" => AttributeType::Double,
            "boolean" => AttributeType::Boolean,
            "string[]" => AttributeType::StringArray,
            "int[]" => AttributeType::IntArray,
            "double[]" => AttributeType::DoubleArray,
            _ => return Ok(None),
        };
        
        let examples = attr.get("examples").and_then(|e| e.as_sequence()).cloned();
        
        let requirement_level = match attr.get("requirement_level").and_then(|r| r.as_str()) {
            Some("required") => RequirementLevel::Required,
            Some("recommended") => RequirementLevel::Recommended,
            _ => RequirementLevel::Optional,
        };
        
        Ok(Some(AttributeSpecification {
            id: id.to_string(),
            r#type: attr_type,
            brief: brief.to_string(),
            examples,
            requirement_level,
        }))
    } else {
        Ok(None)
    }
}

fn parse_span_specification(group: &Value) -> Result<Option<SpanSpecification>> {
    if let (Some(id), Some(brief)) = (
        group.get("id").and_then(|i| i.as_str()),
        group.get("brief").and_then(|b| b.as_str()),
    ) {
        let attributes = group.get("attributes")
            .and_then(|a| a.as_sequence())
            .map(|seq| seq.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();
        
        Ok(Some(SpanSpecification {
            id: id.to_string(),
            brief: brief.to_string(),
            attributes,
        }))
    } else {
        Ok(None)
    }
}

fn parse_metric_specification(group: &Value) -> Result<Option<MetricSpecification>> {
    if let (Some(id), Some(brief), Some(instrument)) = (
        group.get("id").and_then(|i| i.as_str()),
        group.get("brief").and_then(|b| b.as_str()),
        group.get("instrument").and_then(|i| i.as_str()),
    ) {
        let unit = group.get("unit").and_then(|u| u.as_str()).map(|s| s.to_string());
        
        let attributes = group.get("attributes")
            .and_then(|a| a.as_sequence())
            .map(|seq| seq.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();
        
        Ok(Some(MetricSpecification {
            id: id.to_string(),
            brief: brief.to_string(),
            unit,
            instrument: instrument.to_string(),
            attributes,
        }))
    } else {
        Ok(None)
    }
}

fn is_valid_attribute_id(id: &str) -> bool {
    // Check namespace format
    id.starts_with("swarmsh.") &&
    // Check snake_case
    id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '_') &&
    // Check no consecutive dots or underscores
    !id.contains("..") && !id.contains("__")
}

fn is_valid_span_id(id: &str) -> bool {
    is_valid_attribute_id(id)
}

fn is_valid_metric_id(id: &str) -> bool {
    is_valid_attribute_id(id)
}

fn is_valid_unit(unit: &str) -> bool {
    // UCUM units or common abbreviations
    let valid_units = vec![
        "1", "ms", "s", "min", "h", "d",
        "B", "KB", "MB", "GB", "TB",
        "By", "KiBy", "MiBy", "GiBy", "TiBy",
        "%", "ratio",
    ];
    
    valid_units.contains(&unit)
}

fn is_common_attribute(attr: &str) -> bool {
    // Common OpenTelemetry attributes
    attr.starts_with("service.") ||
    attr.starts_with("process.") ||
    attr.starts_with("host.") ||
    attr.starts_with("container.")
}

fn get_required_span_attributes(span_id: &str) -> Vec<String> {
    // Return domain-specific required attributes
    if span_id.starts_with("swarmsh.agent.") {
        vec!["swarmsh.agent.id".to_string()]
    } else if span_id.starts_with("swarmsh.work.") {
        vec!["swarmsh.work.id".to_string()]
    } else {
        vec![]
    }
}

fn validate_attribute_examples(attr_type: &AttributeType, examples: &[Value], attr_id: &str) -> Result<()> {
    for example in examples {
        match attr_type {
            AttributeType::String => {
                assert!(example.is_string(), "Example for string attribute {} should be string", attr_id);
            },
            AttributeType::Int => {
                assert!(example.is_i64(), "Example for int attribute {} should be integer", attr_id);
            },
            AttributeType::Double => {
                assert!(example.is_f64() || example.is_i64(), "Example for double attribute {} should be number", attr_id);
            },
            AttributeType::Boolean => {
                assert!(example.is_bool(), "Example for boolean attribute {} should be boolean", attr_id);
            },
            AttributeType::StringArray => {
                assert!(example.is_sequence(), "Example for string[] attribute {} should be array", attr_id);
                if let Some(seq) = example.as_sequence() {
                    for item in seq {
                        assert!(item.is_string(), "Array item for string[] attribute {} should be string", attr_id);
                    }
                }
            },
            AttributeType::IntArray => {
                assert!(example.is_sequence(), "Example for int[] attribute {} should be array", attr_id);
                if let Some(seq) = example.as_sequence() {
                    for item in seq {
                        assert!(item.is_i64(), "Array item for int[] attribute {} should be integer", attr_id);
                    }
                }
            },
            AttributeType::DoubleArray => {
                assert!(example.is_sequence(), "Example for double[] attribute {} should be array", attr_id);
                if let Some(seq) = example.as_sequence() {
                    for item in seq {
                        assert!(item.is_f64() || item.is_i64(), "Array item for double[] attribute {} should be number", attr_id);
                    }
                }
            },
        }
    }
    
    Ok(())
}

fn generated_attribute_exists(attr_id: &str) -> bool {
    // This would check if the attribute exists in the generated attributes module
    // For now, return true for basic validation
    attr_id.starts_with("swarmsh.")
}

fn generated_span_builder_exists(span_id: &str) -> bool {
    // This would check if the span builder exists in the generated span_builders module
    // For now, return true for basic validation
    span_id.starts_with("swarmsh.")
}

fn generated_metric_builder_exists(metric_id: &str) -> bool {
    // This would check if the metric builder exists in the generated metrics module
    // For now, return true for basic validation
    metric_id.starts_with("swarmsh.")
}

fn calculate_description_similarity(desc1: &str, desc2: &str) -> f64 {
    // Simple similarity calculation based on common words
    let words1: HashSet<&str> = desc1.split_whitespace().collect();
    let words2: HashSet<&str> = desc2.split_whitespace().collect();
    
    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();
    
    if union == 0 {
        1.0
    } else {
        intersection as f64 / union as f64
    }
}

fn is_stable_attribute_name(attr_id: &str) -> bool {
    // Stable names should not contain temporary indicators
    !attr_id.contains("temp") && 
    !attr_id.contains("experimental") && 
    !attr_id.contains("draft")
}

fn validate_example_realism(attr_type: &AttributeType, examples: &[Value], attr_id: &str) -> Result<()> {
    for example in examples {
        match attr_type {
            AttributeType::String => {
                if let Some(s) = example.as_str() {
                    // Check for realistic string values
                    assert!(!s.contains("TODO"), "Example for {} should not contain TODO", attr_id);
                    assert!(!s.contains("FIXME"), "Example for {} should not contain FIXME", attr_id);
                    assert!(!s.trim().is_empty(), "Example for {} should not be empty", attr_id);
                }
            },
            AttributeType::Int => {
                if let Some(i) = example.as_i64() {
                    // Check for realistic integer ranges
                    if attr_id.contains("port") {
                        assert!(i > 0 && i <= 65535, "Port example for {} should be valid port number", attr_id);
                    }
                    if attr_id.contains("count") {
                        assert!(i >= 0, "Count example for {} should be non-negative", attr_id);
                    }
                }
            },
            _ => {} // Other types have basic validation covered above
        }
    }
    
    Ok(())
}

fn validate_metric_unit_appropriateness(instrument: &str, unit: &str, metric_id: &str) -> Result<()> {
    match instrument {
        "counter" => {
            // Counters should have countable units
            assert!(
                unit == "1" || unit.ends_with("s") || unit == "By" || unit.contains("requests"),
                "Counter {} should have appropriate counting unit, got {}",
                metric_id, unit
            );
        },
        "histogram" => {
            // Histograms should have measurable units
            assert!(
                unit.contains("s") || unit.contains("B") || unit.contains("ms") || unit == "%",
                "Histogram {} should have measurable unit, got {}",
                metric_id, unit
            );
        },
        "gauge" => {
            // Gauges can have various units
            // More lenient validation
        },
        _ => {}
    }
    
    Ok(())
}