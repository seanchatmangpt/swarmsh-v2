//! Optional AI integration for SwarmSH v2.
//!
//! AI is an advisory surface. Core coordination does not require network access,
//! and constructing this module does not probe external services.

use anyhow::{bail, Context, Result};
use ollama_rs::{
    generation::{
        chat::{request::ChatMessageRequest, ChatMessage, MessageRole},
        completion::request::GenerationRequest,
        embeddings::{request::GenerateEmbeddingsRequest, GenerateEmbeddingsResponse},
    },
    Ollama,
};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;
use tracing::{debug, instrument};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAnalysis {
    pub recommendations: Vec<String>,
    pub confidence: f64,
    pub optimization_opportunities: Vec<String>,
    pub reasoning: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDecision {
    pub action: String,
    pub parameters: serde_json::Value,
    pub confidence: f64,
    pub alternatives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternSimilarity {
    pub pattern: String,
    pub similarity_score: f64,
    pub embeddings: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}

/// Claude configuration holder.
///
/// A concrete Claude transport is not implemented in this crate. Presence of an
/// endpoint therefore never creates synthetic AI output.
#[derive(Debug, Clone)]
pub struct ClaudeClient {
    api_endpoint: Option<String>,
}

impl ClaudeClient {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            api_endpoint: std::env::var("CLAUDE_API_ENDPOINT").ok(),
        })
    }

    pub fn configured(&self) -> bool {
        self.api_endpoint.is_some()
    }

    pub async fn analyze_system(
        &self,
        _metrics: &crate::analytics::ValueStreamAnalysis,
    ) -> Result<AIAnalysis> {
        bail!("Claude transport is configured but not implemented in swarmsh-v2")
    }

    pub async fn generate_optimization_plan(&self, _current_state: &str) -> Result<String> {
        bail!("Claude transport is configured but not implemented in swarmsh-v2")
    }
}

/// Ollama local LLM client.
///
/// Construction is side-effect free. Reachability is established only when a
/// method that actually calls Ollama is executed.
#[derive(Debug, Clone)]
pub struct OllamaClient {
    ollama: Ollama,
    default_model: String,
}

impl OllamaClient {
    pub async fn new() -> Result<Self> {
        let host = std::env::var("SWARMSH_OLLAMA_HOST")
            .unwrap_or_else(|_| "http://localhost:11434".to_string());
        let model =
            std::env::var("SWARMSH_OLLAMA_MODEL").unwrap_or_else(|_| "llama2:latest".to_string());
        Self::with_config(&host, &model).await
    }

    pub async fn with_config(host: &str, default_model: &str) -> Result<Self> {
        Ok(Self {
            ollama: Ollama::new(host.to_string(), 11434),
            default_model: default_model.to_string(),
        })
    }

    #[instrument(skip(self))]
    pub async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let models = self
            .ollama
            .list_local_models()
            .await
            .context("Failed to list Ollama models")?;

