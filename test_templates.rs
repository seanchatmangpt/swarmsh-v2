#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! serde_yaml = "0.9"
//! serde_json = "1.0"
//! minijinja = "2.10"
//! anyhow = "1.0"
//! ```

//! Standalone template testing script for WeaverForge
//! Tests semantic convention loading and template rendering

use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;

fn main() -> Result<()> {
    println!("🧪 Testing WeaverForge template rendering...\n");

    // Load semantic conventions
    let semconv_files = vec![
        "semantic-conventions/swarmsh-agent.yaml",
        "semantic-conventions/swarmsh-work.yaml", 
        "semantic-conventions/swarmsh-coordination.yaml",
        "semantic-conventions/swarmsh-health.yaml",
        "semantic-conventions/swarmsh-analytics.yaml",
    ];

    let mut all_conventions = Vec::new();

    for file_path in &semconv_files {
        match fs::read_to_string(file_path) {
            Ok(content) => {
                match serde_yaml::from_str::<Value>(&content) {
                    Ok(data) => {
                        all_conventions.push(data);
                        println!("✅ Loaded: {}", file_path);
                    }
                    Err(e) => println!("❌ Parse error in {}: {}", file_path, e),
                }
            }
            Err(e) => println!("❌ Read error for {}: {}", file_path, e),
        }
    }

    println!("\n📊 Loaded {} semantic convention files\n", all_conventions.len());

    // Set up minijinja environment
    let mut env = minijinja::Environment::new();
    env.set_auto_escape_callback(|_| minijinja::AutoEscape::None);
    
    // Add filters
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

    // Test simple CLI template
    let cli_template = r#"
//! Generated SwarmSH v2 CLI Commands
//! Total conventions: {{ ctx | length }}

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct SwarmShCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
{% for convention in ctx %}
{%- if convention.groups is defined %}
{%- for group in convention.groups %}
    /// {{ group.brief }}
    {{ group.id | replace(".", "_") | pascal_case }} {
        #[command(subcommand)]
        action: {{ group.id | replace(".", "_") | pascal_case }}Commands,
    },
{%- endfor %}
{%- endif %}
{%- endfor %}
}

{% for convention in ctx %}
{%- if convention.groups is defined %}
{%- for group in convention.groups %}
#[derive(Subcommand)]
pub enum {{ group.id | replace(".", "_") | pascal_case }}Commands {
    Show,
    Set,
}
{%- endfor %}
{%- endif %}
{%- endfor %}
"#;

    // Render template
    let template = env.template_from_str(cli_template)
        .context("Failed to compile CLI template")?;

    let context = minijinja::context! {
        ctx => all_conventions,
    };

    let rendered = template.render(&context)
        .context("Failed to render CLI template")?;

    println!("🎉 Template rendering successful!\n");
    
    // Show excerpt
    let lines: Vec<&str> = rendered.lines().collect();
    let excerpt_end = std::cmp::min(30, lines.len());
    
    println!("📝 Generated CLI code excerpt:");
    println!("--- First {} lines ---", excerpt_end);
    for (i, line) in lines.iter().take(excerpt_end).enumerate() {
        println!("{:3}: {}", i + 1, line);
    }
    if lines.len() > excerpt_end {
        println!("... ({} more lines)", lines.len() - excerpt_end);
    }
    println!("--- End excerpt ---\n");

    // Write output
    fs::create_dir_all("generated/cli").context("Failed to create output directory")?;
    fs::write("generated/cli/generated_cli.rs", &rendered)
        .context("Failed to write generated CLI")?;
    
    println!("💾 Generated CLI written to: generated/cli/generated_cli.rs");
    println!("📏 Total size: {} lines, {} bytes", lines.len(), rendered.len());

    // Test shell template too
    println!("\n🐚 Testing shell CLI template...");
    
    let shell_template = r#"#!/bin/bash
# Generated SwarmSH v2 Shell CLI
# Total conventions: {{ ctx | length }}

set -euo pipefail

main() {
    case "${1:-help}" in
{% for convention in ctx %}
{%- if convention.groups is defined %}
{%- for group in convention.groups %}
        "{{ group.id | replace(".", "-") }}")
            handle_{{ group.id | replace(".", "_") }}_ "$@"
            ;;
{%- endfor %}
{%- endif %}
{%- endfor %}
        *)
            echo "SwarmSH v2 - Available commands:"
{% for convention in ctx %}
{%- if convention.groups is defined %}
{%- for group in convention.groups %}
            echo "  {{ group.id | replace(".", "-") }} - {{ group.brief }}"
{%- endfor %}
{%- endif %}
{%- endfor %}
            ;;
    esac
}

{% for convention in ctx %}
{%- if convention.groups is defined %}
{%- for group in convention.groups %}
handle_{{ group.id | replace(".", "_") }}_() {
    echo "Handling {{ group.brief | lower }}"
    # TODO: Implement {{ group.id }} operations
}

{%- endfor %}
{%- endif %}
{%- endfor %}
main "$@"
"#;

    let shell_template_obj = env.template_from_str(shell_template)
        .context("Failed to compile shell template")?;

    let shell_rendered = shell_template_obj.render(&context)
        .context("Failed to render shell template")?;

    fs::write("generated/cli/swarmsh_cli.sh", &shell_rendered)
        .context("Failed to write shell CLI")?;
    
    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata("generated/cli/swarmsh_cli.sh")?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions("generated/cli/swarmsh_cli.sh", perms)?;
    }

    println!("💾 Generated shell CLI written to: generated/cli/swarmsh_cli.sh");
    
    let shell_lines = shell_rendered.lines().count();
    println!("📏 Shell size: {} lines, {} bytes", shell_lines, shell_rendered.len());

    println!("\n🎉 WeaverForge template generation test completed successfully!");
    println!("🔍 Check generated/cli/ for output files");

    Ok(())
}