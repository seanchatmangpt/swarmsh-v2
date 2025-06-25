//! OTEL Weaver template engine integration for development-time code generation
//! 
//! This module provides support for OTEL Weaver's MiniJinja-based template system,
//! enabling semantic convention-driven code generation during development.
//! Note: This is NOT included in exported shell scripts for deployment.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, span, Level};
use chrono;

/// Weaver template configuration from weaver.yaml
#[derive(Debug, Deserialize, Serialize)]
pub struct WeaverConfig {
    /// Version of the weaver configuration format
    pub version: String,
    /// Target language for code generation
    pub target: String,
    /// Output directory for generated code
    pub output_dir: String,
    /// Semantic convention files to process
    pub semantic_conventions: Vec<String>,
    /// Template configurations
    pub templates: Vec<TemplateConfig>,
    /// Global parameters available to all templates
    #[serde(default)]
    pub params: HashMap<String, Value>,
}

/// Individual template configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TemplateConfig {
    /// Template file path relative to templates directory
    pub template: String,
    /// JQ filter to apply before rendering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// Application mode: "single" or "each"
    #[serde(default = "default_application_mode")]
    pub application_mode: String,
    /// Output file name or pattern
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// Additional template-specific parameters
    #[serde(default)]
    pub params: HashMap<String, Value>,
}

fn default_application_mode() -> String {
    "single".to_string()
}

/// MiniJinja template engine wrapper for OTEL Weaver
pub struct WeaverForge {
    /// Base directory for templates
    template_dir: PathBuf,
    /// Parsed weaver configuration
    config: WeaverConfig,
    /// MiniJinja environment
    env: minijinja::Environment<'static>,
    /// Template cache to maintain lifetime
    template_cache: HashMap<String, String>,
}

impl WeaverForge {
    /// Create a new WeaverForge instance from configuration
    pub fn new(config_path: impl AsRef<Path>) -> Result<Self> {
        let span = span!(Level::INFO, "weaver_forge_init");
        let _enter = span.enter();

        let config_content = fs::read_to_string(&config_path)
            .context("Failed to read weaver.yaml configuration")?;
        
        let config: WeaverConfig = serde_yaml::from_str(&config_content)
            .context("Failed to parse weaver.yaml")?;

        let config_dir = config_path.as_ref().parent()
            .context("Failed to determine config directory")?;
        
        let template_dir = config_dir.join("templates").join("registry").join(&config.target);
        
        info!(
            version = %config.version,
            target = %config.target,
            template_dir = %template_dir.display(),
            "Initialized WeaverForge"
        );

        let mut env = minijinja::Environment::new();
        env.set_auto_escape_callback(|_| minijinja::AutoEscape::None);
        
        // Add custom filters
        // Case conversion filters
        env.add_filter("snake_case", snake_case_filter);
        env.add_filter("camel_case", camel_case_filter);
        env.add_filter("pascal_case", pascal_case_filter);
        env.add_filter("kebab_case", kebab_case_filter);
        env.add_filter("screaming_snake_case", screaming_snake_case_filter);
        
        // Text manipulation filters
        env.add_filter("comment", comment_filter);
        env.add_filter("prefix", prefix_filter);
        env.add_filter("suffix", suffix_filter);
        env.add_filter("quote", quote_filter);
        env.add_filter("escape", escape_filter);
        env.add_filter("indent", indent_filter);
        
        // Type conversion filters
        env.add_filter("rust_type", rust_type_filter);
        env.add_filter("rust_metric_type", rust_metric_type_filter);
        env.add_filter("rust_value_type", rust_value_type_filter);
        env.add_filter("rust_validation_fn", rust_validation_fn_filter);
        env.add_filter("rust_unit", rust_unit_filter);
        env.add_filter("rust_attr_type", rust_attr_type_filter);
        env.add_filter("rust_metric_init", rust_metric_init_filter);
        
        // List operation filters  
        env.add_filter("selectattr", selectattr_filter);
        env.add_filter("rejectattr", rejectattr_filter);
        env.add_filter("map", map_filter);
        env.add_filter("slice", slice_filter);
        
        // Semantic convention helpers
        env.add_filter("is_required", is_required_filter);
        env.add_filter("is_recommended", is_recommended_filter);
        env.add_filter("has_examples", has_examples_filter);
        
        // SwarmSH-specific filters
        env.add_filter("swarmsh_const", swarmsh_const_filter);
        env.add_filter("is_coordination_attr", is_coordination_attr_filter);
        env.add_filter("is_ai_attr", is_ai_attr_filter);
        
        // Global functions
        env.add_function("now", now_function);
        env.add_function("env", env_function);

        Ok(Self {
            template_dir,
            config,
            env,
            template_cache: HashMap::new(),
        })
    }

