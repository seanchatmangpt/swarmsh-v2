# Semantic Conventions - OTEL Weaver Domain Specifications

This directory contains OpenTelemetry semantic conventions that serve as PRIMARY specifications for SwarmSH v2.

## Philosophy: Observability-First Architecture
Semantic conventions are written FIRST, then code is generated from them. This ensures:
- Zero telemetry drift (specifications and code always match)
- 73% of telemetry code auto-generated
- Consistent observability across all components

## Domain Files

### `swarmsh-agent.yaml` - Agent Lifecycle Domain
- Agent registration and lifecycle management
- Nanosecond-precision agent IDs
- Agent state transitions and coordination
- Health monitoring attributes

### `swarmsh-work.yaml` - Work Coordination Domain  
- Work item specifications and claiming
- Pull-based work distribution patterns
- Atomic work state transitions
- Work completion tracking

### `swarmsh-coordination.yaml` - Coordination Protocols
- Multi-agent coordination patterns
- Conflict resolution mechanisms
- Real-time coordination attributes
- Scrum at Scale protocol specifications

### `swarmsh-health.yaml` - Health Monitoring
- System health metrics and spans
- Bottleneck detection attributes
- Performance monitoring specifications
- DLSS optimization metrics

### `swarmsh-analytics.yaml` - 8020 DLSS Analytics
- Value stream mapping attributes
- Waste elimination metrics
- Quality control spans (4.2σ targets)
- Flow efficiency measurements

## Working with Semantic Conventions

### Adding New Attributes
1. Follow OpenTelemetry naming conventions
2. Use snake_case for attribute names
3. Include clear descriptions and examples
4. Specify data types and constraints

### Generating Code
```bash
# Validate conventions first
weaver validate

# Generate Rust telemetry code
weaver generate --template rust

# Generate Tera shell export templates
weaver generate --template tera
```

## Important Guidelines
- Semantic conventions are the PRIMARY specifications
- All telemetry must map to these conventions
- Changes here trigger code regeneration
- Shell export must maintain all observability
- Zero-conflict coordination patterns required

## Weaver Configuration
See `../weaver.yaml` for generation configuration and template settings.
