# SwarmSH v2 - Documentation Map

**Complete guide to all documentation organized by purpose and audience.**

This document helps you navigate the entire documentation ecosystem using the Diataxis framework.

---

## 📍 Navigation by Purpose

### 🎓 "I'm new to SwarmSH v2"

```
START HERE
    ↓
GETTING_STARTED.md (5 min)  ← Installation, setup, first example
    ↓
README.md - Tutorials (10 min) ← Step-by-step learning
    ↓
    ├→ Basic coordination
    ├→ Add observability (OTEL)
    └→ Shell export
    ↓
Examples in /examples/ ← Learn by reading code
```

### 🛠️ "I need to do something specific"

```
TROUBLESHOOTING.md (10 min) ← Find your specific issue
    ↓
    ├→ Compilation issues
    ├→ Runtime errors
    ├→ Performance problems
    └→ Configuration issues
    ↓
README.md - How-To Guides ← Detailed solutions
```

### 📖 "I need to understand something"

```
README.md - Explanation (20 min) ← Conceptual overview
    ↓
    ├→ File-based coordination design
    ├→ Telemetry & observability
    ├→ Shell export rationale
    ├→ Coordination patterns
    └→ Trade-off analysis
    ↓
POKA_YOKE_GUIDE.md (20 min) ← 80/20 dark matter
    ↓
    ├→ Assumption failures
    ├→ State desynchronization
    ├→ Silent degradation
    └→ Prevention patterns
```

### 🚨 "Something is broken in production"

```
FMEA_ANALYSIS.md (5 min) ← Understand the failure
    ↓
    ├→ RPN score
    ├→ Symptoms
    └→ Root causes
    ↓
FMEA_RISK_MITIGATION.md (15-60 min) ← Fix it
    ↓
    ├→ Diagnostic procedures
    ├→ Immediate mitigation
    ├→ Root cause analysis
    ├→ Permanent fixes
    └→ Recovery procedures
    ↓
TROUBLESHOOTING.md ← If you need more help
```

### 🛡️ "I want to prevent problems"

```
POKA_YOKE_GUIDE.md (30 min) ← Learn prevention
    ↓
    ├→ Pattern 1: Atomic Operations
    ├→ Pattern 2: Version Validation
    ├→ Pattern 3: Fail Fast
    ├→ Pattern 4: Automatic Cleanup
    ├→ Pattern 5: Config Immutability
    └→ Pattern 6: Circuit Breakers
    ↓
Tests in /tests/ ← See implementations
```

### 🔌 "I need the API documentation"

```
API_REFERENCE.md (15 min) ← Complete API
    ↓
    ├→ CoordinationEngine
    ├→ AgentSpec
    ├→ WorkSpec
    ├→ Traits (CoordinationPattern)
    ├→ Error handling
    └→ Feature flags
    ↓
cargo doc --open ← Generated documentation
```

### 📊 "I need to understand all failure modes"

```
FMEA_ANALYSIS.md (20 min) ← All 10 failure modes
    ↓
    ├→ RPN 225: Race Conditions
    ├→ RPN 192: Telemetry Loss
    ├→ RPN 180: Lock Accumulation
    ├→ RPN 168: Health Check False Positives
    ├→ ... (5 more)
    ↓
FMEA_RISK_MITIGATION.md ← How to respond
POKA_YOKE_GUIDE.md ← How to prevent
```

---

## 📚 Documentation by Type (Diataxis Framework)

### 1️⃣ Tutorials (Learn by Doing)

**Purpose**: Get started with SwarmSH v2

