use anyhow::Result;
use swarmsh_v2::weaver_forge::{WeaverForge, WeaverConfig, TemplateConfig};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Running Weaver Forge Demo...");
    
    // Create configuration
    let config = WeaverConfig {
        semantic_conventions_path: PathBuf::from("semantic-conventions"),
        templates_path: PathBuf::from("weaver-templates"),
        output_path: PathBuf::from("."),
        templates: vec![
            TemplateConfig {
                template: "rust/attributes.j2".to_string(),
                filter: ".".to_string(),
                application_mode: "single".to_string(),
                file_name: Some("src/generated/attributes_demo.rs".to_string()),
            },
        ],
    };
    
    // Initialize forge
    let mut forge = WeaverForge::new(config)?;
    
    // Generate code
    forge.generate().await?;
    
    println!("✅ Code generation demo complete!");
    Ok(())
}
