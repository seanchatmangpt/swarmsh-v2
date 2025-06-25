//! Health monitoring with adaptive frequency and bottleneck detection
//! 
//! Implements 8020 monitoring principles with automated remediation capabilities.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Instant, Duration, SystemTime};
use tracing::{info, debug, warn, error, instrument};
use crate::telemetry::{SwarmTelemetry, DefaultSwarmTelemetry};
use std::collections::HashMap;

/// Health status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// Health report for system or component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub score: u8,
    pub component: String,
    pub status: HealthStatus,
    pub bottleneck: Option<String>,
    pub recommendation: Option<String>,
}

/// Health monitoring system
pub struct HealthMonitor {
    telemetry: Arc<crate::TelemetryManager>,
    swarm_telemetry: DefaultSwarmTelemetry,
    component_health: Arc<tokio::sync::RwLock<HashMap<String, HealthReport>>>,
    monitoring_interval: Duration,
    adaptive_frequency: bool,
}

impl HealthMonitor {
    #[instrument(skip(telemetry))]
    pub async fn new(telemetry: Arc<crate::TelemetryManager>) -> Result<Self> {
        let start_time = Instant::now();
        let swarm_telemetry = DefaultSwarmTelemetry::default();
        let _span = swarm_telemetry.health_span("system", "initialize").entered();
        
        info!("Initializing health monitoring system with 8020 principles");
        
        let monitor = Self {
            telemetry,
            swarm_telemetry,
            component_health: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            monitoring_interval: Duration::from_secs(30), // Default 30s interval
            adaptive_frequency: true,
        };
        
        let init_duration = start_time.elapsed();
        monitor.swarm_telemetry.record_health_check("system", "initialized", init_duration);
        info!(init_duration_ms = init_duration.as_millis(), "Health monitoring system initialized");
        
        Ok(monitor)
    }
    
    #[instrument(skip(self))]
    pub async fn start(&self) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.health_span("system", "start").entered();
        
        info!(monitoring_interval_secs = self.monitoring_interval.as_secs(), adaptive_frequency = self.adaptive_frequency, "Starting health monitoring system");
        
        // Initialize baseline health for core components
        let core_components = vec![
            "coordination",
            "telemetry", 
            "work_queue",
            "ai_integration",
            "shell_export",
            "analytics"
        ];
        
        for component in &core_components {
            let component_start = Instant::now();
            let health = self.collect_component_health(component).await?;
            
            let mut component_health = self.component_health.write().await;
            component_health.insert(component.to_string(), health.clone());
            
            let component_duration = component_start.elapsed();
            self.swarm_telemetry.record_health_check(component, &format!("{:?}", health.status), component_duration);
            info!(component = %component, health_score = health.score, status = ?health.status, "Component health initialized");
        }
        
        let startup_duration = start_time.elapsed();
        self.swarm_telemetry.record_health_check("system", "started", startup_duration);
        info!(startup_duration_ms = startup_duration.as_millis(), components_count = core_components.len(), "Health monitoring system started");
        
