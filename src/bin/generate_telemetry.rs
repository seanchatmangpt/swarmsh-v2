//! SwarmSH v2 Telemetry Code Generator
//! 
//! Generates Rust telemetry code from SwarmSH semantic conventions.
//! 80/20 implementation - minimal effort, maximum value.
//! Replaces Python generator with native Rust implementation.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Semantic convention group from YAML
#[derive(Debug, Deserialize)]
struct ConventionGroup {
    id: String,
    #[serde(rename = "type")]
    group_type: Option<String>,
    brief: Option<String>,
    attributes: Option<Vec<ConventionAttribute>>,
}

/// Individual attribute from semantic conventions
#[derive(Debug, Deserialize)]
struct ConventionAttribute {
    id: String,
    requirement_level: Option<String>,
    #[serde(rename = "type")]
    attr_type: Option<String>,
    brief: Option<String>,
    examples: Option<serde_yaml::Value>,
}

/// Top-level semantic convention file structure
#[derive(Debug, Deserialize)]
struct SemanticConvention {
    groups: Vec<ConventionGroup>,
}

/// Registry manifest structure
#[derive(Debug, Deserialize)]
struct RegistryManifest {
    groups: Vec<RegistryGroup>,
}

/// Registry group pointing to convention files
#[derive(Debug, Deserialize)]
struct RegistryGroup {
    id: String,
    brief: Option<String>,
    paths: Vec<String>,
}

fn main() -> Result<()> {
    println!("SwarmSH v2 Telemetry Code Generator");
    println!("Generating from semantic conventions...");

    let generator = TelemetryGenerator::new()?;
    generator.generate_all()?;

    println!("✅ Telemetry code generation complete!");
    Ok(())
}

struct TelemetryGenerator {
    conventions_dir: PathBuf,
    output_dir: PathBuf,
}

impl TelemetryGenerator {
    fn new() -> Result<Self> {
        Ok(Self {
            conventions_dir: PathBuf::from("semantic-conventions"),
            output_dir: PathBuf::from("src/generated"),
        })
    }

    fn generate_all(&self) -> Result<()> {
        // Load all semantic conventions
        let conventions = self.load_semantic_conventions()?;
        println!("Loaded {} convention files", conventions.len());

        // Create output directory
        fs::create_dir_all(&self.output_dir)?;

        // Generate attributes.rs
        let attributes_code = self.generate_attributes_rs(&conventions)?;
        let attributes_path = self.output_dir.join("attributes.rs");
        fs::write(&attributes_path, attributes_code)?;
        println!("Generated {}", attributes_path.display());

        // Generate span_builders.rs
        let span_builders_code = self.generate_span_builders_rs(&conventions)?;
        let span_builders_path = self.output_dir.join("span_builders.rs");
        fs::write(&span_builders_path, span_builders_code)?;
        println!("Generated {}", span_builders_path.display());

        // Generate metrics.rs
        let metrics_code = self.generate_metrics_rs(&conventions)?;
        let metrics_path = self.output_dir.join("metrics.rs");
        fs::write(&metrics_path, metrics_code)?;
        println!("Generated {}", metrics_path.display());

        Ok(())
    }

    fn load_semantic_conventions(&self) -> Result<HashMap<String, SemanticConvention>> {
        let mut conventions = HashMap::new();

        // Load registry manifest
        let manifest_path = self.conventions_dir.join("registry_manifest.yaml");
        let manifest_content = fs::read_to_string(&manifest_path)
            .context("Failed to read registry manifest")?;
        let manifest: RegistryManifest = serde_yaml::from_str(&manifest_content)
            .context("Failed to parse registry manifest")?;

        // Load all convention files referenced in manifest
        for group in manifest.groups {
            for path in group.paths {
                let convention_path = self.conventions_dir.join(&path);
                if convention_path.exists() {
                    let content = fs::read_to_string(&convention_path)
                        .with_context(|| format!("Failed to read {}", path))?;
                    let convention: SemanticConvention = serde_yaml::from_str(&content)
                        .with_context(|| format!("Failed to parse {}", path))?;
                    conventions.insert(path, convention);
                }
            }
        }

        Ok(conventions)
    }

    fn generate_attributes_rs(&self, conventions: &HashMap<String, SemanticConvention>) -> Result<String> {
        let mut lines = vec![
            "// Generated from SwarmSH v2 semantic conventions".to_string(),
            "// 80/20 implementation - core attributes only".to_string(),
            "".to_string(),
        ];

        // Generate constants for each group
        for (_file_path, convention) in conventions {
            for group in &convention.groups {
                let group_id = group.id.replace('.', "_").to_uppercase();
                let module_name = group_id.to_lowercase();
                
                lines.push(format!("// {}", group.brief.as_deref().unwrap_or("Group constants")));
                lines.push(format!("pub mod {} {{", module_name));

                if let Some(attributes) = &group.attributes {
                    for attr in attributes {
                        let attr_name = attr.id.replace('.', "_").to_uppercase();
                        let attr_value = format!("{}.{}", group.id, attr.id);
                        lines.push(format!("    pub const {}: &str = \"{}\";", attr_name, attr_value));
                    }
                }

                lines.push("}".to_string());
                lines.push("".to_string());
            }
        }

        Ok(lines.join("\n"))
    }