| Document | Length | Topics |
|----------|--------|--------|
| [GETTING_STARTED.md](./GETTING_STARTED.md) | 5-10 min | Installation, first program, environment setup |
| [README.md - Tutorials](#-tutorials) | 15 min | Basic coordination, observability, shell export |
| [Examples](/examples/) | Varies | Working code samples |

**How to use**: Follow step-by-step. Code along if possible.

### 2️⃣ How-To Guides (Solve Specific Problems)

**Purpose**: Do real work with SwarmSH v2

| Document | Length | Purpose |
|----------|--------|---------|
| [README.md - How-To Guides](#-how-to-guides) | 20 min | Export to shell, health checks, custom patterns |
| [TROUBLESHOOTING.md](./TROUBLESHOOTING.md) | 10-30 min | Fix compilation, runtime, performance issues |
| [FMEA_RISK_MITIGATION.md](./FMEA_RISK_MITIGATION.md) | 30-60 min | Respond to production failures |
| [POKA_YOKE_GUIDE.md](./POKA_YOKE_GUIDE.md) | 30 min | Prevent common mistakes |

**How to use**: Jump to your specific problem. Follow the procedures.

### 3️⃣ Reference (Look Up Details)

**Purpose**: Find exact information

| Document | Purpose |
|----------|---------|
| [API_REFERENCE.md](./API_REFERENCE.md) | Complete API signatures and types |
| [README.md - Reference](#-reference) | Configuration, environment variables, file structure |
| [FMEA_ANALYSIS.md](./FMEA_ANALYSIS.md) | Failure mode definitions and impact analysis |
| [CHANGELOG.md](./CHANGELOG.md) | Version history and changes |

**How to use**: Search for what you need. Read the relevant section.

### 4️⃣ Explanation (Understand Concepts)

**Purpose**: Understand why and how

| Document | Length | Topics |
|----------|--------|--------|
| [README.md - Explanation](#-explanation) | 30 min | File-based coordination, patterns, telemetry, trade-offs |
| [POKA_YOKE_GUIDE.md](#80-20-dark-matter-hidden-failure-modes) | 20 min | Hidden complexity, failure root causes |
| [Context](/context/) | Varies | Project vision and status |

**How to use**: Read for understanding. Don't skip—this builds intuition.

---

## 🗺️ Documentation Hierarchy

```
📖 README.md (Main documentation)
   ├── 🎯 Quick Facts
   ├── 🎓 Tutorials
   │   ├── Basic coordination
   │   └── Add observability
   ├── 🛠️ How-To Guides
   │   ├── Shell export
   │   ├── Health monitoring
   │   └── Custom patterns
   ├── 📖 Reference
   │   ├── Architecture
   │   ├── API overview
   │   └── Configuration
   ├── 💡 Explanation
   │   ├── Coordination design
   │   ├── Telemetry system
   │   └── Shell export design
   └── 🚀 Getting Help
       ├── Documentation structure
       └── Contributing guidelines

📄 GETTING_STARTED.md (Installation & first steps)
   ├── Prerequisites
   ├── Installation
   ├── Verification
   ├── Configuration
   ├── Examples (2)
   ├── Health monitoring
   ├── Shell export
   └── Troubleshooting quick ref

🔗 API_REFERENCE.md (API documentation)
   ├── Core types
   │   ├── CoordinationEngine
   │   ├── AgentSpec
   │   ├── WorkSpec
   │   ├── Work
   │   └── Config
   ├── Traits
   │   └── CoordinationPattern
   ├── Error handling
   ├── Feature flags
   ├── Environment variables
   └── Complete example

🚨 TROUBLESHOOTING.md (Problem-solving)
   ├── Quick reference table
   ├── Compilation issues
   ├── Runtime issues
   │   ├── No agents found
   │   ├── Lock failures
   │   ├── Disk full
   │   ├── OTEL issues
   │   ├── Test failures
   │   └── Config issues
   ├── Performance issues
   ├── Debugging techniques
   └── Escalation procedures

🛡️ POKA_YOKE_GUIDE.md (Prevention & design)
   ├── 80/20 Dark Matter
   │   ├── Assumption failures (35%)
   │   ├── State desynchronization (25%)
   │   ├── Silent degradation (20%)
   │   ├── Resource accumulation (15%)
   │   └── Configuration drift (5%)
   ├── 6 Core Patterns
   │   ├── Atomic operations
   │   ├── Version validation
   │   ├── Fail fast
   │   ├── Automatic cleanup
   │   ├── Config immutability
   │   └── Circuit breakers
   └── Testing patterns

📊 FMEA_ANALYSIS.md (Failure modes)
   ├── RPN 225: Race conditions
   ├── RPN 192: Telemetry loss
   ├── RPN 180: Lock accumulation
   ├── RPN 168: Health false positives
   ├── RPN 140: File corruption
   ├── RPN 126: Semantic drift
   ├── RPN 96: State desync
   ├── RPN 80: Resource exhaustion
   ├── RPN 70: Deadlock
   └── RPN 48: Config issues

🚀 FMEA_RISK_MITIGATION.md (Runbooks)
   ├── 10 failure runbooks
   │   ├── Race conditions
   │   ├── Telemetry loss
   │   ├── Lock accumulation
   │   └── ... (7 more)
   ├── Diagnostic procedures
   ├── Immediate mitigation
   ├── Root cause analysis
   └── Permanent fixes
```

---

## 🎯 Quick Navigation by Role

### For Developers

```
New to SwarmSH v2?
  → GETTING_STARTED.md
  → README.md - Tutorials
  → /examples

Building features?
  → API_REFERENCE.md
  → README.md - Tutorials & How-To
  → Tests in /tests

Debugging issues?
  → TROUBLESHOOTING.md
  → FMEA_ANALYSIS.md
  → grep -r "error" logs/
```

### For DevOps/Operations

```
Setting up production?
  → GETTING_STARTED.md
  → README.md - How-To: Health Monitoring
  → FMEA_ANALYSIS.md

Something's broken?
  → FMEA_RISK_MITIGATION.md
  → TROUBLESHOOTING.md
  → Monitor with OTEL

Preventing problems?
  → POKA_YOKE_GUIDE.md
  → FMEA_ANALYSIS.md
  → Health monitoring setup
```

### For Architects

```
Understanding design?
  → README.md - Explanation
  → POKA_YOKE_GUIDE.md (80/20 dark matter)
  → FMEA_ANALYSIS.md

Planning for reliability?
  → FMEA_ANALYSIS.md
  → FMEA_RISK_MITIGATION.md
  → POKA_YOKE_GUIDE.md

Understanding patterns?
  → README.md - Explanation (Coordination Patterns)
  → /src/coordination.rs
  → Tests in /tests
```

### For DevEx/Documentation

```
Improving docs?
  → This file (DOCUMENTATION_MAP.md)
  → README.md - Contributing guidelines
  → DOCUMENTATION_ANALYSIS.md (in repo)

Following Diataxis framework?
  → Tutorials: Getting started, learning
  → How-To: Real tasks, problem-solving
  → Reference: Exact information
  → Explanation: Understanding concepts

Seeing what's needed?
  → DOCUMENTATION_ANALYSIS.md
  → DOCUMENTATION_SUMMARY.txt
  → README.md - Known gaps
```

---

## 📊 Documentation Statistics

| Category | Count | Lines | Status |
|----------|-------|-------|--------|
| **Tutorials** | 2 | 150 | ✅ Complete |
| **How-To Guides** | 4 | 1,200+ | ✅ Complete |
| **Reference** | 3 | 1,100+ | ✅ Complete |
| **Explanation** | 3 | 1,500+ | ✅ Complete |
| **Quality (FMEA)** | 3 | 1,500+ | ✅ Complete |
| **Total** | **15** | **5,450+** | ✅ **Complete** |

---

## 🔄 How to Update Documentation

### Adding a New Document

1. **Decide the type**: Tutorials, How-To, Reference, or Explanation
2. **Follow the Diataxis framework** for your document type
3. **Add to this map** (DOCUMENTATION_MAP.md)
4. **Link from README.md**
5. **Cross-link** to related documents

### Keeping Documentation Consistent

- **Single source of truth**: README.md is the main guide
- **Hyperlinks**: Every document links to related ones
- **Cross-references**: Use `[text](./path/to/doc.md#section)`
- **Parallel structure**: Same topics appear in multiple docs (intentional)
- **Complementary**: Each doc serves a different purpose

### Quality Guidelines

✅ **Do**:
- Use clear, concrete examples
- Include code samples
- Link to related docs
- Update when changing code
- Test procedures before documenting

❌ **Don't**:
- Make unsupported claims (see README.md)
- Write marketing copy in technical docs
- Leave outdated sections
- Break links when refactoring
- Document incomplete features

---

## 🎓 Learning Paths

### Path 1: Quick Start (30 minutes)

```
1. README.md intro (5 min)
2. GETTING_STARTED.md (10 min)
3. First example (5 min)
4. README.md - Quick Reference (10 min)

→ Ready to use SwarmSH v2 for basic tasks
```

### Path 2: Becoming Proficient (2 hours)

```
1. Quick Start path above (30 min)
2. README.md - Tutorials (20 min)
3. API_REFERENCE.md (20 min)
4. README.md - How-To Guides (20 min)
5. TROUBLESHOOTING.md (quick scan, 10 min)

→ Can solve most common problems
```

### Path 3: Production Ready (4 hours)

```
1. Becoming Proficient path above (2 hours)
2. FMEA_ANALYSIS.md (20 min)
3. POKA_YOKE_GUIDE.md (30 min)
4. FMEA_RISK_MITIGATION.md (skim, 20 min)
5. README.md - Explanation (30 min)
6. Deep dive on 2-3 topics (30 min)

→ Ready for production deployment
```

### Path 4: Expert (6 hours)

```
1. Production Ready path above (4 hours)
2. Complete FMEA_RISK_MITIGATION.md (1 hour)
3. Complete POKA_YOKE_GUIDE.md (1 hour)
4. Review all tests in /tests (varies)
5. Read key implementation files:
   - src/coordination.rs
   - src/telemetry.rs
   - src/shell_export.rs

→ Can design and implement extensions
```

---

## ✅ Documentation Completeness Checklist

- ✅ Getting started guide with step-by-step instructions
- ✅ API reference with all public types and methods
- ✅ How-to guides for common tasks
- ✅ Troubleshooting guide with quick lookup
- ✅ FMEA analysis identifying 10 failure modes
- ✅ Poka yoke guide with 6 prevention patterns
- ✅ Risk mitigation runbooks for each failure
- ✅ README with complete reference
- ✅ Clear linking between documents
- ✅ Diataxis framework applied throughout
- ✅ 80/20 dark matter documented
- ✅ This navigation document

**Total documentation**: ~5,450+ lines across 15 documents

---

## 🔗 External Resources

### OpenTelemetry
- [OTEL Documentation](https://opentelemetry.io/docs/)
- [Jaeger Getting Started](https://www.jaegertracing.io/docs/getting-started/)
- [Semantic Conventions](https://opentelemetry.io/docs/specs/semconv/)

### File-Based Coordination
- [Advisory File Locks](https://en.wikipedia.org/wiki/File_locking)
- [POSIX File Locking](https://pubs.opengroup.org/onlinepubs/9699919799/functions/fcntl.html)

### Testing & Quality
- [FMEA Handbook](https://asq.org/)
- [Poka-Yoke](https://www.lean.org/lexicon-terms/poka-yoke/)
- [Chicago TDD 80/20 Framework](https://www.industriallogic.com/blog/)

---

**Last Updated**: 2025-11-16
**Framework**: Diataxis Documentation System
**Status**: Complete and navigable