        Ok(models
            .into_iter()
            .map(|model| ModelInfo {
                name: model.name,
                modified_at: model.modified_at,
                size: model.size,
            })
            .collect())
    }

    #[instrument(skip(self))]
    pub async fn analyze_coordination(
        &self,
        pattern: &str,
        context: Option<&str>,
    ) -> Result<AIAnalysis> {
        let mut messages = vec![ChatMessage::new(
            MessageRole::System,
            "Analyze the supplied coordination context. Return bounded recommendations; do not claim execution or proof that was not observed."
                .to_string(),
        )];

        if let Some(context) = context {
            messages.push(ChatMessage::new(MessageRole::User, context.to_string()));
        }
        messages.push(ChatMessage::new(
            MessageRole::User,
            format!("Analyze coordination pattern: {pattern}"),
        ));

        let request = ChatMessageRequest::new(self.default_model.clone(), messages);
        let response = self
            .ollama
            .send_chat_messages(request)
            .await
            .context("Failed to get Ollama chat response")?;

        self.parse_analysis_response(&response.message.content)
    }

    #[instrument(skip(self, agent_context))]
    pub async fn make_agent_decision(
        &self,
        agent_context: &serde_json::Value,
        decision_type: &str,
    ) -> Result<AgentDecision> {
        let messages = vec![
            ChatMessage::new(
                MessageRole::System,
                "Return a JSON advisory decision with fields action, parameters, confidence, alternatives. Never invent execution authority."
                    .to_string(),
            ),
            ChatMessage::new(
                MessageRole::User,
                format!(
                    "Context: {}\nDecision type: {}",
                    serde_json::to_string(agent_context)?,
                    decision_type
                ),
            ),
        ];

        let request = ChatMessageRequest::new(self.default_model.clone(), messages);
        let response = self
            .ollama
            .send_chat_messages(request)
            .await
            .context("Failed to get Ollama decision")?;

        self.parse_decision_response(&response.message.content)
    }

    #[instrument(skip(self, patterns))]
    pub async fn analyze_pattern_similarity(
        &self,
        patterns: Vec<String>,
    ) -> Result<Vec<PatternSimilarity>> {
        let mut results = Vec::with_capacity(patterns.len());

        for pattern in patterns {
            let request = GenerateEmbeddingsRequest::new(
                self.default_model.clone(),
                ollama_rs::generation::embeddings::request::EmbeddingsInput::Single(
                    pattern.clone(),
                ),
            );

            let response: GenerateEmbeddingsResponse = self
                .ollama
                .generate_embeddings(request)
                .await
                .context("Failed to generate embeddings")?;

            if let Some(embedding) = response.embeddings.first() {
                results.push(PatternSimilarity {
                    pattern,
                    similarity_score: 0.0,
                    embeddings: embedding.clone(),
                });
            }
        }

        Ok(results)
    }

    pub fn calculate_similarity(&self, embedding1: &[f32], embedding2: &[f32]) -> f64 {
        if embedding1.len() != embedding2.len() {
            return 0.0;
        }

        let dot_product: f32 = embedding1
            .iter()
            .zip(embedding2.iter())
            .map(|(left, right)| left * right)
            .sum();
        let magnitude1 = embedding1
            .iter()
            .map(|value| value * value)
            .sum::<f32>()
            .sqrt();
        let magnitude2 = embedding2
            .iter()
            .map(|value| value * value)
            .sum::<f32>()
            .sqrt();

        if magnitude1 == 0.0 || magnitude2 == 0.0 {
            return 0.0;
        }

        f64::from(dot_product / (magnitude1 * magnitude2))
    }

    #[instrument(skip(self, metrics))]
    pub async fn stream_optimization_suggestions(
        &self,
        metrics: &serde_json::Value,
    ) -> Result<Pin<Box<dyn Stream<Item = String> + Send>>> {
        let prompt = format!(
            "Provide bounded optimization suggestions for these coordination metrics:\n{}",
            serde_json::to_string(metrics)?
        );
        let request = GenerationRequest::new(self.default_model.clone(), prompt);

        let response = self
            .ollama
            .generate(request)
            .await
            .context("Failed to generate optimization suggestion")?;
        Ok(Box::pin(tokio_stream::once(response.response)))
    }

    #[instrument(skip(self, current_script))]
    pub async fn generate_shell_optimization(
        &self,
        current_script: &str,
        requirements: &str,
    ) -> Result<String> {
        let prompt = format!(
            "Optimize this shell script under these requirements: {requirements}\n\n{current_script}"
        );
        let request = GenerationRequest::new(self.default_model.clone(), prompt);
        let response = self
            .ollama
            .generate(request)
            .await
            .context("Failed to generate shell optimization")?;
        Ok(response.response)
    }

    #[instrument(skip(self, health_data))]
    pub async fn analyze_bottlenecks(&self, health_data: &serde_json::Value) -> Result<AIAnalysis> {
        self.analyze_coordination(
            "health_bottleneck_analysis",
            Some(&serde_json::to_string(health_data)?),
        )
        .await
    }

    fn parse_analysis_response(&self, content: &str) -> Result<AIAnalysis> {
        if let Ok(analysis) = serde_json::from_str::<AIAnalysis>(content) {
            return Ok(analysis);
        }

        let recommendations = content
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                line.strip_prefix("- ")
                    .or_else(|| line.strip_prefix("* "))
                    .map(str::to_string)
            })
            .collect();

        Ok(AIAnalysis {
            recommendations,
            confidence: 0.0,
            optimization_opportunities: Vec::new(),
            reasoning: Some(content.to_string()),
        })
    }

    fn parse_decision_response(&self, content: &str) -> Result<AgentDecision> {
        serde_json::from_str::<AgentDecision>(content)
            .context("Ollama decision was not valid AgentDecision JSON")
    }
}

