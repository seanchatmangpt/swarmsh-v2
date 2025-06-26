#!/bin/bash
# SwarmSH v2 - 80/20 Master Tooling Script
# The ONE script to validate/fix the most important issues

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🚀 SwarmSH v2 - 80/20 Master Tooling"
echo "Solving 80% of problems with 20% of the effort"
echo "Project: $PROJECT_ROOT"
echo ""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

usage() {
    echo "Usage: $0 <command>"
    echo ""
    echo "Commands:"
    echo "  validate    - Run complete validation suite (30 seconds)"
    echo "  fix         - Auto-fix common issues (import errors, weaver validation)"
    echo "  test        - Test shell/Rust parity (functionality preservation)"
    echo "  all         - Run fix → validate → test sequence"
    echo "  status      - Quick status check"
    echo ""
    echo "Examples:"
    echo "  $0 all       # Complete 80/20 workflow"
    echo "  $0 validate  # Just validate current state"
    echo "  $0 fix       # Fix known issues"
}

print_section() {
    echo -e "\n${BLUE}════════════════════════════════════════${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}════════════════════════════════════════${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

cd "$PROJECT_ROOT"

run_validation() {
    print_section "80/20 VALIDATION SUITE"
    
    if [ -f "src/bin/revolutionary_validator.rs" ]; then
        print_success "Running comprehensive validator..."
        cargo run --bin revolutionary_validator
    else
        print_error "Revolutionary validator not found"
        return 1
    fi
}

run_fixes() {
    print_section "80/20 AUTO-FIXES"
    
    # Fix 1: OTEL imports
    print_success "Fixing OTEL imports in binaries..."
    if [ -f "scripts/fix_otel_imports.sh" ]; then
        chmod +x scripts/fix_otel_imports.sh
        ./scripts/fix_otel_imports.sh
    else
        print_error "OTEL import fixer not found"
    fi
    
    echo ""
    
    # Fix 2: Weaver validation
    print_success "Fixing OTEL Weaver validation..."
    if [ -f "scripts/fix_weaver_validation.sh" ]; then
        chmod +x scripts/fix_weaver_validation.sh
        ./scripts/fix_weaver_validation.sh
    else
        print_error "Weaver validation fixer not found"
    fi
}

run_parity_test() {
    print_section "80/20 SHELL/RUST PARITY TEST"
    
    if [ -f "scripts/test_shell_parity.sh" ]; then
        chmod +x scripts/test_shell_parity.sh
        ./scripts/test_shell_parity.sh
    else
        print_error "Shell parity tester not found"
        return 1
    fi
}

run_status() {
    print_section "80/20 QUICK STATUS"
    
    echo "📊 Build Status:"
    if cargo check --lib >/dev/null 2>&1; then
        print_success "Core library compiles"
    else
        print_error "Core library has compilation errors"
    fi
    
    echo ""
    echo "📊 Binary Status:"
    binary_errors=$(cargo check --bins 2>&1 | grep "error\[" | wc -l | tr -d ' ')
    if [ "$binary_errors" -eq 0 ]; then
        print_success "All binaries compile"
    else
        print_warning "$binary_errors binaries have compilation errors"
    fi
    
    echo ""
    echo "📊 Shell Export Status:"
    if [ -d "exported-shell" ] && [ "$(ls -A exported-shell)" ]; then
        print_success "Shell scripts exported"
    else
        print_warning "No shell exports found"
    fi
    
    echo ""
    echo "📊 OTEL Weaver Status:"
    if command -v weaver &> /dev/null; then
        cd semantic-conventions
        if weaver validate . >/dev/null 2>&1; then
            print_success "Semantic conventions are valid"
        else
            print_warning "Semantic convention validation errors"
        fi
        cd ..
    else
        print_warning "OTEL Weaver not installed"
    fi
}

run_all() {
    print_section "80/20 COMPLETE WORKFLOW"
    
    echo "Running complete 80/20 tooling sequence..."
    echo ""
    
    # Step 1: Fix issues
    run_fixes
    
    echo ""
    
    # Step 2: Validate
    if run_validation; then
        validation_success=1
    else
        validation_success=0
    fi
    
    echo ""
    
    # Step 3: Test parity
    if run_parity_test; then
        parity_success=1
    else
        parity_success=0
    fi
    
    echo ""
    print_section "80/20 WORKFLOW RESULTS"
    
    if [ $validation_success -eq 1 ] && [ $parity_success -eq 1 ]; then
        print_success "ALL 80/20 CHECKS PASSED - SwarmSH v2 core claims verified"
    elif [ $validation_success -eq 1 ]; then
        print_warning "Core validation passed, shell parity needs work"
    elif [ $parity_success -eq 1 ]; then
        print_warning "Shell parity good, core validation needs work"
    else
        print_error "Multiple issues detected - focus on fixes first"
    fi
}

# Main command processing
case "${1:-help}" in
    validate)
        run_validation
        ;;
    fix)
        run_fixes
        ;;
    test)
        run_parity_test
        ;;
    all)
        run_all
        ;;
    status)
        run_status
        ;;
    help|--help|-h)
        usage
        ;;
    *)
        echo "Unknown command: $1"
        echo ""
        usage
        exit 1
        ;;
esac

echo ""
echo "🎯 80/20 Tooling Complete"
echo "   For more details, run individual commands or check script output above"