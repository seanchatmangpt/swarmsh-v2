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
#[derive(Debug, Deserialize, Serialize)]
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
        env.add_filter("snake_case", snake_case_filter);
        env.add_filter("camel_case", camel_case_filter);
        env.add_filter("pascal_case", pascal_case_filter);
        env.add_filter("kebab_case", kebab_case_filter);
        env.add_filter("screaming_snake_case", screaming_snake_case_filter);
        env.add_filter("comment", comment_filter);
        env.add_filter("prefix", prefix_filter);
        env.add_filter("suffix", suffix_filter);

        Ok(Self {
            template_dir,
            config,
            env,
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
        let template_content = fs::read_to_string(&template_path)
            .with_context(|| format!("Failed to read template: {}", template_path.display()))?;

        self.env.add_template(template_name, &template_content)
            .context("Failed to compile template")?;

        let template = self.env.get_template(template_name)?;
        
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