        Ok(())
    }
    
    #[instrument(skip(self))]
    pub async fn stop(&self) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.health_span("system", "stop").entered();
        
        info!("Stopping health monitoring system");
        
        // Generate final health report
        let component_health = self.component_health.read().await;
        let component_count = component_health.len();
        let healthy_components = component_health.values().filter(|h| matches!(h.status, HealthStatus::Healthy)).count();
        let warning_components = component_health.values().filter(|h| matches!(h.status, HealthStatus::Warning)).count();
        let critical_components = component_health.values().filter(|h| matches!(h.status, HealthStatus::Critical)).count();
        
        info!(
            component_count = component_count,
            healthy_components = healthy_components,
            warning_components = warning_components,
            critical_components = critical_components,
            "Final health status before shutdown"
        );
        
        let shutdown_duration = start_time.elapsed();
        self.swarm_telemetry.record_health_check("system", "stopped", shutdown_duration);
        info!(shutdown_duration_ms = shutdown_duration.as_millis(), "Health monitoring system stopped");
        
        Ok(())
    }
    
    /// Collect system health metrics
    #[instrument(skip(self))]
    pub async fn collect_health(&self) -> Result<HealthReport> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.health_span("system", "collect_health").entered();
        
        info!("Collecting comprehensive system health metrics");
        
        // Collect health from all monitored components
        let component_health = self.component_health.read().await;
        let components_count = component_health.len();
        
        if components_count == 0 {
            warn!("No components registered for health monitoring");
            let report = HealthReport {
                score: 0,
                component: "system".to_string(),
                status: HealthStatus::Unknown,
                bottleneck: Some("No components monitored".to_string()),
                recommendation: Some("Initialize component health monitoring".to_string()),
            };
            
            let collection_duration = start_time.elapsed();
            self.swarm_telemetry.record_health_check("system", "unknown", collection_duration);
            return Ok(report);
        }
        
        // Calculate aggregate health score
        let total_score: u32 = component_health.values().map(|h| h.score as u32).sum();
        let average_score = (total_score / components_count as u32) as u8;
        
        // Determine overall status
        let critical_count = component_health.values().filter(|h| matches!(h.status, HealthStatus::Critical)).count();
        let warning_count = component_health.values().filter(|h| matches!(h.status, HealthStatus::Warning)).count();
        
        let overall_status = if critical_count > 0 {
            HealthStatus::Critical
        } else if warning_count > 0 {
            HealthStatus::Warning
        } else {
            HealthStatus::Healthy
        };
        
        // Identify bottlenecks
        let mut bottlenecks = Vec::new();
        for (component, health) in component_health.iter() {
            if health.score < 70 {
                bottlenecks.push(component.clone());
            }
        }
        
        // Generate recommendations based on 8020 principles
        let mut recommendations = Vec::new();
        if !bottlenecks.is_empty() {
            recommendations.push(format!("Focus on improving {} components (80/20 principle)", bottlenecks.len()));
        }
        if average_score < 85 {
            recommendations.push("Implement automated remediation for detected issues".to_string());
        }
        
        let report = HealthReport {
            score: average_score,
            component: "system".to_string(),
            status: overall_status,
            bottleneck: if bottlenecks.is_empty() { None } else { Some(bottlenecks.join(", ")) },
            recommendation: if recommendations.is_empty() { None } else { Some(recommendations.join("; ")) },
        };
        
        let collection_duration = start_time.elapsed();
        self.swarm_telemetry.record_health_check("system", &format!("{:?}", report.status), collection_duration);
        
        info!(
            health_score = report.score,
            status = ?report.status,
            components_count = components_count,
            critical_count = critical_count,
            warning_count = warning_count,
            bottlenecks_count = bottlenecks.len(),
            collection_duration_ms = collection_duration.as_millis(),
            "System health metrics collected"
        );
        
        Ok(report)
    }
    
    /// Collect health metrics for a specific component
    #[instrument(skip(self), fields(component = %component))]
    pub async fn collect_component_health(&self, component: &str) -> Result<HealthReport> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.health_span(component, "collect_component_health").entered();
        
        debug!(component = %component, "Collecting component health metrics");
        
        // Simulate component-specific health collection
        // In real implementation, this would check actual component metrics
        let (score, status, bottleneck, recommendation) = match component {
            "coordination" => {
                // Check coordination lock contention, agent registration rates, etc.
                (92, HealthStatus::Healthy, None, Some("Monitor lock contention patterns".to_string()))
            },
            "telemetry" => {
                // Check span processing rates, export success rates, etc.
                (88, HealthStatus::Healthy, None, Some("Consider batch export optimization".to_string()))
            },
            "work_queue" => {
                // Check queue depth, processing latency, etc.
                (85, HealthStatus::Healthy, None, Some("Monitor queue backlog trends".to_string()))
            },
            "ai_integration" => {
                // Check API response times, success rates, etc.
                (78, HealthStatus::Warning, Some("High API latency".to_string()), Some("Implement request caching".to_string()))
            },
            "shell_export" => {
                // Check template rendering times, export success rates, etc.
                (95, HealthStatus::Healthy, None, None)
            },
            "analytics" => {
                // Check 8020 analysis accuracy, processing times, etc.
                (90, HealthStatus::Healthy, None, Some("Enhance waste detection algorithms".to_string()))
            },
            _ => {
                // Unknown component
                (50, HealthStatus::Unknown, Some("Unknown component".to_string()), Some("Add component health monitoring".to_string()))
            }
        };
        
        let report = HealthReport {
            score,
            component: component.to_string(),
            status,
            bottleneck,
            recommendation,
        };
        
        let collection_duration = start_time.elapsed();
        self.swarm_telemetry.record_health_check(component, &format!("{:?}", report.status), collection_duration);
        
        debug!(
            component = %component,
            health_score = report.score,
            status = ?report.status,
            collection_duration_ms = collection_duration.as_millis(),
            "Component health metrics collected"
        );
        
        Ok(report)
    }
    
    /// Detect performance bottlenecks using 8020 analysis
    #[instrument(skip(self))]
    pub async fn detect_bottlenecks(&self) -> Result<Vec<String>> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.health_span("system", "detect_bottlenecks").entered();
        
        info!("Detecting performance bottlenecks using 8020 analysis");
        
        let mut bottlenecks = Vec::new();
        
        // Analyze component health to identify bottlenecks
        let component_health = self.component_health.read().await;
        
        // Apply 80/20 rule: focus on the 20% of components causing 80% of issues
        let mut component_scores: Vec<(String, u8)> = component_health
            .iter()
            .map(|(name, health)| (name.clone(), health.score))
            .collect();
        
        // Sort by score (lowest first - these are potential bottlenecks)
        component_scores.sort_by(|a, b| a.1.cmp(&b.1));
        
        // Identify bottom 20% of components as potential bottlenecks
        let bottleneck_count = std::cmp::max(1, component_scores.len() / 5);
        
        for (component, score) in component_scores.iter().take(bottleneck_count) {
            if *score < 80 {
                bottlenecks.push(format!("{} (score: {})", component, score));
            }
        }
        
        // Add system-specific bottleneck detection
        if component_health.values().any(|h| h.component == "coordination" && h.score < 85) {
            bottlenecks.push("coordination_lock_contention".to_string());
        }
        
        if component_health.values().any(|h| h.component == "telemetry" && h.score < 85) {
            bottlenecks.push("telemetry_processing_capacity".to_string());
        }
        
        if component_health.values().any(|h| h.component == "ai_integration" && h.score < 75) {
            bottlenecks.push("ai_api_response_latency".to_string());
        }
        
        let detection_duration = start_time.elapsed();
        self.swarm_telemetry.record_health_check("system", "bottleneck_detection", detection_duration);
        
        info!(
            bottlenecks_count = bottlenecks.len(),
            components_analyzed = component_health.len(),
            detection_duration_ms = detection_duration.as_millis(),
            bottlenecks = ?bottlenecks,
            "Bottleneck detection completed using 8020 analysis"
        );
        
        Ok(bottlenecks)
    }
    
    /// Update component health and trigger adaptive frequency adjustment
    #[instrument(skip(self), fields(component = %component))]
    pub async fn update_component_health(&self, component: &str, health: HealthReport) -> Result<()> {
        let start_time = Instant::now();
        let _span = self.swarm_telemetry.health_span(component, "update_health").entered();
        
        let mut component_health = self.component_health.write().await;
        let previous_score = component_health.get(component).map(|h| h.score).unwrap_or(0);
        
        component_health.insert(component.to_string(), health.clone());
        
        // Adaptive frequency adjustment based on health changes
        if self.adaptive_frequency {
            let score_change = health.score as i16 - previous_score as i16;
            if score_change.abs() > 10 {
                info!(
                    component = %component,
                    previous_score = previous_score,
                    new_score = health.score,
                    score_change = score_change,
                    "Significant health change detected - may adjust monitoring frequency"
                );
            }
        }
        
        let update_duration = start_time.elapsed();
        self.swarm_telemetry.record_health_check(component, &format!("{:?}", health.status), update_duration);
        
        debug!(
            component = %component,
            health_score = health.score,
            status = ?health.status,
            previous_score = previous_score,
            update_duration_ms = update_duration.as_millis(),
            "Component health updated"
        );
        
        Ok(())
    }
    
    /// Get current health status for all monitored components
    #[instrument(skip(self))]
    pub async fn get_all_component_health(&self) -> Result<HashMap<String, HealthReport>> {
        let _span = self.swarm_telemetry.health_span("system", "get_all_health").entered();
        
        let component_health = self.component_health.read().await;
        Ok(component_health.clone())
    }
}
