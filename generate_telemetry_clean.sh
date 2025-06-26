#!/bin/bash
# SwarmSH v2 Telemetry Code Generator (Clean Version)
# Generates clean Rust telemetry code from SwarmSH semantic conventions

set -euo pipefail

echo "SwarmSH v2 Telemetry Code Generator (Clean)"
echo "Generating from semantic conventions..."

SEMCONV_DIR="semantic-conventions"
OUTPUT_DIR="src/generated"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Generate attributes.rs
echo "Generating attributes.rs..."
cat > "$OUTPUT_DIR/attributes.rs" << 'EOF'
// Generated from SwarmSH v2 semantic conventions
// 80/20 implementation - core attributes only

EOF

# Track processed modules to avoid duplicates (use simple approach for compatibility)
processed_modules=""

# Process key semantic convention files in a specific order
key_files=(
    "swarmsh-agent.yaml"
    "swarmsh-work.yaml" 
    "swarmsh-coordination.yaml"
    "swarmsh-health.yaml"
    "swarmsh-analytics.yaml"
    "swarmsh-infinite-loop.yaml"
    "swarmsh-worktree.yaml"
    "swarmsh-auto-8020.yaml"
)

for filename in "${key_files[@]}"; do
    yaml_file="$SEMCONV_DIR/$filename"
    
    if [[ -f "$yaml_file" ]]; then
        # Extract group ID from file
        group_id=$(grep -m1 "id:" "$yaml_file" | sed 's/.*id: //' | tr -d '"' | tr -d "'" || echo "unknown")
        
        if [[ "$group_id" == "unknown" ]]; then
            continue
        fi
        
        module_name=$(echo "$group_id" | tr '.' '_' | tr '[:upper:]' '[:lower:]')
        
        # Skip if module already processed
        if [[ "$processed_modules" == *"$module_name"* ]]; then
            echo "Skipping duplicate module: $module_name"
            continue
        fi
        
        processed_modules="$processed_modules $module_name"
        
        echo "Processing $filename -> $module_name"
        
        # Get brief description
        brief=$(grep -m1 "brief:" "$yaml_file" | sed 's/.*brief: //' | tr -d '"' | head -c 80 || echo "Generated constants")
        
        cat >> "$OUTPUT_DIR/attributes.rs" << EOF
// $brief
pub mod $module_name {
EOF
        
        # Extract unique attributes, filtering for valid identifiers
        grep -A 10000 "attributes:" "$yaml_file" | grep "id:" | sed 's/.*id: //' | sort -u | while read -r attr_id; do
            # Clean and validate identifier
            attr_id=$(echo "$attr_id" | tr -d '"' | tr -d "'" | sed 's/[[:space:]]*$//' | sed 's/^[[:space:]]*//')
            
            # Only process valid attribute identifiers (no numbers, examples, etc)
            if [[ -n "$attr_id" && "$attr_id" =~ ^[a-zA-Z][a-zA-Z0-9_.]*$ && ! "$attr_id" =~ ^[0-9] ]]; then
                attr_name=$(echo "$attr_id" | tr '.' '_' | tr '[:lower:]' '[:upper:]')
                echo "    pub const ${attr_name}: &str = \"${group_id}.${attr_id}\";"
            fi
        done >> "$OUTPUT_DIR/attributes.rs"
        
        echo "}" >> "$OUTPUT_DIR/attributes.rs"
        echo "" >> "$OUTPUT_DIR/attributes.rs"
    fi
done

# Generate span_builders.rs
echo "Generating span_builders.rs..."
cat > "$OUTPUT_DIR/span_builders.rs" << 'EOF'
// Generated span builders from SwarmSH v2 semantic conventions

use opentelemetry::global::BoxedSpan;
use opentelemetry::trace::Tracer;

// Core span builders expected by lib.rs
pub fn agent_lifecycle_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.agent.lifecycle")
}

pub fn work_coordination_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.work.coordination")
}

pub fn coordination_protocol_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.coordination.protocol")
}

// Generated span builders for all groups
EOF

# Add span builders for each processed group
for filename in "${key_files[@]}"; do
    yaml_file="$SEMCONV_DIR/$filename"
    
    if [[ -f "$yaml_file" ]]; then
        group_id=$(grep -m1 "id:" "$yaml_file" | sed 's/.*id: //' | tr -d '"' | tr -d "'" || echo "unknown")
        group_type=$(grep -m1 "type:" "$yaml_file" | sed 's/.*type: //' | tr -d '"' | tr -d "'" || echo "unknown")
        
        if [[ "$group_id" != "unknown" && "$group_type" == "span" ]]; then
            function_name="create_$(echo "$group_id" | tr '.' '_')_span"
            cat >> "$OUTPUT_DIR/span_builders.rs" << EOF
pub fn $function_name(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("$group_id")
}

EOF
        fi
    fi
done

# Generate metrics.rs
echo "Generating metrics.rs..."
cat > "$OUTPUT_DIR/metrics.rs" << 'EOF'
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
EOF

echo "✅ Generated clean telemetry files:"
echo "  - $OUTPUT_DIR/attributes.rs"
echo "  - $OUTPUT_DIR/span_builders.rs" 
echo "  - $OUTPUT_DIR/metrics.rs"

# Count generated attributes
attr_count=$(grep -c "pub const" "$OUTPUT_DIR/attributes.rs" || echo "0")
span_count=$(grep -c "pub fn.*span" "$OUTPUT_DIR/span_builders.rs" || echo "0")

echo "📊 Generation summary:"
echo "  - $attr_count attribute constants"
echo "  - $span_count span builder functions"
echo "  - 1 metrics registry"

echo "🎉 Clean telemetry code generation complete!"