//! Shell export system - Export Rust implementation to shell scripts
//! 
//! Provides complete functionality as optimized shell scripts for UNIX deployment.
//! Maintains all coordination guarantees while enabling shell-only execution.
//! Uses Tera templating engine for powerful template generation.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tera::{Tera, Context};
use std::collections::HashMap;
use std::sync::Arc;
use crate::ai_integration::AIIntegration;
use tracing::{info, debug, warn, error, instrument};
use crate::telemetry::{SwarmTelemetry, DefaultSwarmTelemetry};
use std::time::Instant;

/// Configuration for shell export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    pub output_dir: PathBuf,
    pub include_telemetry: bool,
    pub include_ai_integration: bool,
    pub optimization_level: u8, // 1-3, higher = more optimized
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./shell-export"),
            include_telemetry: true,
            include_ai_integration: true,
            optimization_level: 2,
        }
    }
}

/// Shell exporter for converting Rust implementation to shell scripts
pub struct ShellExporter {
    /// Tera template engine for generating shell scripts
    tera: Tera,
    /// AI integration for intelligent shell optimization
    ai_integration: Option<Arc<AIIntegration>>,
    /// Telemetry for shell export operations
    swarm_telemetry: DefaultSwarmTelemetry,
}

impl ShellExporter {
    #[instrument(skip_all)]
    pub async fn new() -> Result<Self> {
        let start_time = Instant::now();
        let swarm_telemetry = DefaultSwarmTelemetry::default();
        let _span = swarm_telemetry.coordination_span("shell_export", "initialize").entered();
        
        info!("Initializing shell exporter with Tera templating engine");
        
        // Initialize Tera with template directory
        let mut tera = Tera::new("templates/**/*.tera")?;
        
        // Add custom filters for shell script generation
        tera.register_filter("shell_escape", shell_escape_filter);
        tera.register_filter("to_bash_array", bash_array_filter);
        tera.register_filter("nanosecond_id", nanosecond_id_filter);
        
        // Initialize AI integration for intelligent optimization
        let ai_integration = match AIIntegration::new().await {
            Ok(ai) => {
                info!("Shell exporter initialized with AI optimization support");
                Some(Arc::new(ai))
            }
            Err(e) => {
                debug!("AI integration unavailable for shell export: {}", e);
                None
            }
        };
        
        let init_duration = start_time.elapsed();
        info!(init_duration_ms = init_duration.as_millis(), ai_available = ai_integration.is_some(), "Shell exporter initialized");
        
        Ok(Self { tera, ai_integration, swarm_telemetry })
    }
    
    /// Export complete SwarmSH system to shell scripts
    #[instrument(skip(self, system), fields(output_dir = ?config.output_dir, optimization_level = config.optimization_level))]
    pub async fn export_system(
        &self, 
        system: &crate::SwarmSystem,
        config: ExportConfig,
    ) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.coordination_span("shell_export", "export_system").entered();
        
        info!(
            output_dir = ?config.output_dir,
            include_telemetry = config.include_telemetry,
            include_ai_integration = config.include_ai_integration,
            optimization_level = config.optimization_level,
            "Starting SwarmSH system export to shell scripts using Tera templating"
        );
        
        // Create output directory
        std::fs::create_dir_all(&config.output_dir)?;
        
        // Prepare template context
        let mut context = Context::new();
        context.insert("config", &config);
        context.insert("timestamp", &chrono::Utc::now().to_rfc3339());
        context.insert("version", "2.0.0");
        
        // Export each component using Tera templates
        self.export_coordination_with_template(&config, &context).await?;
        self.export_telemetry_with_template(&config, &context).await?;
        self.export_health_monitoring_with_template(&config, &context).await?;
        self.export_analytics_with_template(&config, &context).await?;
        
        if config.include_ai_integration {
            let ai_start = Instant::now();
            self.export_ai_integration_with_template(&config, &context).await?;
            let ai_duration = ai_start.elapsed();
            info!(ai_export_duration_ms = ai_duration.as_millis(), "AI integration export completed");
        }
        
        let total_duration = start_time.elapsed();
        info!(
            total_export_duration_ms = total_duration.as_millis(),
            telemetry_included = config.include_telemetry,
            ai_included = config.include_ai_integration,
            "Shell export completed successfully using Tera templating"
        );
        