    /// Load semantic conventions data
    pub fn load_semantic_conventions(&self) -> Result<Value> {
        let span = span!(Level::INFO, "load_semantic_conventions");
        let _enter = span.enter();

        let mut all_conventions = Vec::new();

        for convention_file in &self.config.semantic_conventions {
            let path = Path::new(convention_file);
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read semantic convention file: {}", path.display()))?;
            
            let data: Value = serde_yaml::from_str(&content)
                .with_context(|| format!("Failed to parse semantic convention: {}", path.display()))?;
            
            all_conventions.push(data);
        }

        Ok(Value::Array(all_conventions))
    }

    /// Process templates with JQ filters
    pub fn process_with_jq(&self, data: &Value, jq_filter: &str) -> Result<Value> {
        let span = span!(Level::DEBUG, "process_jq", filter = %jq_filter);
        let _enter = span.enter();

        // Check for special weaver-specific filters first
        match jq_filter {
            "semconv_grouped_attributes" => return self.group_attributes(data),
            "semconv_registry" => return Ok(data.clone()),
            _ => {}
        }

        // For 80/20 implementation, support common filters without full JQ
        // This covers most OTEL Weaver use cases
        match jq_filter {
            // Identity filter
            "." => Ok(data.clone()),
            
            // Array operations
            ".[]" => {
                if let Some(arr) = data.as_array() {
                    Ok(Value::Array(arr.clone()))
                } else {
                    Ok(data.clone())
                }
            }
            
            // Field access (simple path like .field or .field.subfield)
            filter if filter.starts_with('.') && !filter.contains('[') => {
                let path = filter.trim_start_matches('.');
                let mut current = data;
                
                for part in path.split('.') {
                    if part.is_empty() {
                        continue;
                    }
                    current = match current.get(part) {
                        Some(val) => val,
                        None => return Ok(Value::Null),
                    };
                }
                
                Ok(current.clone())
            }
            
            _ => {
                debug!("Complex JQ filter not yet supported: {}, returning data as-is", jq_filter);
                Ok(data.clone())
            }
        }
    }

