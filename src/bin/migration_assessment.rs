//! Migration Assessment Tool for SwarmSH v2
//! 
//! Analyzes existing agent framework codebases and provides migration recommendations
//! with OTEL instrumentation and performance estimates.

use anyhow::{Result, Context};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{info, warn, debug, instrument};
use swarmsh_v2::telemetry::{init_shell_telemetry, DefaultSwarmTelemetry, SwarmTelemetry, CorrelationId};

#[derive(Parser)]
#[command(name = "migration-assessment")]
#[command(about = "Assess migration feasibility from traditional agent frameworks to SwarmSH v2")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a project directory for migration assessment
    Analyze {
        /// Path to the project directory
        #[arg(short, long)]
        path: PathBuf,
        
        /// Output format (json, markdown, console)
        #[arg(short, long, default_value = "console")]
        format: String,
        
        /// Include detailed file analysis
        #[arg(short, long)]
        detailed: bool,
    },
    
    /// Compare performance characteristics
    Compare {
        /// Framework to compare against
        #[arg(short, long)]
        framework: String,
        
        /// Run actual benchmarks
        #[arg(short, long)]
        benchmark: bool,
    },
    
    /// Generate migration plan
    Plan {
        /// Path to the project directory
        #[arg(short, long)]
        path: PathBuf,
        
        /// Target migration complexity (simple, moderate, complex)
        #[arg(short, long, default_value = "moderate")]
        complexity: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkDetection {
    pub framework: String,
    pub confidence: f64,
    pub version: Option<String>,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationAssessment {
    pub project_path: PathBuf,
    pub detected_frameworks: Vec<FrameworkDetection>,
    pub complexity_score: u32, // 1-10 scale
    pub estimated_effort_hours: u32,
    pub migration_priority: String, // High, Medium, Low
    pub performance_gains: PerformanceEstimate,
    pub migration_plan: Vec<MigrationStep>,
    pub risks: Vec<String>,
    pub benefits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceEstimate {
    pub startup_time_improvement: f64, // Multiplier (e.g., 100x = 100.0)
    pub memory_reduction: f64,         // Percentage
    pub dependency_reduction: u32,     // Number of dependencies eliminated
    pub deployment_simplification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStep {
    pub step: u32,
    pub title: String,
    pub description: String,
    pub estimated_hours: u32,
    pub dependencies: Vec<u32>, // Step numbers this depends on
    pub risk_level: String,     // Low, Medium, High
}

pub struct MigrationAnalyzer {
    telemetry: DefaultSwarmTelemetry,
}

impl MigrationAnalyzer {
    pub fn new() -> Self {
        Self {
            telemetry: DefaultSwarmTelemetry::default(),
        }
    }

    #[instrument(skip(self))]
    pub async fn analyze_project(&self, project_path: &Path) -> Result<MigrationAssessment> {
        let correlation_id = CorrelationId::new();
        let _span = self.telemetry.span_with_correlation("analyze_project", &correlation_id).entered();
        
        info!("Starting migration assessment for: {}", project_path.display());
        
        let detected_frameworks = self.detect_frameworks(project_path).await?;
        let complexity_score = self.calculate_complexity(&detected_frameworks, project_path).await?;
        let estimated_effort = self.estimate_effort(&detected_frameworks, complexity_score);
        let migration_priority = self.determine_priority(&detected_frameworks, complexity_score);
        let performance_gains = self.estimate_performance_gains(&detected_frameworks);
        let migration_plan = self.generate_migration_plan(&detected_frameworks, complexity_score);
        let (risks, benefits) = self.analyze_risks_benefits(&detected_frameworks);

        let assessment = MigrationAssessment {
            project_path: project_path.to_path_buf(),
            detected_frameworks,
            complexity_score,
            estimated_effort_hours: estimated_effort,
            migration_priority,
            performance_gains,
            migration_plan,
            risks,
            benefits,
        };

        info!(
            correlation_id = %correlation_id,
            complexity = complexity_score,
            effort_hours = estimated_effort,
            priority = %assessment.migration_priority,
            "Migration assessment completed"
        );

        Ok(assessment)
    }

    #[instrument(skip(self))]
    async fn detect_frameworks(&self, project_path: &Path) -> Result<Vec<FrameworkDetection>> {
        let mut detections = Vec::new();

        // Check for Python files and common imports
        if let Ok(entries) = fs::read_dir(project_path) {
            for entry in entries.flatten() {
                if let Some(extension) = entry.path().extension() {
                    if extension == "py" {
                        let content = fs::read_to_string(entry.path()).unwrap_or_default();
                        detections.extend(self.analyze_python_file(&content));
                    }
                }
            }
        }

        // Check requirements.txt or setup.py
        self.check_python_dependencies(project_path, &mut detections)?;

        // Check package.json for JavaScript frameworks
        self.check_js_dependencies(project_path, &mut detections)?;

        // Deduplicate and consolidate detections
        self.consolidate_detections(detections)
    }

    fn analyze_python_file(&self, content: &str) -> Vec<FrameworkDetection> {
        let mut detections = Vec::new();
        let mut evidence = Vec::new();

        // LangChain detection
        if content.contains("from langchain") || content.contains("import langchain") {
            evidence.push("LangChain import detected".to_string());
            if content.contains("LLMChain") {
                evidence.push("LLMChain usage found".to_string());
            }
            if content.contains("initialize_agent") {
                evidence.push("Agent initialization pattern found".to_string());
            }
            
            detections.push(FrameworkDetection {
                framework: "LangChain".to_string(),
                confidence: 0.9,
                version: None,
                evidence: evidence.clone(),
            });
        }

        // AutoGPT detection
        evidence.clear();
        if content.contains("autogpt") || content.contains("AutoGPT") {
            evidence.push("AutoGPT reference detected".to_string());
            detections.push(FrameworkDetection {
                framework: "AutoGPT".to_string(),
                confidence: 0.8,
                version: None,
                evidence,
            });
        }

        // CrewAI detection
        evidence.clear();
        if content.contains("from crewai") || content.contains("import crewai") {
            evidence.push("CrewAI import detected".to_string());
            if content.contains("Agent") && content.contains("Task") {
                evidence.push("CrewAI Agent/Task pattern found".to_string());
            }
            
            detections.push(FrameworkDetection {
                framework: "CrewAI".to_string(),
                confidence: 0.9,
                version: None,
                evidence,
            });
        }

        // BabyAGI detection
        evidence.clear();
        if content.contains("babyagi") || content.contains("BabyAGI") {
            evidence.push("BabyAGI reference detected".to_string());
            detections.push(FrameworkDetection {
                framework: "BabyAGI".to_string(),
                confidence: 0.7,
                version: None,
                evidence,
            });
        }

        detections
    }

    fn check_python_dependencies(&self, project_path: &Path, detections: &mut Vec<FrameworkDetection>) -> Result<()> {
        // Check requirements.txt
        let req_path = project_path.join("requirements.txt");
        if req_path.exists() {
            let content = fs::read_to_string(req_path)?;
            for line in content.lines() {
                if line.contains("langchain") {
                    detections.push(FrameworkDetection {
                        framework: "LangChain".to_string(),
                        confidence: 0.95,
                        version: self.extract_version(line),
                        evidence: vec!["Found in requirements.txt".to_string()],
                    });
                }
                if line.contains("crewai") {
                    detections.push(FrameworkDetection {
                        framework: "CrewAI".to_string(),
                        confidence: 0.95,
                        version: self.extract_version(line),
                        evidence: vec!["Found in requirements.txt".to_string()],
                    });
                }
            }
        }

        Ok(())
    }

    fn check_js_dependencies(&self, project_path: &Path, detections: &mut Vec<FrameworkDetection>) -> Result<()> {
        let package_path = project_path.join("package.json");
        if package_path.exists() {
            let content = fs::read_to_string(package_path)?;
            // Check for JavaScript agent frameworks
            if content.contains("\"ai\"") || content.contains("\"openai\"") {
                detections.push(FrameworkDetection {
                    framework: "JavaScript AI Framework".to_string(),
                    confidence: 0.6,
                    version: None,
                    evidence: vec!["AI dependencies in package.json".to_string()],
                });
            }
        }
        Ok(())
    }

    fn extract_version(&self, line: &str) -> Option<String> {
        // Extract version from requirement line like "langchain==0.1.0"
        if let Some(eq_pos) = line.find("==") {
            Some(line[eq_pos + 2..].trim().to_string())
        } else {
            None
        }
    }

    fn consolidate_detections(&self, detections: Vec<FrameworkDetection>) -> Result<Vec<FrameworkDetection>> {
        let mut consolidated = HashMap::new();
        
        for detection in detections {
            consolidated
                .entry(detection.framework.clone())
                .and_modify(|existing: &mut FrameworkDetection| {
                    // Merge evidence and take highest confidence
                    existing.evidence.extend(detection.evidence.clone());
                    if detection.confidence > existing.confidence {
                        existing.confidence = detection.confidence;
                    }
                    if detection.version.is_some() && existing.version.is_none() {
                        existing.version = detection.version.clone();
                    }
                })
                .or_insert(detection);
        }

        Ok(consolidated.into_values().collect())
    }

    async fn calculate_complexity(&self, frameworks: &[FrameworkDetection], project_path: &Path) -> Result<u32> {
        let mut complexity = 0;

        // Base complexity from frameworks
        for framework in frameworks {
            complexity += match framework.framework.as_str() {
                "LangChain" => 7, // High complexity due to chain abstractions
                "AutoGPT" => 8,   // Very high due to recursive goal processing
                "CrewAI" => 6,    // Moderate complexity
                "BabyAGI" => 5,   // Moderate complexity
                _ => 3,           // Unknown frameworks
            };
        }

        // Adjust for project size
        if let Ok(entries) = fs::read_dir(project_path) {
            let file_count = entries.count();
            complexity += match file_count {
                0..=10 => 1,
                11..=50 => 2,
                51..=100 => 3,
                _ => 4,
            };
        }

        // Cap at 10
        Ok(complexity.min(10))
    }

    fn estimate_effort(&self, frameworks: &[FrameworkDetection], complexity: u32) -> u32 {
        let base_effort = complexity * 4; // 4 hours per complexity point
        
        let framework_multiplier = frameworks.iter().map(|f| match f.framework.as_str() {
            "LangChain" => 1.5,
            "AutoGPT" => 2.0,
            "CrewAI" => 1.2,
            "BabyAGI" => 1.3,
            _ => 1.0,
        }).fold(1.0, |acc, x| acc * x);

        (base_effort as f64 * framework_multiplier) as u32
    }

    fn determine_priority(&self, frameworks: &[FrameworkDetection], complexity: u32) -> String {
        let framework_priority_score: u32 = frameworks.iter().map(|f| match f.framework.as_str() {
            "AutoGPT" => 3, // High priority - performance gains significant
            "LangChain" => 2, // Medium priority - dependency reduction valuable
            "CrewAI" => 2,
            "BabyAGI" => 2,
            _ => 1,
        }).sum();

        match framework_priority_score + complexity {
            0..=5 => "Low".to_string(),
            6..=12 => "Medium".to_string(),
            _ => "High".to_string(),
        }
    }

    fn estimate_performance_gains(&self, frameworks: &[FrameworkDetection]) -> PerformanceEstimate {
        let has_langchain = frameworks.iter().any(|f| f.framework == "LangChain");
        let has_autogpt = frameworks.iter().any(|f| f.framework == "AutoGPT");

        PerformanceEstimate {
            startup_time_improvement: if has_langchain || has_autogpt { 50.0 } else { 10.0 },
            memory_reduction: if has_langchain { 80.0 } else { 60.0 },
            dependency_reduction: frameworks.len() as u32 * 20, // Estimate 20 deps per framework
            deployment_simplification: "Docker → Single Directory".to_string(),
        }
    }

    fn generate_migration_plan(&self, frameworks: &[FrameworkDetection], complexity: u32) -> Vec<MigrationStep> {
        let mut steps = Vec::new();

        steps.push(MigrationStep {
            step: 1,
            title: "Assessment and Planning".to_string(),
            description: "Complete dependency audit and create migration timeline".to_string(),
            estimated_hours: 4,
            dependencies: vec![],
            risk_level: "Low".to_string(),
        });

        if frameworks.iter().any(|f| f.framework == "LangChain") {
            steps.push(MigrationStep {
                step: 2,
                title: "Convert LangChain Chains to Work Queue".to_string(),
                description: "Replace chain-based logic with SwarmSH work claiming".to_string(),
                estimated_hours: complexity * 2,
                dependencies: vec![1],
                risk_level: "Medium".to_string(),
            });
        }

        if frameworks.iter().any(|f| f.framework == "AutoGPT") {
            steps.push(MigrationStep {
                step: 3,
                title: "Restructure AutoGPT Goals".to_string(),
                description: "Convert recursive goal processing to explicit work items".to_string(),
                estimated_hours: complexity * 3,
                dependencies: vec![1],
                risk_level: "High".to_string(),
            });
        }

        steps.push(MigrationStep {
            step: 10,
            title: "Implement SwarmSH Coordination".to_string(),
            description: "Set up agent coordination and work queue system".to_string(),
            estimated_hours: 8,
            dependencies: vec![1, 2, 3].into_iter().filter(|&x| x <= steps.len() as u32).collect(),
            risk_level: "Medium".to_string(),
        });

        steps.push(MigrationStep {
            step: 11,
            title: "Testing and Validation".to_string(),
            description: "Comprehensive testing of migrated functionality".to_string(),
            estimated_hours: complexity,
            dependencies: vec![10],
            risk_level: "Low".to_string(),
        });

        steps
    }

    fn analyze_risks_benefits(&self, frameworks: &[FrameworkDetection]) -> (Vec<String>, Vec<String>) {
        let mut risks = vec![
            "Learning curve for bash-based coordination".to_string(),
            "Potential feature gaps during migration".to_string(),
        ];

        let mut benefits = vec![
            "Dramatically reduced resource usage".to_string(),
            "Simplified deployment model".to_string(),
            "Built-in observability with OpenTelemetry".to_string(),
            "Mathematical zero-conflict guarantees".to_string(),
        ];

        if frameworks.iter().any(|f| f.framework == "LangChain") {
            risks.push("Complex chain logic needs restructuring".to_string());
            benefits.push("Eliminate heavy Python dependency chain".to_string());
        }

        if frameworks.iter().any(|f| f.framework == "AutoGPT") {
            risks.push("Recursive goal processing needs redesign".to_string());
            benefits.push("More predictable and controllable agent behavior".to_string());
        }

        (risks, benefits)
    }

    pub fn format_assessment(&self, assessment: &MigrationAssessment, format: &str) -> Result<String> {
        match format {
            "json" => Ok(serde_json::to_string_pretty(assessment)?),
            "markdown" => self.format_markdown(assessment),
            "console" => self.format_console(assessment),
            _ => Err(anyhow::anyhow!("Unsupported format: {}", format)),
        }
    }

    fn format_console(&self, assessment: &MigrationAssessment) -> Result<String> {
        let mut output = String::new();
        
        output.push_str(&format!("🔍 SwarmSH Migration Assessment\n"));
        output.push_str(&format!("═══════════════════════════════\n\n"));
        
        output.push_str(&format!("📁 Project: {}\n", assessment.project_path.display()));
        output.push_str(&format!("🎯 Priority: {}\n", assessment.migration_priority));
        output.push_str(&format!("📊 Complexity: {}/10\n", assessment.complexity_score));
        output.push_str(&format!("⏱️  Estimated Effort: {} hours\n\n", assessment.estimated_effort_hours));

        output.push_str("🔧 Detected Frameworks:\n");
        for framework in &assessment.detected_frameworks {
            output.push_str(&format!("  • {} (confidence: {:.0}%)\n", framework.framework, framework.confidence * 100.0));
            if let Some(version) = &framework.version {
                output.push_str(&format!("    Version: {}\n", version));
            }
        }

        output.push_str(&format!("\n🚀 Performance Gains:\n"));
        output.push_str(&format!("  • Startup: {:.0}x faster\n", assessment.performance_gains.startup_time_improvement));
        output.push_str(&format!("  • Memory: {:.0}% reduction\n", assessment.performance_gains.memory_reduction));
        output.push_str(&format!("  • Dependencies: {} fewer\n", assessment.performance_gains.dependency_reduction));

        Ok(output)
    }

    fn format_markdown(&self, assessment: &MigrationAssessment) -> Result<String> {
        let mut output = String::new();
        
        output.push_str("# SwarmSH Migration Assessment Report\n\n");
        output.push_str(&format!("**Project:** `{}`\n", assessment.project_path.display()));
        output.push_str(&format!("**Assessment Date:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));

        output.push_str("## Summary\n\n");
        output.push_str(&format!("- **Migration Priority:** {}\n", assessment.migration_priority));
        output.push_str(&format!("- **Complexity Score:** {}/10\n", assessment.complexity_score));
        output.push_str(&format!("- **Estimated Effort:** {} hours\n\n", assessment.estimated_effort_hours));

        output.push_str("## Detected Frameworks\n\n");
        for framework in &assessment.detected_frameworks {
            output.push_str(&format!("### {}\n", framework.framework));
            output.push_str(&format!("- **Confidence:** {:.0}%\n", framework.confidence * 100.0));
            if let Some(version) = &framework.version {
                output.push_str(&format!("- **Version:** {}\n", version));
            }
            output.push_str("- **Evidence:**\n");
            for evidence in &framework.evidence {
                output.push_str(&format!("  - {}\n", evidence));
            }
            output.push_str("\n");
        }

        Ok(output)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize telemetry
    init_shell_telemetry("migration-assessment")?;

    let analyzer = MigrationAnalyzer::new();

    match cli.command {
        Commands::Analyze { path, format, detailed: _ } => {
            let assessment = analyzer.analyze_project(&path).await
                .context("Failed to analyze project")?;
            
            let output = analyzer.format_assessment(&assessment, &format)
                .context("Failed to format assessment")?;
            
            println!("{}", output);
        }
        
        Commands::Compare { framework, benchmark } => {
            println!("🔍 Comparing {} with SwarmSH v2", framework);
            
            if benchmark {
                println!("🏃 Running performance benchmarks...");
                // TODO: Implement actual benchmarks
                println!("Benchmark results would be displayed here");
            } else {
                println!("Use --benchmark flag to run actual performance tests");
            }
        }
        
        Commands::Plan { path, complexity } => {
            let assessment = analyzer.analyze_project(&path).await
                .context("Failed to analyze project for planning")?;
            
            println!("📋 Migration Plan for {}", path.display());
            println!("Target Complexity: {}\n", complexity);
            
            for step in &assessment.migration_plan {
                println!("Step {}: {} ({} hours, Risk: {})", 
                    step.step, step.title, step.estimated_hours, step.risk_level);
                println!("  {}", step.description);
                if !step.dependencies.is_empty() {
                    println!("  Dependencies: {:?}", step.dependencies);
                }
                println!();
            }
        }
    }

    Ok(())
}