    fn generate_span_builders_rs(&self, conventions: &HashMap<String, SemanticConvention>) -> Result<String> {
        let mut lines = vec![
            "// Generated span builders from SwarmSH v2 semantic conventions".to_string(),
            "".to_string(),
            "use opentelemetry::global::BoxedSpan;".to_string(),
            "use opentelemetry::trace::Tracer;".to_string(),
            "".to_string(),
            "// Core span builders expected by lib.rs".to_string(),
            "pub fn agent_lifecycle_span(tracer: &impl Tracer) -> BoxedSpan {".to_string(),
            "    tracer.start(\"swarmsh.agent.lifecycle\")".to_string(),
            "}".to_string(),
            "".to_string(),
            "pub fn work_coordination_span(tracer: &impl Tracer) -> BoxedSpan {".to_string(),
            "    tracer.start(\"swarmsh.work.coordination\")".to_string(),
            "}".to_string(),
            "".to_string(),
            "pub fn coordination_protocol_span(tracer: &impl Tracer) -> BoxedSpan {".to_string(),
            "    tracer.start(\"swarmsh.coordination.protocol\")".to_string(),
            "}".to_string(),
            "".to_string(),
            "// Generated span builders for all groups".to_string(),
        ];

        // Generate span builders for each group
        for (_file_path, convention) in conventions {
            for group in &convention.groups {
                if group.group_type.as_deref() == Some("span") {
                    let group_name = group.id.replace('.', "_");
                    let function_name = format!("create_{}_span", group_name);
                    
                    lines.push(format!("pub fn {}(tracer: &impl Tracer) -> BoxedSpan {{", function_name));
                    lines.push(format!("    tracer.start(\"{}\")", group.id));
                    lines.push("}".to_string());
                    lines.push("".to_string());
                }
            }
        }

        Ok(lines.join("\n"))
    }

    fn generate_metrics_rs(&self, conventions: &HashMap<String, SemanticConvention>) -> Result<String> {
        let mut lines = vec![
            "// Generated metrics from SwarmSH v2 semantic conventions".to_string(),
            "".to_string(),
            "use std::collections::HashMap;".to_string(),
            "".to_string(),
            "/// SwarmSH v2 metrics registry".to_string(),
            "#[derive(Debug, Clone)]".to_string(),
            "pub struct SwarmMetrics {".to_string(),
            "    counters: HashMap<String, u64>,".to_string(),
            "    gauges: HashMap<String, f64>,".to_string(),
            "}".to_string(),
            "".to_string(),
            "impl SwarmMetrics {".to_string(),
            "    pub fn new() -> Self {".to_string(),
            "        Self {".to_string(),
            "            counters: HashMap::new(),".to_string(),
            "            gauges: HashMap::new(),".to_string(),
            "        }".to_string(),
            "    }".to_string(),
            "".to_string(),
            "    pub fn increment_counter(&mut self, name: &str) {".to_string(),
            "        *self.counters.entry(name.to_string()).or_insert(0) += 1;".to_string(),
            "    }".to_string(),
            "".to_string(),
            "    pub fn set_gauge(&mut self, name: &str, value: f64) {".to_string(),
            "        self.gauges.insert(name.to_string(), value);".to_string(),
            "    }".to_string(),
            "".to_string(),
            "    pub fn get_counter(&self, name: &str) -> u64 {".to_string(),
            "        self.counters.get(name).copied().unwrap_or(0)".to_string(),
            "    }".to_string(),
            "".to_string(),
            "    pub fn get_gauge(&self, name: &str) -> f64 {".to_string(),
            "        self.gauges.get(name).copied().unwrap_or(0.0)".to_string(),
            "    }".to_string(),
            "}".to_string(),
            "".to_string(),
            "impl Default for SwarmMetrics {".to_string(),
            "    fn default() -> Self {".to_string(),
            "        Self::new()".to_string(),
            "    }".to_string(),
            "}".to_string(),
        ];

        Ok(lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creation() {
        let generator = TelemetryGenerator::new().unwrap();
        assert!(generator.conventions_dir.exists());
    }

    #[test]
    fn test_attribute_name_conversion() {
        let input = "agent.id";
        let expected = "AGENT_ID";
        let actual = input.replace('.', "_").to_uppercase();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_module_name_conversion() {
        let input = "SWARMSH_AGENT";
        let expected = "swarmsh_agent";
        let actual = input.to_lowercase();
        assert_eq!(actual, expected);
    }
}