#[derive(Debug, Clone)]
pub struct AIIntegration {
    claude: Option<ClaudeClient>,
    ollama: Option<OllamaClient>,
}

impl AIIntegration {
    #[instrument(skip_all)]
    pub async fn new() -> Result<Self> {
        let claude = ClaudeClient::new()
            .await
            .ok()
            .filter(ClaudeClient::configured);
        let ollama = if env_flag("SWARMSH_ENABLE_OLLAMA") {
            Some(OllamaClient::new().await?)
        } else {
            None
        };

        Ok(Self { claude, ollama })
    }

    pub fn ollama_enabled(&self) -> bool {
        self.ollama.is_some()
    }

    pub fn claude_configured(&self) -> bool {
        self.claude.is_some()
    }

    #[instrument(skip(self))]
    pub async fn analyze(&self, context: &str) -> Result<AIAnalysis> {
        if let Some(ref ollama) = self.ollama {
            return ollama.analyze_coordination(context, None).await;
        }

        if self.claude.is_some() {
            bail!("Claude endpoint is configured but no Claude transport is implemented")
        }

        bail!("no AI provider is enabled")
    }

    #[instrument(skip(self, patterns))]
    pub async fn get_pattern_embeddings(
        &self,
        patterns: Vec<String>,
    ) -> Result<Vec<PatternSimilarity>> {
        match self.ollama {
            Some(ref ollama) => ollama.analyze_pattern_similarity(patterns).await,
            None => bail!("Ollama is not enabled"),
        }
    }

    #[instrument(skip(self, context))]
    pub async fn make_decision(
        &self,
        context: &serde_json::Value,
        decision_type: &str,
    ) -> Result<AgentDecision> {
        if let Some(ref ollama) = self.ollama {
            return ollama.make_agent_decision(context, decision_type).await;
        }

        Ok(AgentDecision {
            action: "deterministic_fallback".to_string(),
            parameters: serde_json::json!({"reason": "no_ai_provider_enabled"}),
            confidence: 0.0,
            alternatives: Vec::new(),
        })
    }

    pub async fn stream_optimizations(
        &self,
        metrics: &serde_json::Value,
    ) -> Result<Pin<Box<dyn Stream<Item = String> + Send>>> {
        if let Some(ref ollama) = self.ollama {
            return ollama.stream_optimization_suggestions(metrics).await;
        }

        Ok(Box::pin(tokio_stream::empty()))
    }

    #[instrument(skip(self, script))]
    pub async fn optimize_shell_script(&self, script: &str, requirements: &str) -> Result<String> {
        if let Some(ref ollama) = self.ollama {
            return ollama
                .generate_shell_optimization(script, requirements)
                .await;
        }

        debug!(
            requirements,
            "AI shell optimization skipped because no provider is enabled"
        );
        Ok(script.to_string())
    }

    #[instrument(skip(self, metadata, correlation_id))]
    pub async fn analyze_with_context(
        &self,
        prompt: &str,
        metadata: &std::collections::HashMap<String, String>,
        correlation_id: &crate::telemetry::CorrelationId,
    ) -> Result<AIAnalysis> {
        let context = serde_json::json!({
            "prompt": prompt,
            "metadata": metadata,
            "correlation_id": correlation_id.to_string(),
        });
        self.analyze(&serde_json::to_string(&context)?).await
    }
}

fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn creation_is_network_independent_by_default() {
        std::env::remove_var("SWARMSH_ENABLE_OLLAMA");
        let ai = AIIntegration::new().await.expect("AI integration");
        assert!(!ai.ollama_enabled());
    }

    #[tokio::test]
    async fn no_provider_returns_explicit_unsupported_analysis() {
        std::env::remove_var("SWARMSH_ENABLE_OLLAMA");
        std::env::remove_var("CLAUDE_API_ENDPOINT");
        let ai = AIIntegration::new().await.expect("AI integration");
        assert!(ai.analyze("test").await.is_err());
    }

    #[test]
    fn similarity_calculation_is_bounded() {
        let client = OllamaClient {
            ollama: Ollama::new("http://localhost:11434".to_string(), 11434),
            default_model: "test".to_string(),
        };

        let same = client.calculate_similarity(&[1.0, 0.0], &[1.0, 0.0]);
        let orthogonal = client.calculate_similarity(&[1.0, 0.0], &[0.0, 1.0]);
        assert!((same - 1.0).abs() < 0.001);
        assert!(orthogonal.abs() < 0.001);
    }
}
