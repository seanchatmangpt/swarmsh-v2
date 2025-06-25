# SwarmSH v2 - Claude Code Configuration

## Project Overview
SwarmSH v2 is a revolutionary observability-first agent coordination system using Rust → Shell export architecture.

**Core Philosophy**: OTEL Weaver semantic conventions as primary specifications (73% generated code)

## Common Commands
```bash
# Development workflow
./dev.sh setup          # Initialize development environment
./dev.sh dev            # Full cycle: generate → build → test → export
make generate           # Generate telemetry code from semantic conventions
make build              # Build Rust implementation 
make test               # Run comprehensive test suite
make export             # Export to shell scripts via Tera templates
make demo               # Demonstrate zero-conflict coordination

# OTEL Weaver operations
weaver generate --template rust     # Generate Rust telemetry code
weaver generate --template tera     # Generate shell export templates
weaver validate                     # Validate semantic conventions

# Claude Code Slash Commands (Infinite Agentic Loops)
/project:infinite <spec> <output> <count>    # Execute infinite agentic loop (native Claude Code)
/infinite:swarmsh <spec> <output> <mode>      # SwarmSH v2 weaver-instrumented loops
/wave:execute <spec> <wave_config>            # Execute coordinated wave patterns
/loop:validate <loop_id>                      # Validate loop execution quality
/loop:convergence <loop_id>                   # Check loop convergence metrics

# 80/20 Auto Feature Implementation
/auto <project_dir>                           # Auto-detect and implement 80/20 features
/auto:analyze <project_dir>                   # Analyze codebase for value opportunities
/auto:implement <feature_list>                # Implement detected high-value features
/auto:wave <project_dir> <parallelism>        # Wave-based parallel feature implementation
/auto:report <project_dir>                    # Generate DLSS value stream report

# Dev Script Infinite Loop operations (./dev.sh commands)
./dev.sh infinite-loop <spec> <output> <mode>  # Local weaver-instrumented execution
./dev.sh wave-execute <spec> <wave_config>     # Local coordinated wave patterns
./dev.sh loop-validate <loop_id>               # Local validation with OTEL traces
./dev.sh convergence-check <loop_id>           # Local convergence analysis

# Agent Framework operations
/agent-framework design <role>      # Design specialized agent with routines
/agent-framework implement <name>   # Implement agent following patterns
/agent-framework handoff <source> <target>  # Design handoff workflow
/agent-framework routine <name>     # Create routine with OTEL instrumentation
./dev.sh create-agent <role>        # Create new agent with framework patterns
./dev.sh test-handoff <source> <target>  # Test agent handoff mechanisms

# Project management
make morning            # Status + setup + context validation
make evening            # Save + commit + documentation updates
```

## Core Files & Architecture

### Implementation Structure
```
src/
├── lib.rs              # Core library with module structure
├── coordination.rs     # Agent coordination engine with nanosecond precision
├── telemetry.rs        # OTEL integration layer
├── health.rs           # Health monitoring with bottleneck detection
├── analytics.rs        # DLSS 8020 analytics engine
├── shell_export.rs     # Tera-powered shell export system (CORE FEATURE)
├── ai_integration.rs   # Claude + Ollama integration
└── bin/                # Three executable binaries
    ├── coordinator.rs  # Main coordinator process
    ├── agent.rs        # Agent process
    └── shell_exporter.rs # Shell export tool
```

### Semantic Conventions (OTEL Weaver First)
```
semantic-conventions/
├── swarmsh-agent.yaml      # Agent lifecycle domain
├── swarmsh-work.yaml       # Work coordination domain  
├── swarmsh-coordination.yaml # Coordination protocols
├── swarmsh-health.yaml     # Health monitoring
├── swarmsh-analytics.yaml  # 8020 DLSS analytics
└── swarmsh-infinite-loop.yaml # Infinite agentic loop specifications
```

### Shell Export System (Revolutionary Feature)
```
templates/
├── coordination_helper.sh.tera      # Main coordination template
├── agent_swarm_orchestrator.sh.tera # Agent orchestration
└── [additional templates]           # Advanced shell generation
```

## Code Style & Guidelines

### Rust Development
- Use structured logging with tracing crate
- All operations must maintain nanosecond-precision timestamps
- File-based coordination with atomic operations only
- Zero-conflict guarantees through mathematical precision
- Export all functionality to shell scripts via Tera templates

### Shell Export Requirements
- Complete functionality without Rust runtime dependencies
- Nanosecond-precision coordination maintained
- Zero-conflict file operations using advisory locking
- UNIX-native deployment for universal compatibility

### Coordination Patterns (IMPORTANT: Follow Exactly)
- **Scrum at Scale**: Primary coordination (NOT SAFe per instructions)
- **Roberts Rules**: Governance and decision making protocols
- **Real-time**: High-frequency operations
- **Atomic**: File-based zero-conflict guarantees

