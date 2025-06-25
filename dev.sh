#!/bin/bash
# SwarmSH v2 Development Helper Script
# Provides common development workflows and CDCS integration

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR"

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_header() {
    echo -e "${PURPLE}${1}${NC}"
    echo -e "${PURPLE}$(printf '=%.0s' $(seq 1 ${#1}))${NC}"
}

# Check dependencies
check_dependencies() {
    log_header "Checking Dependencies"
    
    # Check Rust
    if command -v rustc &> /dev/null; then
        log_success "Rust found: $(rustc --version)"
    else
        log_error "Rust not found. Please install: https://rustup.rs/"
        exit 1
    fi
    
    # Check OTEL Weaver
    if command -v otel-weaver &> /dev/null; then
        log_success "OTEL Weaver found: $(otel-weaver --version)"
    else
        log_warning "OTEL Weaver not found. Installing..."
        cargo install otel-weaver --features=cli
        log_success "OTEL Weaver installed"
    fi
    
    # Check make
    if command -v make &> /dev/null; then
        log_success "Make found: $(make --version | head -n1)"
    else
        log_error "Make not found. Please install build tools."
        exit 1
    fi
}

# Development environment setup
setup_dev_env() {
    log_header "Setting Up Development Environment"
    
    cd "$PROJECT_ROOT"
    
    # Create directories
    mkdir -p shell-export logs generated examples docs
    
    # Run make setup
    log_info "Running make setup..."
    make setup
    
    log_success "Development environment ready!"
}

# Quick development workflow
dev_workflow() {
    log_header "Quick Development Workflow"
    
    cd "$PROJECT_ROOT"
    
    # Generate code
    log_info "Generating telemetry code from semantic conventions..."
    make generate
    
    # Build
    log_info "Building Rust implementation..."
    make build
    
    # Test
    log_info "Running tests..."
    make test
    
    # Export to shell
    log_info "Exporting to shell scripts..."
    make export
    
    log_success "Development workflow completed!"
}

# CDCS integration
cdcs_integration() {
    log_header "CDCS Compound Intelligence Integration"
    
    cd "$PROJECT_ROOT"
    
    # Activate compound intelligence
    log_info "Activating compound intelligence workflows..."
    make compound &
    COMPOUND_PID=$!
    
    # Wait for startup
    sleep 5
    
    # Deploy infinite loops
    log_info "Deploying infinite agentic loops..."
    make infinite &
    INFINITE_PID=$!
    
    # Scale system
    log_info "Scaling to maximum compound intelligence..."
    make scale
    
    log_success "CDCS integration active!"
    log_info "Process IDs: Compound=$COMPOUND_PID, Infinite=$INFINITE_PID"
    log_warning "Use 'kill $COMPOUND_PID $INFINITE_PID' to stop"
}

# Demo mode
demo_mode() {
    log_header "SwarmSH v2 Demonstration Mode"
    
    cd "$PROJECT_ROOT"
    
    # Build if needed
    if [[ ! -f "target/release/swarmsh-coordinator" ]]; then
        log_info "Building system for demo..."
        make build
    fi
    
    # Export to shell
    log_info "Preparing shell export..."
    make export
    
    # Start coordinator
    log_info "Starting SwarmSH v2 coordinator..."
    timeout 30s make start &
    COORDINATOR_PID=$!
    
    sleep 3
    
    # Start agents
    log_info "Deploying demonstration agents..."
    timeout 20s make agent &
    sleep 2
    timeout 20s make agent &
    sleep 2
    timeout 20s make agent &
    
    sleep 5
    
    # Health check
    log_info "Checking system health..."
    make health || true
    
    # Analytics
    log_info "Running 8020 analysis..."
    make analyze || true
    
    # Shell demo
    log_info "Demonstrating shell export functionality..."
    cd shell-export
    timeout 10s ./coordination_helper.sh --help || true
    cd ..
    
    # Cleanup
    log_info "Cleaning up demo processes..."
    kill $COORDINATOR_PID 2>/dev/null || true
    make stop 2>/dev/null || true
    
    log_success "Demonstration completed!"
}

# Continuous development mode
continuous_dev() {
    log_header "Continuous Development Mode"
    
    cd "$PROJECT_ROOT"
    
    log_info "Starting continuous development with auto-reload..."
    log_info "Press Ctrl+C to stop"
    
    # Start file watcher
    if command -v cargo-watch &> /dev/null; then
        cargo watch -x "build" -x "test" -s "make export"
    else
        log_warning "cargo-watch not found. Installing..."
        cargo install cargo-watch
        cargo watch -x "build" -x "test" -s "make export"
    fi
}

