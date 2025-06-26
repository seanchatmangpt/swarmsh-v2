// Generated metrics from SwarmSH v2 semantic conventions

use std::collections::HashMap;

/// SwarmSH v2 metrics registry
#[derive(Debug, Clone)]
pub struct SwarmMetrics {
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
}

impl SwarmMetrics {
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
            gauges: HashMap::new(),
        }
    }

    pub fn increment_counter(&mut self, name: &str) {
        *self.counters.entry(name.to_string()).or_insert(0) += 1;
    }

    pub fn set_gauge(&mut self, name: &str, value: f64) {
        self.gauges.insert(name.to_string(), value);
    }

    pub fn get_counter(&self, name: &str) -> u64 {
        self.counters.get(name).copied().unwrap_or(0)
    }

    pub fn get_gauge(&self, name: &str) -> f64 {
        self.gauges.get(name).copied().unwrap_or(0.0)
    }
}

impl Default for SwarmMetrics {
    fn default() -> Self {
        Self::new()
    }
}