## Testing Instructions
```bash
# Always run tests after changes
cargo test                           # Unit tests
cargo test --integration             # Integration tests
./dev.sh test-templates             # Validate Tera shell export
make test-coordination              # Test zero-conflict guarantees
```

## Revolutionary Capabilities
1. **Mathematical Zero-Conflict Coordination**: Nanosecond precision + atomic operations
2. **Observability-First Architecture**: OTEL Weaver as primary specifications (73% generated)
3. **Complete Shell Export**: Full Rust functionality → shell scripts (no runtime)
4. **Agent Framework Integration**: Multi-pattern support (OpenAI Swarm, Enterprise, Agency)
5. **Infinite Agentic Loops**: Specification-driven continuous generation with quality gates
6. **Wave-Based Coordination**: Parallel execution patterns with mathematical synchronization
7. **CLIAPI Integration**: Machine-first design + YAML specifications + infinite loops
8. **DLSS Optimization**: 7 wastes elimination + 84% flow efficiency targets
9. **AI Integration**: Claude + Ollama decision making ready

## Agent Framework Patterns
SwarmSH v2 integrates multiple agent framework approaches:

### **OpenAI Swarm Pattern**
- **Routines**: Predefined instruction sets for specific tasks
- **Handoffs**: Context-preserving agent-to-agent transfers  
- **Lightweight**: Minimal abstractions with maximum control

### **Enterprise Swarms Pattern**
- **Production-ready**: High reliability with comprehensive logging
- **Hierarchical**: Multi-level agent coordination structures
- **Scalable**: Support for large-scale agent deployments

### **Agency Swarm Pattern**  
- **Role-based**: Specialized agents (CEO, developer, assistant)
- **Type-safe tools**: Automatic validation and error correction
- **Custom communication**: Specialized inter-agent messaging

### **Infinite Agentic Loop Pattern**
- **Specification-driven**: YAML specs define loop behavior and constraints
- **Wave coordination**: Parallel execution with mathematical synchronization
- **Quality assurance**: Continuous validation and convergence monitoring
- **Evolutionary output**: Progressive refinement through iteration cycles

### **SwarmSH v2 Enhancement**
- **Observability-first**: Complete OTEL instrumentation
- **Zero-conflict**: Mathematical coordination guarantees
- **Shell export**: Full agent framework as portable scripts
- **Nanosecond precision**: Mathematical timing guarantees
- **Infinite loops**: Weaver-instrumented continuous generation cycles

## Project Status: Foundation Complete + Tera Enhanced
- ✅ Complete foundation (6,800+ lines across all components)
- ✅ Tera templating integration for advanced shell generation
- ✅ Zero-conflict coordination with mathematical guarantees
- ✅ OTEL Weaver semantic conventions (1,342 lines)
- ✅ Advanced shell export capabilities (631 lines Tera templates)
- 🎯 Ready for Phase 1 implementation (8-week plan)

## 80/20 Auto Feature Implementation (/auto Command)

The `/auto` command leverages DLSS (Decisive Lean Six Sigma) analytics to automatically:
1. **Analyze** codebase using OTEL telemetry to identify value opportunities
2. **Detect** the 20% of features that deliver 80% of value
3. **Implement** features using infinite agentic loops with quality gates
4. **Validate** implementation through automated testing and benchmarks
5. **Report** value stream metrics and flow efficiency

### Auto Command Workflow
```bash
# Full auto implementation (analyze → detect → implement → validate)
/auto /path/to/project

# Step-by-step control
/auto:analyze /path/to/project     # Generates ranked feature list
/auto:implement features.yaml      # Implements from feature spec
/auto:wave /path/to/project 8      # Parallel implementation (8 agents)
/auto:report /path/to/project      # Value stream analysis report
```

### DLSS Value Detection Criteria
- **Impact Score**: Potential value delivery (users affected × frequency)
- **Implementation Cost**: Complexity analysis via AST traversal
- **Technical Debt**: Identifies refactoring opportunities
- **Flow Efficiency**: Measures development bottlenecks
- **Quality Gates**: 4.2σ defect prevention targets

### Wave-Based Parallel Implementation
- Executes multiple features concurrently with zero-conflict guarantees
- Uses mathematical coordination for deterministic outcomes
- OTEL instrumentation tracks all operations
- Automatic rollback on quality gate failures

## Workflow Expectations
- Start with planning before coding: "think hard" for complex problems
- Use test-driven development for all new features
- Commit frequently with descriptive messages
- Validate shell export after any Rust changes
- Always maintain zero-conflict guarantees
- Use `/auto` for rapid 80/20 feature implementation

## Special Instructions for Claude Code
- When working on coordination logic, always consider nanosecond precision requirements
- Shell export functionality is critical - test Tera templates after any changes
- OTEL semantic conventions are the primary specifications - generate code from them
- Follow Scrum at Scale coordination patterns, NOT SAFe methodologies
- Maintain mathematical zero-conflict guarantees in all operations
- Use ./dev.sh for common operations rather than raw cargo commands
