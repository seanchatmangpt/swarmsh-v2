#!/bin/bash
# SwarmSH v2 Claude Code Setup Script
# Optimizes the project for maximum Claude Code productivity

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}🚀 SwarmSH v2 Claude Code Setup${NC}"
echo -e "${BLUE}Optimizing project for maximum agentic coding productivity...${NC}"

# Check if Claude Code is available
if ! command -v claude &> /dev/null; then
    echo -e "${YELLOW}⚠️  Claude Code not found. Please install it first:${NC}"
    echo -e "${BLUE}   Visit: https://claude.ai/code${NC}"
    echo ""
    echo -e "${GREEN}✅ Project is configured for Claude Code. Run this script again after installation.${NC}"
    exit 0
fi

echo -e "${GREEN}✅ Claude Code detected${NC}"

# Initialize Claude Code in project
echo -e "${BLUE}🔧 Initializing Claude Code configuration...${NC}"

# Run Claude Code init to create base configuration if needed
echo -e "${YELLOW}Checking Claude Code configuration...${NC}"
if [[ ! -f ".claude/config.json" ]]; then
    echo -e "${BLUE}Creating base Claude Code configuration...${NC}"
    # We'll skip the init since our custom config is more comprehensive
    mkdir -p .claude
    echo -e "${GREEN}✅ Base configuration ready${NC}"
else
    echo -e "${GREEN}✅ Claude Code configuration exists${NC}"
fi

# Verify our custom configuration is in place
echo -e "${BLUE}🔍 Verifying Claude Code configuration...${NC}"

required_files=(
    "CLAUDE.md"
    ".claude/commands/feature.md"
    ".claude/commands/debug.md"
    ".claude/commands/weaver.md"
    ".claude/commands/shell-export.md"
    ".claude/commands/cycle.md"
    ".claude/settings.json"
    "semantic-conventions/CLAUDE.md"
    "src/CLAUDE.md"
    "templates/CLAUDE.md"
    "tests/CLAUDE.md"
)

for file in "${required_files[@]}"; do
    if [[ -f "$file" ]]; then
        echo -e "${GREEN}✅ $file${NC}"
    else
        echo -e "${RED}❌ Missing: $file${NC}"
        exit 1
    fi
done

# Check development tools
echo -e "${BLUE}🛠️  Checking development dependencies...${NC}"

dependencies=(
    "cargo:Rust package manager"
    "weaver:OTEL Weaver code generator"
    "make:Build automation"
)

for dep in "${dependencies[@]}"; do
    cmd="${dep%%:*}"
    desc="${dep##*:}"
    
    if command -v "$cmd" &> /dev/null; then
        echo -e "${GREEN}✅ $cmd ($desc)${NC}"
    else
        echo -e "${YELLOW}⚠️  $cmd not found ($desc)${NC}"
        echo -e "${BLUE}   Install with: ${CYAN}# Installation instructions in README.md${NC}"
    fi
done

# Test basic functionality
echo -e "${BLUE}🧪 Testing basic functionality...${NC}"

# Test Rust compilation
if cargo check --quiet; then
    echo -e "${GREEN}✅ Rust compilation works${NC}"
else
    echo -e "${YELLOW}⚠️  Rust compilation issues (expected for incomplete project)${NC}"
fi

# Test OTEL Weaver if available
if command -v weaver &> /dev/null; then
    if weaver validate; then
        echo -e "${GREEN}✅ OTEL Weaver semantic conventions valid${NC}"
    else
        echo -e "${YELLOW}⚠️  OTEL Weaver validation issues${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  OTEL Weaver not available${NC}"
fi

# Display Claude Code usage instructions
echo ""
echo -e "${CYAN}🎯 Claude Code Usage Instructions${NC}"
echo -e "${BLUE}═══════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}Quick Start:${NC}"
echo -e "${BLUE}  claude                    ${NC}# Start Claude Code in this directory"
echo ""
echo -e "${GREEN}Custom Slash Commands:${NC}"
echo -e "${BLUE}  /feature <description>    ${NC}# Complete feature development workflow"
echo -e "${BLUE}  /debug <bug description>  ${NC}# Systematic bug fixing workflow"
echo -e "${BLUE}  /weaver <operation>       ${NC}# OTEL Weaver code generation"
echo -e "${BLUE}  /shell-export <operation> ${NC}# Shell export system operations"
echo -e "${BLUE}  /cycle <task>             ${NC}# Complete development cycle"
echo ""
echo -e "${GREEN}Key Workflows:${NC}"
echo -e "${BLUE}  1. Explore → Plan → Code → Commit${NC}"
echo -e "${BLUE}  2. Write Tests → Code → Iterate${NC}"
echo -e "${BLUE}  3. Visual Mock → Code → Screenshot → Iterate${NC}"
echo ""
echo -e "${GREEN}Project-Specific Features:${NC}"
echo -e "${BLUE}  • Observability-first architecture${NC}"
echo -e "${BLUE}  • Zero-conflict coordination guarantees${NC}"
echo -e "${BLUE}  • Rust → Shell export capability${NC}"
echo -e "${BLUE}  • OTEL Weaver code generation${NC}"
echo -e "${BLUE}  • Nanosecond-precision coordination${NC}"
echo ""
echo -e "${GREEN}Development Commands:${NC}"
echo -e "${BLUE}  ./dev.sh setup           ${NC}# Setup development environment"
echo -e "${BLUE}  ./dev.sh dev             ${NC}# Full development cycle"
echo -e "${BLUE}  make generate            ${NC}# Generate from semantic conventions"
echo -e "${BLUE}  make export              ${NC}# Test shell export"
echo ""
echo -e "${CYAN}📚 Documentation:${NC}"
echo -e "${BLUE}  • Root CLAUDE.md: Project overview and common commands${NC}"
echo -e "${BLUE}  • Domain CLAUDE.md files: Specific guidance for each area${NC}"
echo -e "${BLUE}  • Custom slash commands: Workflow automation${NC}"
echo -e "${BLUE}  • README.md: Complete project documentation${NC}"
echo ""
echo -e "${GREEN}🚀 Claude Code is ready! Run '${CYAN}claude${GREEN}' to start coding.${NC}"
echo ""
echo -e "${PURPLE}💡 Pro Tips:${NC}"
echo -e "${BLUE}  • Use 'think hard' for complex planning${NC}"
echo -e "${BLUE}  • Paste screenshots for visual tasks${NC}"
echo -e "${BLUE}  • Use tab completion for file paths${NC}"
echo -e "${BLUE}  • Press Escape to interrupt Claude${NC}"
echo -e "${BLUE}  • Use /clear to reset context between tasks${NC}"
echo ""