# Performance testing
performance_test() {
    log_header "Performance Testing"
    
    cd "$PROJECT_ROOT"
    
    # Build release
    log_info "Building optimized release..."
    make build
    
    # Run benchmarks
    log_info "Running performance benchmarks..."
    cargo bench || log_warning "Benchmarks not available yet"
    
    # Test nanosecond precision
    log_info "Testing nanosecond precision ID generation..."
    for i in {1..100}; do
        ./target/release/swarmsh-agent generate-id
    done | sort | uniq -d | wc -l | {
        read duplicates
        if [[ $duplicates -eq 0 ]]; then
            log_success "Zero duplicates in 100 nanosecond-precision IDs"
        else
            log_error "Found $duplicates duplicate IDs!"
        fi
    }
    
    # Test coordination performance
    log_info "Testing coordination performance..."
    timeout 30s ./target/release/swarmsh-coordinator start &
    COORDINATOR_PID=$!
    
    sleep 5
    
    # Measure agent registration time
    start_time=$(date +%s%N)
    ./target/release/swarmsh-agent join --role worker &
    AGENT_PID=$!
    sleep 2
    end_time=$(date +%s%N)
    
    duration_ms=$(( (end_time - start_time) / 1000000 ))
    log_info "Agent registration took ${duration_ms}ms"
    
    if [[ $duration_ms -lt 100 ]]; then
        log_success "Registration performance target met (<100ms)"
    else
        log_warning "Registration took ${duration_ms}ms (target: <100ms)"
    fi
    
    # Cleanup
    kill $COORDINATOR_PID $AGENT_PID 2>/dev/null || true
    make stop 2>/dev/null || true
}

# Infinite agentic loop execution
infinite_loop() {
    local spec_file="$1"
    local output_dir="$2"
    local mode="${3:-single}"
    
    log_header "Infinite Agentic Loop Execution"
    
    cd "$PROJECT_ROOT"
    
    # Validate inputs
    if [[ ! -f "$spec_file" ]]; then
        log_error "Specification file not found: $spec_file"
        exit 1
    fi
    
    # Create output directory
    mkdir -p "$output_dir"
    
    # Generate loop configuration with OTEL instrumentation
    local loop_id="loop_$(date +%s%N)"
    local loop_config="$output_dir/loop_config.yaml"
    
    cat > "$loop_config" << EOF
loop_id: "$loop_id"
specification_path: "$spec_file"
output_directory: "$output_dir"
mode: "$mode"
quality_threshold: 0.8
max_iterations: 50
parallel_agents: 4
telemetry:
  otel_endpoint: "http://localhost:4317"
  service_name: "swarmsh-infinite-loop"
  attributes:
    swarmsh.infinite_loop.loop_id: "$loop_id"
    swarmsh.infinite_loop.mode: "$mode"
EOF
    
    log_info "Generated loop configuration: $loop_config"
    
    # Execute with OTEL instrumentation
    log_info "Starting infinite agentic loop with ID: $loop_id"
    
    # Use weaver to generate telemetry
    otel-weaver generate --template rust --output src/generated/ semantic-conventions/swarmsh-infinite-loop.yaml
    
    # Build with loop support
    make build
    
    # Execute loop
    case "$mode" in
        single)
            log_info "Executing single iteration mode"
            ./target/release/swarmsh-coordinator infinite-loop --config "$loop_config" --iterations 1
            ;;
        batch)
            log_info "Executing batch mode (5 iterations)"
            ./target/release/swarmsh-coordinator infinite-loop --config "$loop_config" --iterations 5
            ;;
        infinite)
            log_info "Executing infinite mode (continuous)"
            ./target/release/swarmsh-coordinator infinite-loop --config "$loop_config" --continuous
            ;;
        wave)
            log_info "Executing wave mode"
            wave_execute "$spec_file" "$loop_config"
            ;;
        *)
            log_error "Unknown mode: $mode"
            exit 1
            ;;
    esac
    
    log_success "Infinite agentic loop completed: $loop_id"
}

