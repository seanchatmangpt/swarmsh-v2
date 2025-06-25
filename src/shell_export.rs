//! Shell export system - Export Rust implementation to shell scripts
//! 
//! Provides complete functionality as optimized shell scripts for UNIX deployment.
//! Maintains all coordination guarantees while enabling shell-only execution.
//! Uses minijinja templating engine for powerful template generation.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use minijinja::{Environment, Value, context};
use std::collections::HashMap;
use std::sync::Arc;
use crate::ai_integration::AIIntegration;
use tracing::{info, debug, warn, error, instrument};
use crate::telemetry::{SwarmTelemetry, DefaultSwarmTelemetry};
use std::time::Instant;
use std::fs;

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
    /// minijinja template engine for generating shell scripts
    env: Environment<'static>,
    /// Template directory path
    template_dir: PathBuf,
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
        
        info!("Initializing shell exporter with minijinja templating engine");
        
        let template_dir = PathBuf::from("templates");
        
        // Initialize minijinja environment
        let mut env = Environment::new();
        env.set_auto_escape_callback(|_| minijinja::AutoEscape::None);
        
        // Add custom filters for shell script generation
        env.add_filter("shell_escape", shell_escape_filter);
        env.add_filter("to_bash_array", bash_array_filter);
        env.add_filter("nanosecond_id", nanosecond_id_filter);
        
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
        
        Ok(Self { env, template_dir, ai_integration, swarm_telemetry })
    }
    
    /// Helper method to render templates with minijinja
    fn render_template(&self, template_name: &str, context: &Value) -> Result<String> {
        let template_path = self.template_dir.join(template_name);
        let template_content = fs::read_to_string(&template_path)
            .map_err(|e| anyhow::anyhow!("Failed to read template {}: {}", template_name, e))?;
        
        let template = self.env.template_from_str(&template_content)
            .map_err(|e| anyhow::anyhow!("Failed to compile template {}: {}", template_name, e))?;
        
        template.render(context)
            .map_err(|e| anyhow::anyhow!("Failed to render template {}: {}", template_name, e))
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
        let render_context = context! {
            config => config,
            timestamp => chrono::Utc::now().to_rfc3339(),
            version => "2.0.0"
        };
        
        // Export each component using minijinja templates
        self.export_coordination_with_template(&config, &render_context).await?;
        self.export_telemetry_with_template(&config, &render_context).await?;
        self.export_health_monitoring_with_template(&config, &render_context).await?;
        self.export_analytics_with_template(&config, &render_context).await?;
        
        if config.include_ai_integration {
            let ai_start = Instant::now();
            self.export_ai_integration_with_template(&config, &render_context).await?;
            let ai_duration = ai_start.elapsed();
            info!(ai_export_duration_ms = ai_duration.as_millis(), "AI integration export completed");
        }
        
        let total_duration = start_time.elapsed();
        info!(
            total_export_duration_ms = total_duration.as_millis(),
            telemetry_included = config.include_telemetry,
            ai_included = config.include_ai_integration,
            "Shell export completed successfully using minijinja templating"
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
        let empty_context = context! {};
        self.export_coordination_with_template(config, &empty_context).await
    }
    
    async fn export_telemetry(&self, config: &ExportConfig) -> Result<()> {
        let empty_context = context! {};
        self.export_telemetry_with_template(config, &empty_context).await
    }
    
    async fn export_health_monitoring(&self, config: &ExportConfig) -> Result<()> {
        let empty_context = context! {};
        self.export_health_monitoring_with_template(config, &empty_context).await
    }
    
    async fn export_analytics(&self, config: &ExportConfig) -> Result<()> {
        let empty_context = context! {};
        self.export_analytics_with_template(config, &empty_context).await
    }
    
    async fn export_ai_integration(&self, config: &ExportConfig) -> Result<()> {
        let empty_context = context! {};
        self.export_ai_integration_with_template(config, &empty_context).await
    }
    
    // minijinja template-based export methods with AI enhancement
    #[instrument(skip(self, config, context))]
    async fn export_coordination_with_template(&self, config: &ExportConfig, context: &Value) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.coordination_span("shell_export", "export_coordination").entered();
        
        info!("Generating coordination shell scripts with AI-enhanced minijinja templates");
        
        let mut template_context = context.clone();
        let additional_context = context! {
            coordination_patterns => vec![
                "scrum_at_scale",
                "roberts_rules", 
                "realtime",
                "atomic"
            ],
            nanosecond_precision => true,
            zero_conflict_guarantee => true
        };
        
        // Merge contexts (simplified approach)
        let template_context = additional_context;
        
        // Get AI enhancement for coordination
        let enhanced_context = self.get_ai_enhanced_context(&template_context, "coordination").await?;
        
        // Render and optimize coordination helper
        let coordination_script = self.render_template("coordination_helper.sh.tera", &enhanced_context)?;
        let optimized_script = self.optimize_shell_script(
            &coordination_script,
            "Zero-conflict agent coordination with nanosecond precision"
        ).await?;
        let output_path = config.output_dir.join("coordination_helper.sh");
        std::fs::write(output_path, optimized_script)?;
        
        // Render and optimize agent orchestrator
        let orchestrator_script = self.render_template("agent_swarm_orchestrator.sh.tera", &enhanced_context)?;
        let optimized_orchestrator = self.optimize_shell_script(
            &orchestrator_script,
            "Agent swarm orchestration with intelligent work distribution"
        ).await?;
        let output_path = config.output_dir.join("agent_swarm_orchestrator.sh");
        std::fs::write(output_path, optimized_orchestrator)?;
        
        // Render and optimize real agent coordinator
        let coordinator_script = self.render_template("real_agent_coordinator.sh.tera", &enhanced_context)?;
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
    async fn export_telemetry_with_template(&self, config: &ExportConfig, context: &Value) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.coordination_span("shell_export", "export_telemetry").entered();
        
        info!("Generating telemetry shell scripts with minijinja templates");
        
        if !config.include_telemetry {
            return Ok(());
        }
        
        let template_context = context! {
            semantic_conventions => vec![
                "swarmsh.agent",
                "swarmsh.work",
                "swarmsh.coordination", 
                "swarmsh.health",
                "swarmsh.analytics"
            ],
            otel_export_format => "json"
        };
        
        let telemetry_script = self.render_template("telemetry_spans.sh.tera", &template_context)?;
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
    async fn export_health_monitoring_with_template(&self, config: &ExportConfig, context: &Value) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.coordination_span("shell_export", "export_health").entered();
        
        info!("Generating health monitoring shell scripts with minijinja templates");
        
        let template_context = context! {
            monitoring_tiers => vec!["tier1", "tier2"],
            health_components => vec![
                "coordination",
                "telemetry", 
                "automation",
                "ai",
                "work_queue",
                "storage"
            ],
            automated_remediation => true
        };
        
        let health_script = self.render_template("health_monitor.sh.tera", &template_context)?;
        let output_path = config.output_dir.join("health_monitor.sh");
        std::fs::write(output_path, health_script)?;
        
        let health_duration = start_time.elapsed();
        info!(
            health_export_duration_ms = health_duration.as_millis(),
            monitoring_components = 6,
            "Health monitoring shell scripts generated successfully"
        );
        
        Ok(())
    }
    
    #[instrument(skip(self, config, context))]
    async fn export_analytics_with_template(&self, config: &ExportConfig, context: &Value) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.coordination_span("shell_export", "export_analytics").entered();
        
        info!("Generating analytics shell scripts with minijinja templates");
        
        let template_context = context! {
            optimization_targets => context! {
                waste_elimination => 73.0,
                flow_efficiency => 84.0,
                lead_time_ms => 126000,
                sigma_level => 4.2
            },
            dlss_principles => vec![
                "overproduction",
                "waiting",
                "transport", 
                "over_processing",
                "inventory",
                "motion",
                "defects"
            ]
        };
        
        let analytics_script = self.render_template("8020_automation.sh.tera", &template_context)?;
        let output_path = config.output_dir.join("8020_automation.sh");
        std::fs::write(output_path, analytics_script)?;
        
        let analytics_duration = start_time.elapsed();
        info!(
            analytics_export_duration_ms = analytics_duration.as_millis(),
            dlss_principles_count = 7,
            "Analytics shell scripts generated successfully"
        );
        
        Ok(())
    }
    
    #[instrument(skip(self, config, context))]
    async fn export_ai_integration_with_template(&self, config: &ExportConfig, context: &Value) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.coordination_span("shell_export", "export_ai_integration").entered();
        
        info!("Generating AI integration shell scripts with AI-enhanced minijinja templates");
        
        let template_context = context! {
            ai_providers => vec!["claude", "ollama"],
            confidence_threshold => 0.8,
            fallback_to_human => true,
            ollama_features => context! {
                chat_completion => true,
                embeddings => true,
                streaming => true,
                model_management => true,
                pattern_analysis => true,
                decision_making => true,
                shell_optimization => true
            }
        };
        
        // Get AI enhancement for the AI integration component itself
        let enhanced_context = self.get_ai_enhanced_context(&template_context, "ai_integration").await?;
        
        // Claude integration with optimization
        let claude_script = self.render_template("claude_integration.sh.tera", &enhanced_context)?;
        let optimized_claude = self.optimize_shell_script(
            &claude_script,
            "Claude API integration for comprehensive analysis and planning"
        ).await?;
        let output_path = config.output_dir.join("claude_integration.sh");
        std::fs::write(output_path, optimized_claude)?;
        
        // Ollama integration with full feature optimization
        let ollama_script = self.render_template("ollama_integration.sh.tera", &enhanced_context)?;
        let optimized_ollama = self.optimize_shell_script(
            &ollama_script,
            "Local Ollama integration with chat, embeddings, streaming, and model management"
        ).await?;
        let output_path = config.output_dir.join("ollama_integration.sh");
        std::fs::write(output_path, optimized_ollama)?;
        
        // Generate AI-powered shell utilities
        let ai_utils_script = self.render_template("ai_shell_utils.sh.tera", &enhanced_context)?;
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
            ai_providers_count = 2,
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
    async fn get_ai_enhanced_context(&self, base_context: &Value, component: &str) -> Result<Value> {
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
                    enhanced_context = context! {
                        ai_recommendations => analysis.recommendations,
                        ai_confidence => analysis.confidence,
                        optimization_opportunities => analysis.optimization_opportunities,
                        ai_reasoning => analysis.reasoning.unwrap_or_default()
                    };
                    
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

// minijinja custom filters for shell script generation

/// Shell escape filter for safe string interpolation
fn shell_escape_filter(value: String) -> String {
    let escaped = value.replace('\\', "\\\\")
                      .replace('\"', "\\\"")
                      .replace('$', "\\$")
                      .replace('`', "\\`");
    format!("\"{}\"", escaped)
}

/// Convert array to bash array format
fn bash_array_filter(value: Vec<String>) -> String {
    let elements: Vec<String> = value.iter()
        .map(|s| format!("\"{}\"", s))
        .collect();
    format!("({})", elements.join(" "))
}

/// Generate nanosecond-precision ID
fn nanosecond_id_filter(prefix: String) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    format!("{}_{}", prefix, timestamp)
}