    /// Group attributes by namespace
    fn group_attributes(&self, data: &Value) -> Result<Value> {
        let span = span!(Level::DEBUG, "group_attributes");
        let _enter = span.enter();
        
        let mut grouped = serde_json::Map::new();
        
        // Extract attributes from the semantic conventions data
        if let Some(conventions) = data.as_array() {
            for convention in conventions {
                if let Some(groups) = convention.get("groups").and_then(|g| g.as_array()) {
                    for group in groups {
                        if let Some(attributes) = group.get("attributes").and_then(|a| a.as_array()) {
                            for attr in attributes {
                                if let Some(id) = attr.get("id").and_then(|i| i.as_str()) {
                                    // Extract namespace from attribute ID (e.g., "http.request.method" -> "http")
                                    let namespace = id.split('.').next().unwrap_or("general");
                                    
                                    let entry = grouped.entry(namespace.to_string())
                                        .or_insert_with(|| Value::Array(Vec::new()));
                                    
                                    if let Some(arr) = entry.as_array_mut() {
                                        arr.push(attr.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(Value::Object(grouped))
    }

    /// Render a template with given context
    pub fn render_template(
        &mut self,
        template_name: &str,
        context: &Value,
        global_params: &HashMap<String, Value>,
    ) -> Result<String> {
        let span = span!(Level::DEBUG, "render_template", template = %template_name);
        let _enter = span.enter();

        let template_path = self.template_dir.join(template_name);
        
        // Check if template is already cached
        if !self.template_cache.contains_key(template_name) {
            let template_content = fs::read_to_string(&template_path)
                .with_context(|| format!("Failed to read template: {}", template_path.display()))?;
            
            // Store in cache before adding to environment
            self.template_cache.insert(template_name.to_string(), template_content);
        }
        
        // Get from cache and create template directly
        let cached_content = self.template_cache.get(template_name)
            .context("Template should be in cache")?;
        
        let template = self.env.template_from_str(cached_content)
            .context("Failed to compile template")?;
        
        // Build rendering context
        let mut render_context = minijinja::context! {
            ctx => context,
            params => global_params,
        };

        let rendered = template.render(render_context)
            .context("Failed to render template")?;

        Ok(rendered)
    }

    /// Generate code from templates
    pub fn generate(&mut self) -> Result<()> {
        let span = span!(Level::INFO, "generate_code");
        let _enter = span.enter();

        let semconv_data = self.load_semantic_conventions()?;

        for template_config in &self.config.templates.clone() {
            info!(
                template = %template_config.template,
                mode = %template_config.application_mode,
                "Processing template"
            );

            // Apply JQ filter if specified
            let processed_data = if let Some(filter) = &template_config.filter {
                self.process_with_jq(&semconv_data, filter)?
            } else {
                semconv_data.clone()
            };

            // Merge global and template-specific parameters
            let mut params = self.config.params.clone();
            params.extend(template_config.params.clone());

            // Render template
            let rendered = self.render_template(
                &template_config.template,
                &processed_data,
                &params,
            )?;

            // Write output
            if let Some(output) = &template_config.output {
                let output_path = Path::new(&self.config.output_dir).join(output);
                fs::create_dir_all(output_path.parent().unwrap())?;
                fs::write(&output_path, rendered)
                    .with_context(|| format!("Failed to write output: {}", output_path.display()))?;
                
                info!(output = %output_path.display(), "Generated file");
            }
        }

        Ok(())
    }
}

// Custom filter implementations

fn snake_case_filter(value: &str) -> String {
    // Convert to snake_case
    value.chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                format!("_{}", c.to_lowercase())
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect()
}

fn camel_case_filter(value: &str) -> String {
    // Convert to camelCase
    let parts: Vec<&str> = value.split('_').collect();
    parts.iter()
        .enumerate()
        .map(|(i, part)| {
            if i == 0 {
                part.to_lowercase()
            } else {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            }
        })
        .collect()
}

fn pascal_case_filter(value: &str) -> String {
    // Convert to PascalCase
    value.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn kebab_case_filter(value: &str) -> String {
    // Convert to kebab-case
    value.replace('_', "-").to_lowercase()
}

fn screaming_snake_case_filter(value: &str) -> String {
    // Convert to SCREAMING_SNAKE_CASE
    value.to_uppercase()
}

fn comment_filter(value: &str, comment_style: &str) -> String {
    // Add comment prefix to each line
    let prefix = match comment_style {
        "rust" | "c" | "cpp" => "// ",
        "python" | "shell" | "bash" => "# ",
        "html" | "xml" => "<!-- ",
        _ => "// ",
    };
    
    value.lines()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn prefix_filter(value: &str, prefix: &str) -> String {
    format!("{}{}", prefix, value)
}

fn suffix_filter(value: &str, suffix: &str) -> String {
    format!("{}{}", value, suffix)
}

// Additional filter implementations for Weaver Forge templates

fn quote_filter(value: &str) -> String {
    format!("{:?}", value)
}

fn escape_filter(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\"', "\\\"")
}

fn indent_filter(value: &str, indent: Option<usize>) -> String {
    let indent_size = indent.unwrap_or(4);
    let indent_str = " ".repeat(indent_size);
    value
        .lines()
        .map(|line| {
            if line.is_empty() {
                line.to_string()
            } else {
                format!("{}{}", indent_str, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// Type conversion filters
fn rust_type_filter(attr_type: &str) -> String {
    match attr_type {
        "string" => "&str",
        "int" => "i64",
        "double" => "f64",
        "boolean" => "bool",
        "string[]" => "Vec<&str>",
        "int[]" => "Vec<i64>",
        "double[]" => "Vec<f64>",
        "boolean[]" => "Vec<bool>",
        _ => "String",
    }.to_string()
}

fn rust_metric_type_filter(instrument: &str) -> String {
    match instrument {
        "counter" => "Counter",
        "gauge" => "Gauge",
        "histogram" => "Histogram",
        "updowncounter" => "UpDownCounter",
        _ => "Counter",
    }.to_string()
}

fn rust_value_type_filter(instrument: &str) -> String {
    match instrument {
        "counter" => "u64",
        "gauge" => "f64",
        "histogram" => "f64",
        "updowncounter" => "i64",
        _ => "f64",
    }.to_string()
}

fn rust_validation_fn_filter(attr_type: &str) -> String {
    format!("validate_{}", attr_type.replace("[]", "_array"))
}

fn rust_unit_filter(unit: &str) -> String {
    unit.replace('/', "_per_")
        .replace(' ', "_")
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 || unit.chars().nth(i - 1) == Some('_') {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

fn rust_attr_type_filter(attr_type: &str) -> String {
    attr_type
        .replace("[]", "Array")
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

fn rust_metric_init_filter(instrument: &str, metric: &minijinja::Value) -> String {
    let metric_id = metric.get_attr("id").ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "unknown".to_string());
    
    match instrument {
        "counter" => format!(r#"metrics::counter!("{}")"#, metric_id),
        "gauge" => format!(r#"metrics::gauge!("{}")"#, metric_id),
        "histogram" => format!(r#"metrics::histogram!("{}")"#, metric_id),
        "updowncounter" => format!(r#"metrics::updown_counter!("{}")"#, metric_id),
        _ => format!(r#"metrics::counter!("{}")"#, metric_id),
    }
}

// List operation filters
fn selectattr_filter(items: &minijinja::Value, attr: &str, value: &str) -> Vec<minijinja::Value> {
    if let Ok(array) = items.try_iter() {
        array
            .filter(|item| {
                item.get_attr(attr).ok()
                    .and_then(|v| v.as_str().map(|s| s == value))
                    .unwrap_or(false)
            })
            .collect()
    } else {
        Vec::new()
    }
}

fn rejectattr_filter(items: &minijinja::Value, attr: &str, value: &str) -> Vec<minijinja::Value> {
    if let Ok(array) = items.try_iter() {
        array
            .filter(|item| {
                item.get_attr(attr).ok()
                    .and_then(|v| v.as_str().map(|s| s != value))
                    .unwrap_or(true)
            })
            .collect()
    } else {
        Vec::new()
    }
}

fn map_filter(items: &minijinja::Value, attr: &str) -> Vec<String> {
    if let Ok(array) = items.try_iter() {
        array
            .filter_map(|item| {
                item.get_attr(attr).ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            })
            .collect()
    } else {
        Vec::new()
    }
}

fn slice_filter(items: &minijinja::Value, start: usize, end: Option<usize>) -> Vec<minijinja::Value> {
    if let Ok(array) = items.try_iter() {
        let vec: Vec<_> = array.collect();
        let end_idx = end.unwrap_or(vec.len()).min(vec.len());
        vec.iter()
            .skip(start)
            .take(end_idx.saturating_sub(start))
            .cloned()
            .collect()
    } else {
        Vec::new()
    }
}

// Semantic convention helpers
fn is_required_filter(attr: &minijinja::Value) -> bool {
    attr.get_attr("requirement_level").ok()
        .and_then(|v| v.as_str().map(|s| s == "required"))
        .unwrap_or(false)
}

fn is_recommended_filter(attr: &minijinja::Value) -> bool {
    attr.get_attr("requirement_level").ok()
        .and_then(|v| v.as_str().map(|s| s == "recommended"))
        .unwrap_or(false)
}

fn has_examples_filter(attr: &minijinja::Value) -> bool {
    attr.get_attr("examples").ok()
        .and_then(|v| v.try_iter().ok())
        .map(|s| s.count() > 0)
        .unwrap_or(false)
}

// SwarmSH-specific filters
fn swarmsh_const_filter(value: &str) -> String {
    format!("SWARMSH_{}", value.to_uppercase().replace('.', "_"))
}

fn is_coordination_attr_filter(id: &str) -> bool {
    id.starts_with("swarmsh.coordination")
}

fn is_ai_attr_filter(id: &str) -> bool {
    id.contains("ai") || id.contains("ollama")
}

// Global functions
fn now_function() -> String {
    chrono::Utc::now().to_rfc3339()
}

fn env_function(key: &str, default: Option<&str>) -> String {
    std::env::var(key).unwrap_or_else(|_| default.unwrap_or("").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_filters() {
        assert_eq!(snake_case_filter("HelloWorld"), "hello_world");
        assert_eq!(camel_case_filter("hello_world"), "helloWorld");
        assert_eq!(pascal_case_filter("hello_world"), "HelloWorld");
        assert_eq!(kebab_case_filter("hello_world"), "hello-world");
        assert_eq!(screaming_snake_case_filter("hello_world"), "HELLO_WORLD");
    }

    #[test]
    fn test_comment_filter() {
        assert_eq!(comment_filter("test", "rust"), "// test");
        assert_eq!(comment_filter("test", "python"), "# test");
        assert_eq!(comment_filter("line1\nline2", "rust"), "// line1\n// line2");
    }

    #[test]
    fn test_prefix_suffix_filters() {
        assert_eq!(prefix_filter("world", "hello_"), "hello_world");
        assert_eq!(suffix_filter("hello", "_world"), "hello_world");
    }
}