# Wave execution with parallel coordination
wave_execute() {
    local spec_file="$1"
    local wave_config="$2"
    
    log_header "Wave-Based Execution"
    
    # Generate wave pattern
    local wave_id="wave_$(date +%s%N)"
    local wave_size=5
    local parallel_factor=3
    
    log_info "Executing wave $wave_id (size: $wave_size, parallel: $parallel_factor)"
    
    # Execute parallel waves
    for ((i=1; i<=wave_size; i++)); do
        log_info "Starting wave iteration $i/$wave_size"
        
        # Launch parallel agents
        for ((j=1; j<=parallel_factor; j++)); do
            ./target/release/swarmsh-agent wave-execute \
                --wave-id "$wave_id" \
                --iteration "$i" \
                --agent-id "$j" \
                --config "$wave_config" &
        done
        
        # Wait for wave completion
        wait
        log_success "Wave iteration $i completed"
    done
    
    log_success "Wave execution completed: $wave_id"
}

# Loop validation and quality checking
loop_validate() {
    local loop_id="$1"
    
    log_header "Loop Validation"
    
    cd "$PROJECT_ROOT"
    
    log_info "Validating loop execution: $loop_id"
    
    # Check telemetry data
    ./target/release/swarmsh-coordinator validate-loop --loop-id "$loop_id"
    
    # Quality metrics
    ./target/release/swarmsh-coordinator quality-metrics --loop-id "$loop_id"
    
    log_success "Loop validation completed: $loop_id"
}

# Convergence checking
convergence_check() {
    local loop_id="$1"
    
    log_header "Convergence Analysis"
    
    cd "$PROJECT_ROOT"
    
    log_info "Checking convergence for loop: $loop_id"
    
    # Analyze convergence metrics
    ./target/release/swarmsh-coordinator convergence-check --loop-id "$loop_id"
    
    log_success "Convergence analysis completed: $loop_id"
}

# 80/20 Auto Feature Detection and Implementation
auto_features() {
    local project_dir="${1:-.}"
    
    log_header "80/20 Auto Feature Implementation"
    
    cd "$PROJECT_ROOT"
    
    # Validate project directory
    if [[ ! -d "$project_dir" ]]; then
        log_error "Project directory not found: $project_dir"
        exit 1
    fi
    
    local analysis_id="auto_$(date +%s%N)"
    local features_dir="$project_dir/.swarmsh/features"
    mkdir -p "$features_dir"
    
    log_info "Starting 80/20 analysis with ID: $analysis_id"
    
    # Step 1: Analyze codebase
    log_info "Analyzing codebase for value opportunities..."
    auto_analyze "$project_dir" > "$features_dir/analysis_$analysis_id.yaml"
    
    # Step 2: Select 80/20 features
    log_info "Selecting top 20% features by value ratio..."
    local feature_count=$(grep -c "^- feature_id:" "$features_dir/analysis_$analysis_id.yaml" || echo 0)
    local selected_count=$((feature_count * 20 / 100))
    [[ $selected_count -lt 1 ]] && selected_count=1
    
    log_info "Selected $selected_count features from $feature_count detected"
    
    # Step 3: Generate implementation spec
    cat > "$features_dir/implementation_$analysis_id.yaml" << EOF
analysis_id: "$analysis_id"
project_path: "$project_dir"
features_detected: $feature_count
features_selected: $selected_count
value_score_threshold: 0.8
telemetry:
  service_name: "swarmsh-auto-8020"
  attributes:
    swarmsh.auto.analysis_id: "$analysis_id"
    swarmsh.auto.project_path: "$project_dir"
implementation:
  mode: "wave"
  parallelism: 4
  quality_gates:
    test_coverage: 0.8
    performance_regression: 0.05
    defect_density: 0.5
EOF
    
    # Step 4: Implement features
    log_info "Implementing selected features..."
    auto_implement "$features_dir/implementation_$analysis_id.yaml"
    
    # Step 5: Generate report
    log_info "Generating DLSS value stream report..."
    auto_report "$project_dir" > "$features_dir/report_$analysis_id.md"
    
    log_success "80/20 auto implementation completed: $analysis_id"
    log_info "Report saved to: $features_dir/report_$analysis_id.md"
}

# Auto analyze codebase for 80/20 opportunities
auto_analyze() {
    local project_dir="$1"
    
    log_header "Auto Feature Analysis"
    
    cd "$PROJECT_ROOT"
    
    # Generate telemetry
    weaver registry generate --registry semantic-conventions/ --template rust --output src/generated/
    
    # Build analyzer
    make build
    
    # Run DLSS analysis
    ./target/release/swarmsh-coordinator auto-analyze \
        --project "$project_dir" \
        --output-format yaml \
        --include-metrics \
        --dlss-analysis \
        --value-stream-map
}

