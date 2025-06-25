# Tera Templates - Shell Export System

This directory contains Tera templates for SwarmSH v2's revolutionary Rust → Shell export capability.

## Template Architecture

### Core Templates
- `coordination_helper.sh.tera` - Main coordination logic template
- `agent_swarm_orchestrator.sh.tera` - Agent orchestration template
- `work_queue_manager.sh.tera` - Work distribution template
- `health_monitor.sh.tera` - Health monitoring template

### Template Features
- **Custom Filters**: shell_escape, bash_array, nanosecond_id
- **Inheritance**: Shared patterns across templates
- **Conditional Logic**: Dynamic shell generation
- **Type Safety**: Rust-integrated template rendering

## Custom Tera Filters

### shell_escape
```tera
{{ user_input | shell_escape }}
# Safely escapes shell special characters
```

### bash_array  
```tera
{{ rust_vec | bash_array }}
# Converts Rust Vec to bash array syntax
```

### nanosecond_id
```tera
{{ "" | nanosecond_id }}
# Generates nanosecond-precision unique ID
```

## Template Development Guidelines

### Critical Requirements
- **UNIX Compatibility**: Must work on all UNIX systems
- **No Dependencies**: Generated scripts require no external tools
- **Zero Conflicts**: Maintain mathematical guarantees in shell
- **Atomic Operations**: Use advisory locking for file operations

### Shell Patterns
```bash
# Nanosecond-precision ID generation
AGENT_ID="agent_$(date +%s%N)"

# Atomic file operations with advisory locking
{
    flock -x 200
    # Critical section - file operations here
} 200>/tmp/swarmsh_lock_${WORK_ID}

# Pull-based work claiming
claim_work() {
    local work_dir="$1"
    local agent_id="$2"
    
    for work_file in "$work_dir"/work_*.todo; do
        if flock -n -x "$work_file" true 2>/dev/null; then
            mv "$work_file" "${work_file%.todo}.claimed_${agent_id}"
            return 0
        fi
    done
    return 1
}
```

### Template Structure
```tera
{# coordination_helper.sh.tera #}
#!/bin/bash
# Generated from SwarmSH v2 Rust implementation
# Maintains zero-conflict coordination guarantees

{% include "shared/header.sh.tera" %}

{% for agent in agents %}
setup_agent_{{ agent.id | shell_escape }}() {
    local agent_config="{{ agent.config | shell_escape }}"
    local work_dir="{{ work_directory | shell_escape }}"
    
    # Nanosecond-precision registration
    AGENT_ID="{{ agent.id | nanosecond_id }}"
    
    {% if agent.coordination_enabled %}
    # Enable coordination protocols
    setup_coordination "$AGENT_ID" "$work_dir"
    {% endif %}
}
{% endfor %}
```

## Testing Templates

### Validation Commands
```bash
# Test template rendering
cargo run --bin shell_exporter -- --template coordination_helper.sh.tera --output test.sh

# Validate generated shell syntax
bash -n test.sh

# Test coordination behavior
./test.sh --validate-coordination

# Performance testing
time ./test.sh --benchmark-mode
```

### Integration Testing
- Templates must pass shellcheck validation
- Generated scripts tested with multiple agents
- Zero-conflict guarantees verified in shell
- Performance compared to Rust implementation

## Shell Export Requirements

### Functionality Preservation
- Complete agent coordination capability
- Work queue management with atomic claiming
- Health monitoring and bottleneck detection
- Real-time coordination protocols
- DLSS analytics and optimization

### Performance Targets
- <10% overhead vs Rust implementation
- Nanosecond precision maintained
- Zero coordination conflicts
- UNIX-native file operations only

## Template Context Data

### Available Variables
```rust
pub struct TemplateContext {
    pub agents: Vec<AgentConfig>,
    pub work_directory: PathBuf,
    pub coordination_config: CoordinationConfig,
    pub health_config: HealthConfig,
    pub analytics_config: AnalyticsConfig,
}
```

### Generated Script Features
- Agent registration with nanosecond IDs
- Atomic work claiming mechanisms
- Health monitoring loops
- Coordination protocol implementation
- OTEL telemetry collection (JSON output)

## Advanced Patterns

### Template Inheritance
```tera
{% extends "base_script.sh.tera" %}

{% block agent_setup %}
# Agent-specific setup logic
{% endblock %}

{% block coordination_loop %}
# Coordination protocol implementation
{% endblock %}
```

### Conditional Generation
```tera
{% if coordination_config.scrum_at_scale_enabled %}
# Scrum at Scale coordination patterns
{% elif coordination_config.roberts_rules_enabled %}  
# Roberts Rules governance patterns
{% endif %}
```

Remember: These templates generate complete distributed coordination systems as portable shell scripts!
