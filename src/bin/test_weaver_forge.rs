//! Test WeaverForge template rendering without full compilation
//! 
//! This binary tests WeaverForge template generation functionality
//! without requiring the full SwarmSH v2 codebase to compile.

use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

// We'll implement a minimal version of WeaverForge functions here
// to test template rendering without dependencies

use serde_json::Value;
use std::fs;
use std::collections::HashMap;

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(fmt::layer().compact())
        .with(EnvFilter::new("info"))
        .init();

    println!("Testing WeaverForge semantic conventions loading...");

    // Load semantic conventions directly
    let semconv_files = vec![
        "semantic-conventions/swarmsh-agent.yaml",
        "semantic-conventions/swarmsh-work.yaml",
        "semantic-conventions/swarmsh-coordination.yaml",
        "semantic-conventions/swarmsh-health.yaml",
        "semantic-conventions/swarmsh-analytics.yaml",
    ];

    let mut all_conventions = Vec::new();

    for file_path in &semconv_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(data) = serde_yaml::from_str::<Value>(&content) {
                all_conventions.push(data);
                println!("✅ Loaded semantic convention: {}", file_path);
            } else {
                println!("❌ Failed to parse: {}", file_path);
            }
        } else {
            println!("❌ Failed to read: {}", file_path);
        }
    }

    println!("\nLoaded {} semantic convention files", all_conventions.len());

    // Test template rendering with minijinja
    println!("\nTesting minijinja template rendering...");
    
    let mut env = minijinja::Environment::new();
    env.set_auto_escape_callback(|_| minijinja::AutoEscape::None);
    
    // Add basic filters
    env.add_filter("pascal_case", |value: &str| -> String {
        value.split('.')
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect()
    });

    env.add_filter("snake_case", |value: &str| -> String {
        value.replace('.', "_").to_lowercase()
    });

    env.add_filter("replace", |value: &str, from: &str, to: &str| -> String {
        value.replace(from, to)
    });

    env.add_filter("lower", |value: &str| -> String {
        value.to_lowercase()
    });

    // Create a simple test template
    let test_template = r#"
//! Generated CLI Test
// Total conventions loaded: {{ ctx | length }}

{% for convention in ctx %}
{%- if convention.groups is defined %}
{%- for group in convention.groups %}
// Group: {{ group.id }} - {{ group.brief }}
pub enum {{ group.id | replace(".", "_") | pascal_case }}Commands {
    Show,
    Set,
}
{%- endfor %}
{%- endif %}
{%- endfor %}
"#;

    let template = env.template_from_str(test_template)
        .context("Failed to compile test template")?;

    let context = minijinja::context! {
        ctx => all_conventions,
    };

    let rendered = template.render(context)
        .context("Failed to render test template")?;

    println!("✅ Template rendering successful!");
    println!("\n--- Generated Output ---");
    println!("{}", rendered);
    println!("--- End Output ---\n");

    // Write test output
    fs::create_dir_all("generated/test")?;
    fs::write("generated/test/test_output.rs", rendered)?;
    println!("✅ Test output written to generated/test/test_output.rs");

    Ok(())
}