# Auto implement detected features
auto_implement() {
    local feature_spec="$1"
    
    log_header "Auto Feature Implementation"
    
    cd "$PROJECT_ROOT"
    
    # Validate feature spec
    if [[ ! -f "$feature_spec" ]]; then
        log_error "Feature specification not found: $feature_spec"
        exit 1
    fi
    
    # Extract configuration
    local parallelism=$(grep "parallelism:" "$feature_spec" | awk '{print $2}')
    local mode=$(grep "mode:" "$feature_spec" | awk '{print $2}' | tr -d '"')
    
    log_info "Implementation mode: $mode, parallelism: $parallelism"
    
    # Execute implementation
    if [[ "$mode" == "wave" ]]; then
        # Wave-based parallel implementation
        ./target/release/swarmsh-coordinator auto-implement \
            --spec "$feature_spec" \
            --mode wave \
            --parallelism "$parallelism" \
            --with-telemetry \
            --quality-gates
    else
        # Sequential implementation
        ./target/release/swarmsh-coordinator auto-implement \
            --spec "$feature_spec" \
            --mode sequential \
            --with-telemetry \
            --quality-gates
    fi
}

# Wave-based parallel auto implementation
auto_wave() {
    local project_dir="$1"
    local parallelism="${2:-8}"
    
    log_header "Wave-Based Auto Implementation"
    
    cd "$PROJECT_ROOT"
    
    local wave_id="wave_auto_$(date +%s%N)"
    
    log_info "Starting wave $wave_id with parallelism: $parallelism"
    
    # Create wave configuration
    local wave_config="/tmp/wave_config_$wave_id.yaml"
    cat > "$wave_config" << EOF
wave_id: "$wave_id"
project_path: "$project_dir"
parallelism_factor: $parallelism
coordination_method: "nanosecond_ordering"
quality_gates:
  wave_completion_threshold: 0.9
  rollback_on_failure: true
telemetry:
  service_name: "swarmsh-auto-wave"
  attributes:
    swarmsh.auto.wave.wave_id: "$wave_id"
    swarmsh.auto.wave.parallelism_factor: $parallelism
EOF
    
    # Analyze features
    local features_dir="$project_dir/.swarmsh/features"
    mkdir -p "$features_dir"
    auto_analyze "$project_dir" > "$features_dir/wave_analysis_$wave_id.yaml"
    
    # Execute wave implementation
    log_info "Executing parallel feature implementation..."
    
    # Launch parallel agents
    for ((i=1; i<=parallelism; i++)); do
        ./target/release/swarmsh-agent auto-wave \
            --wave-id "$wave_id" \
            --agent-id "$i" \
            --config "$wave_config" \
            --features "$features_dir/wave_analysis_$wave_id.yaml" &
    done
    
    # Wait for wave completion
    wait
    
    # Validate wave results
    ./target/release/swarmsh-coordinator wave-validate \
        --wave-id "$wave_id" \
        --quality-gates
    
    log_success "Wave implementation completed: $wave_id"
}

# Generate DLSS value stream report
auto_report() {
    local project_dir="$1"
    
    log_header "DLSS Value Stream Report"
    
    cd "$PROJECT_ROOT"
    
    # Generate comprehensive report
    ./target/release/swarmsh-coordinator auto-report \
        --project "$project_dir" \
        --format markdown \
        --include-telemetry \
        --value-stream-analysis \
        --waste-identification \
        --flow-efficiency \
        --bottleneck-analysis \
        --sigma-levels
}