        Ok(())
    }
    
    /// Export specific component to shell
    #[instrument(skip(self, config), fields(component = %component))]
    pub async fn export_component(&self, component: &str, config: &ExportConfig) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.coordination_span("shell_export", "export_component").entered();
        
        info!(component = %component, "Starting component export to shell");
        
        let result = match component {
            "coordination" => self.export_coordination(config).await,
            "telemetry" => self.export_telemetry(config).await,
            "health" => self.export_health_monitoring(config).await,
            "analytics" => self.export_analytics(config).await,
            "ai" => self.export_ai_integration(config).await,
            _ => {
                warn!(component = %component, "Unknown component requested for export");
                Ok(())
            }
        };
        
        let component_duration = start_time.elapsed();
        info!(
            component = %component,
            export_duration_ms = component_duration.as_millis(),
            "Component export completed"
        );
        
        result
    }
    
    async fn export_coordination(&self, config: &ExportConfig) -> Result<()> {
        self.export_coordination_with_template(config, &Context::new()).await
    }
    
    async fn export_telemetry(&self, config: &ExportConfig) -> Result<()> {
        self.export_telemetry_with_template(config, &Context::new()).await
    }
    
    async fn export_health_monitoring(&self, config: &ExportConfig) -> Result<()> {
        self.export_health_monitoring_with_template(config, &Context::new()).await
    }
    
    async fn export_analytics(&self, config: &ExportConfig) -> Result<()> {
        self.export_analytics_with_template(config, &Context::new()).await
    }
    
    async fn export_ai_integration(&self, config: &ExportConfig) -> Result<()> {
        self.export_ai_integration_with_template(config, &Context::new()).await
    }
    
    // Tera template-based export methods with AI enhancement
    #[instrument(skip(self, config, context))]
    async fn export_coordination_with_template(&self, config: &ExportConfig, context: &Context) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.coordination_span("shell_export", "export_coordination").entered();
        
        info!("Generating coordination shell scripts with AI-enhanced Tera templates");
        
        let mut template_context = context.clone();
        template_context.insert("coordination_patterns", &vec![
            "scrum_at_scale",
            "roberts_rules", 
            "realtime",
            "atomic"
        ]);
        template_context.insert("nanosecond_precision", &true);
        template_context.insert("zero_conflict_guarantee", &true);
        
        // Get AI enhancement for coordination
        let enhanced_context = self.get_ai_enhanced_context(&template_context, "coordination").await?;
        
        // Render and optimize coordination helper
        let coordination_script = self.tera.render("coordination_helper.sh.tera", &enhanced_context)?;
        let optimized_script = self.optimize_shell_script(
            &coordination_script,
            "Zero-conflict agent coordination with nanosecond precision"
        ).await?;
        let output_path = config.output_dir.join("coordination_helper.sh");
        std::fs::write(output_path, optimized_script)?;
        
        // Render and optimize agent orchestrator
        let orchestrator_script = self.tera.render("agent_swarm_orchestrator.sh.tera", &enhanced_context)?;
        let optimized_orchestrator = self.optimize_shell_script(
            &orchestrator_script,
            "Agent swarm orchestration with intelligent work distribution"
        ).await?;
        let output_path = config.output_dir.join("agent_swarm_orchestrator.sh");
        std::fs::write(output_path, optimized_orchestrator)?;
        
        // Render and optimize real agent coordinator
        let coordinator_script = self.tera.render("real_agent_coordinator.sh.tera", &enhanced_context)?;
        let optimized_coordinator = self.optimize_shell_script(
            &coordinator_script,
            "Real-time agent coordination with AI decision making"
        ).await?;
        let output_path = config.output_dir.join("real_agent_coordinator.sh");
        std::fs::write(output_path, optimized_coordinator)?;
        
        let coordination_duration = start_time.elapsed();
        info!(
            coordination_export_duration_ms = coordination_duration.as_millis(),
            scripts_generated = 3,
            "Coordination shell scripts generated successfully"
        );
        
        Ok(())
    }
    
    #[instrument(skip(self, config, context))]
    async fn export_telemetry_with_template(&self, config: &ExportConfig, context: &Context) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.coordination_span("shell_export", "export_telemetry").entered();
        
        info!("Generating telemetry shell scripts with Tera templates");
        
        if !config.include_telemetry {
            return Ok(());
        }
        
        let mut template_context = context.clone();
        template_context.insert("semantic_conventions", &vec![
            "swarmsh.agent",
            "swarmsh.work",
            "swarmsh.coordination", 
            "swarmsh.health",
            "swarmsh.analytics"
        ]);
        template_context.insert("otel_export_format", &"json");
        
        let telemetry_script = self.tera.render("telemetry_spans.sh.tera", &template_context)?;
        let output_path = config.output_dir.join("telemetry_spans.sh");
        std::fs::write(output_path, telemetry_script)?;
        
        let telemetry_duration = start_time.elapsed();
        info!(
            telemetry_export_duration_ms = telemetry_duration.as_millis(),
            "Telemetry shell scripts generated successfully"
        );
        
        Ok(())
    }
    
    #[instrument(skip(self, config, context))]
    async fn export_health_monitoring_with_template(&self, config: &ExportConfig, context: &Context) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.coordination_span("shell_export", "export_health").entered();
        
        info!("Generating health monitoring shell scripts with Tera templates");
        
        let mut template_context = context.clone();
        template_context.insert("monitoring_tiers", &vec!["tier1", "tier2"]);
        template_context.insert("health_components", &vec![
            "coordination",
            "telemetry", 
            "automation",
            "ai",
            "work_queue",
            "storage"
        ]);
        template_context.insert("automated_remediation", &true);
        
        let health_script = self.tera.render("health_monitor.sh.tera", &template_context)?;
        let output_path = config.output_dir.join("health_monitor.sh");
        std::fs::write(output_path, health_script)?;
        
        let health_duration = start_time.elapsed();
        info!(
            health_export_duration_ms = health_duration.as_millis(),
            monitoring_components = template_context.get("health_components").map(|v| v.as_array().map(|a| a.len()).unwrap_or(0)).unwrap_or(0),
            "Health monitoring shell scripts generated successfully"
        );
        
        Ok(())
    }
    
    #[instrument(skip(self, config, context))]
    async fn export_analytics_with_template(&self, config: &ExportConfig, context: &Context) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.coordination_span("shell_export", "export_analytics").entered();
        
        info!("Generating analytics shell scripts with Tera templates");
        
        let mut template_context = context.clone();
        template_context.insert("optimization_targets", &serde_json::json!({
            "waste_elimination": 73.0,
            "flow_efficiency": 84.0,
            "lead_time_ms": 126000,
            "sigma_level": 4.2
        }));
        template_context.insert("dlss_principles", &vec![
            "overproduction",
            "waiting",
            "transport", 
            "over_processing",
            "inventory",
            "motion",
            "defects"
        ]);
        
        let analytics_script = self.tera.render("8020_automation.sh.tera", &template_context)?;
        let output_path = config.output_dir.join("8020_automation.sh");
        std::fs::write(output_path, analytics_script)?;
        
        let analytics_duration = start_time.elapsed();
        info!(
            analytics_export_duration_ms = analytics_duration.as_millis(),
            dlss_principles_count = template_context.get("dlss_principles").map(|v| v.as_array().map(|a| a.len()).unwrap_or(0)).unwrap_or(0),
            "Analytics shell scripts generated successfully"
        );
        
        Ok(())
    }
    
    #[instrument(skip(self, config, context))]
    async fn export_ai_integration_with_template(&self, config: &ExportConfig, context: &Context) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.coordination_span("shell_export", "export_ai_integration").entered();
        
        info!("Generating AI integration shell scripts with AI-enhanced Tera templates");
        
        let mut template_context = context.clone();
        template_context.insert("ai_providers", &vec!["claude", "ollama"]);
        template_context.insert("confidence_threshold", &0.8);
        template_context.insert("fallback_to_human", &true);
        
        // Add Ollama-specific features
        template_context.insert("ollama_features", &serde_json::json!({
            "chat_completion": true,
            "embeddings": true,
            "streaming": true,
            "model_management": true,
            "pattern_analysis": true,
            "decision_making": true,
            "shell_optimization": true
        }));
        
        // Get AI enhancement for the AI integration component itself
        let enhanced_context = self.get_ai_enhanced_context(&template_context, "ai_integration").await?;
        
        // Claude integration with optimization
        let claude_script = self.tera.render("claude_integration.sh.tera", &enhanced_context)?;
        let optimized_claude = self.optimize_shell_script(
            &claude_script,
            "Claude API integration for comprehensive analysis and planning"
        ).await?;
        let output_path = config.output_dir.join("claude_integration.sh");
        std::fs::write(output_path, optimized_claude)?;
        
        // Ollama integration with full feature optimization
        let ollama_script = self.tera.render("ollama_integration.sh.tera", &enhanced_context)?;
        let optimized_ollama = self.optimize_shell_script(
            &ollama_script,
            "Local Ollama integration with chat, embeddings, streaming, and model management"
        ).await?;
        let output_path = config.output_dir.join("ollama_integration.sh");
        std::fs::write(output_path, optimized_ollama)?;
        
        // Generate AI-powered shell utilities
        let ai_utils_script = self.tera.render("ai_shell_utils.sh.tera", &enhanced_context)?;
        let optimized_utils = self.optimize_shell_script(
            &ai_utils_script,
            "AI utility functions for intelligent shell script enhancement"
        ).await?;
        let output_path = config.output_dir.join("ai_shell_utils.sh");
        std::fs::write(output_path, optimized_utils)?;
        
        let ai_integration_duration = start_time.elapsed();
        info!(
            ai_integration_export_duration_ms = ai_integration_duration.as_millis(),
            ai_scripts_generated = 3,
            ai_providers_count = template_context.get("ai_providers").map(|v| v.as_array().map(|a| a.len()).unwrap_or(0)).unwrap_or(0),
            "AI integration shell scripts generated successfully"
        );
        
        Ok(())
    }
    
    /// Optimize shell script using AI analysis
    #[instrument(skip(self, script))]
    pub async fn optimize_shell_script(&self, script: &str, requirements: &str) -> Result<String> {
        if let Some(ref ai) = self.ai_integration {
            match ai.optimize_shell_script(script, requirements).await {
                Ok(optimized) => {
                    info!("AI optimization applied to shell script");
                    return Ok(optimized);
                }
                Err(e) => {
                    debug!("AI optimization failed, using original: {}", e);
                }
            }
        }
        
        // Return original script if AI optimization fails or is unavailable
        Ok(script.to_string())
    }
    
    /// Generate AI-enhanced shell template context
    #[instrument(skip(self, base_context), fields(component = %component))]
    async fn get_ai_enhanced_context(&self, base_context: &Context, component: &str) -> Result<Context> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.coordination_span("shell_export", "ai_enhance_context").entered();
        
        let mut enhanced_context = base_context.clone();
        
        if let Some(ref ai) = self.ai_integration {
            let analysis_prompt = format!(
                "Analyze shell export requirements for SwarmSH component: {}. Provide optimization suggestions for performance, reliability, and maintainability.",
                component
            );
            
            match ai.analyze(&analysis_prompt).await {
                Ok(analysis) => {
                    enhanced_context.insert("ai_recommendations", &analysis.recommendations);
                    enhanced_context.insert("ai_confidence", &analysis.confidence);
                    enhanced_context.insert("optimization_opportunities", &analysis.optimization_opportunities);
                    
                    if let Some(ref reasoning) = analysis.reasoning {
                        enhanced_context.insert("ai_reasoning", reasoning);
                    }
                    
                    info!(
                        component = %component,
                        recommendations_count = analysis.recommendations.len(),
                        ai_confidence = analysis.confidence,
                        "Enhanced shell context with AI analysis"
                    );
                }
                Err(e) => {
                    warn!(component = %component, error = %e, "Failed to get AI analysis for component");
                }
            }
        }
        
        let enhancement_duration = start_time.elapsed();
        debug!(
            component = %component,
            enhancement_duration_ms = enhancement_duration.as_millis(),
            ai_available = self.ai_integration.is_some(),
            "AI context enhancement completed"
        );
        
        Ok(enhanced_context)
    }
}

// Tera custom filters for shell script generation

/// Shell escape filter for safe string interpolation
fn shell_escape_filter(value: &tera::Value, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    match value {
        tera::Value::String(s) => {
            let escaped = s.replace('\\', "\\\\")
                          .replace('\"', "\\\"")
                          .replace('$', "\\$")
                          .replace('`', "\\`");
            Ok(tera::Value::String(format!("\"{}\"", escaped)))
        }
        _ => Ok(value.clone())
    }
}

/// Convert array to bash array format
fn bash_array_filter(value: &tera::Value, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    match value {
        tera::Value::Array(arr) => {
            let elements: Vec<String> = arr.iter()
                .map(|v| match v {
                    tera::Value::String(s) => format!("\"{}\"", s),
                    _ => format!("\"{}\"", v),
                })
                .collect();
            Ok(tera::Value::String(format!("({})", elements.join(" "))))
        }
        _ => Ok(value.clone())
    }
}

/// Generate nanosecond-precision ID
fn nanosecond_id_filter(value: &tera::Value, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let prefix = match value {
        tera::Value::String(s) => s.clone(),
        _ => "item".to_string(),
    };
    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    Ok(tera::Value::String(format!("{}_{}", prefix, timestamp)))
}