# Quality checks
quality_check() {
    log_header "Quality Assurance Checks"
    
    cd "$PROJECT_ROOT"
    
    # Rust formatting
    log_info "Checking Rust code formatting..."
    if cargo fmt -- --check; then
        log_success "Code formatting is correct"
    else
        log_warning "Code formatting issues found. Run 'cargo fmt' to fix."
    fi
    
    # Clippy lints
    log_info "Running Clippy lints..."
    if cargo clippy -- -D warnings; then
        log_success "No Clippy warnings found"
    else
        log_warning "Clippy warnings found. Please address them."
    fi
    
    # Semantic convention validation
    log_info "Validating semantic conventions..."
    if otel-weaver validate semantic-conventions/; then
        log_success "Semantic conventions are valid"
    else
        log_error "Semantic convention validation failed"
    fi
    
    # Shell script validation
    log_info "Validating shell export..."
    make export
    
    if bash -n shell-export/*.sh; then
        log_success "Shell scripts are syntactically correct"
    else
        log_error "Shell script syntax errors found"
    fi
    
    # Documentation check
    log_info "Checking documentation..."
    if [[ -f "README.md" && -f "Cargo.toml" ]]; then
        log_success "Core documentation files present"
    else
        log_warning "Missing documentation files"
    fi
}

# OTEL Weaver Template Functions
weaver_generate() {
    log_header "OTEL Weaver Code Generation"
    cd "$PROJECT_ROOT"
    
    # Check if weaver is installed
    if ! command -v weaver &> /dev/null; then
        log_error "OTEL Weaver not found. Please install it first."
        log_info "Visit: https://github.com/open-telemetry/weaver"
        exit 1
    fi
    
    log_info "Generating Rust code from semantic conventions..."
    if weaver generate --template rust; then
        log_success "Rust code generation completed!"
    else
        log_error "Rust code generation failed"
        exit 1
    fi
    
    log_info "Generating shell export templates..."
    if weaver generate --template tera; then
        log_success "Shell export template generation completed!"
    else
        log_error "Shell export template generation failed"
        exit 1
    fi
}

weaver_validate() {
    log_header "OTEL Weaver Semantic Convention Validation"
    cd "$PROJECT_ROOT"
    
    # Check if weaver is installed
    if ! command -v weaver &> /dev/null; then
        log_error "OTEL Weaver not found. Please install it first."
        log_info "Visit: https://github.com/open-telemetry/weaver"
        exit 1
    fi
    
    log_info "Validating semantic conventions..."
    if weaver validate; then
        log_success "All semantic conventions are valid!"
    else
        log_error "Semantic convention validation failed"
        exit 1
    fi
}

weaver_forge_template() {
    local template_name="${1:-}"
    
    log_header "OTEL Weaver Forge Template Generation"
    cd "$PROJECT_ROOT"
    
    if [[ -z "$template_name" ]]; then
        log_error "Usage: ./dev.sh weaver-forge <template_name>"
        log_info "Example: ./dev.sh weaver-forge attributes"
        exit 1
    fi
    
    log_info "Running WeaverForge for template: $template_name"
    
    # Create a simple Rust binary to run WeaverForge
    cat > "$PROJECT_ROOT/src/bin/weaver_forge_runner.rs" << 'EOF'
use swarmsh_v2::weaver_forge::WeaverForge;
use anyhow::Result;
use tracing_subscriber;

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Create WeaverForge instance
    let mut forge = WeaverForge::new("weaver.yaml")?;
    
    // Generate code from templates
    forge.generate()?;
    
    println!("✅ WeaverForge template generation completed!");
    Ok(())
}
EOF
    
    # Build and run the WeaverForge runner
    log_info "Building WeaverForge runner..."
    if cargo build --bin weaver_forge_runner; then
        log_info "Running WeaverForge..."
        if cargo run --bin weaver_forge_runner; then
            log_success "WeaverForge template generation completed!"
            # Clean up temporary runner
            rm -f "$PROJECT_ROOT/src/bin/weaver_forge_runner.rs"
        else
            log_error "WeaverForge generation failed"
            rm -f "$PROJECT_ROOT/src/bin/weaver_forge_runner.rs"
            exit 1
        fi
    else
        log_error "Failed to build WeaverForge runner"
        rm -f "$PROJECT_ROOT/src/bin/weaver_forge_runner.rs"
        exit 1
    fi
}

# Main menu
show_menu() {
    echo -e "${CYAN}SwarmSH v2 Development Helper${NC}"
    echo -e "${CYAN}=============================${NC}"
    echo ""
    echo "Available commands:"
    echo "  deps        - Check and install dependencies"
    echo "  setup       - Set up development environment"
    echo "  dev         - Quick development workflow (generate, build, test, export)"
    echo "  watch       - Continuous development with auto-reload"
    echo "  cdcs        - Activate CDCS compound intelligence integration"
    echo "  demo        - Run comprehensive demonstration"
    echo "  perf        - Run performance tests"
    echo "  quality     - Run quality assurance checks"
    echo "  clean       - Clean build artifacts"
    echo "  help        - Show this help menu"
    echo ""
    echo "Infinite Agentic Loop commands:"
    echo "  infinite-loop <spec> <output> <mode>  - Execute infinite agentic loop"
    echo "  wave-execute <spec> <config>          - Execute wave-based coordination"
    echo "  loop-validate <loop_id>               - Validate loop execution quality"
    echo "  convergence-check <loop_id>           - Check loop convergence metrics"
    echo ""
    echo "80/20 Auto Feature commands:"
    echo "  auto <project_dir>                    - Auto-detect and implement 80/20 features"
    echo "  auto-analyze <project_dir>            - Analyze codebase for value opportunities"
    echo "  auto-implement <feature_list>         - Implement detected high-value features"
    echo "  auto-wave <project_dir> <parallel>    - Wave-based parallel implementation"
    echo "  auto-report <project_dir>             - Generate DLSS value stream report"
    echo ""
    echo "OTEL Weaver Template commands:"
    echo "  weaver-generate                       - Generate code from semantic conventions"
    echo "  weaver-validate                       - Validate semantic conventions"
    echo "  weaver-forge <template>               - Generate custom templates with MiniJinja"
    echo ""
    echo "Examples:"
    echo "  ./dev.sh setup                           # Initial setup"
    echo "  ./dev.sh dev                             # Standard development workflow"
    echo "  ./dev.sh auto .                          # Auto-implement 80/20 features"
    echo "  ./dev.sh auto-wave . 8                   # Parallel implementation (8 agents)"
    echo "  ./dev.sh infinite-loop specs/ui.md out infinite # Continuous loop"
}

# Main script logic
main() {
    case "${1:-help}" in
        deps)
            check_dependencies
            ;;
        setup)
            check_dependencies
            setup_dev_env
            ;;
        dev)
            check_dependencies
            dev_workflow
            ;;
        watch)
            check_dependencies
            continuous_dev
            ;;
        cdcs)
            check_dependencies
            cdcs_integration
            ;;
        demo)
            check_dependencies
            demo_mode
            ;;
        perf)
            check_dependencies
            performance_test
            ;;
        quality)
            check_dependencies
            quality_check
            ;;
        infinite-loop)
            check_dependencies
            if [[ $# -lt 4 ]]; then
                log_error "Usage: ./dev.sh infinite-loop <spec_file> <output_dir> <mode>"
                exit 1
            fi
            infinite_loop "$2" "$3" "$4"
            ;;
        wave-execute)
            check_dependencies
            if [[ $# -lt 3 ]]; then
                log_error "Usage: ./dev.sh wave-execute <spec_file> <config_file>"
                exit 1
            fi
            wave_execute "$2" "$3"
            ;;
        loop-validate)
            check_dependencies
            if [[ $# -lt 2 ]]; then
                log_error "Usage: ./dev.sh loop-validate <loop_id>"
                exit 1
            fi
            loop_validate "$2"
            ;;
        convergence-check)
            check_dependencies
            if [[ $# -lt 2 ]]; then
                log_error "Usage: ./dev.sh convergence-check <loop_id>"
                exit 1
            fi
            convergence_check "$2"
            ;;
        auto)
            check_dependencies
            if [[ $# -lt 2 ]]; then
                log_error "Usage: ./dev.sh auto <project_dir>"
                exit 1
            fi
            auto_features "$2"
            ;;
        auto-analyze)
            check_dependencies
            if [[ $# -lt 2 ]]; then
                log_error "Usage: ./dev.sh auto-analyze <project_dir>"
                exit 1
            fi
            auto_analyze "$2"
            ;;
        auto-implement)
            check_dependencies
            if [[ $# -lt 2 ]]; then
                log_error "Usage: ./dev.sh auto-implement <feature_list>"
                exit 1
            fi
            auto_implement "$2"
            ;;
        auto-wave)
            check_dependencies
            if [[ $# -lt 2 ]]; then
                log_error "Usage: ./dev.sh auto-wave <project_dir> [parallelism]"
                exit 1
            fi
            auto_wave "$2" "${3:-8}"
            ;;
        auto-report)
            check_dependencies
            if [[ $# -lt 2 ]]; then
                log_error "Usage: ./dev.sh auto-report <project_dir>"
                exit 1
            fi
            auto_report "$2"
            ;;
        weaver-generate)
            check_dependencies
            weaver_generate
            ;;
        weaver-validate)
            check_dependencies
            weaver_validate
            ;;
        weaver-forge)
            check_dependencies
            if [[ $# -lt 2 ]]; then
                log_error "Usage: ./dev.sh weaver-forge <template_name>"
                exit 1
            fi
            weaver_forge_template "$2"
            ;;
        clean)
            log_info "Cleaning build artifacts..."
            make clean
            log_success "Cleanup completed"
            ;;
        help|--help|-h)
            show_menu
            ;;
        *)
            log_error "Unknown command: $1"
            echo ""
            show_menu
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
