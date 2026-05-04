# OBINexus: Self-Aware Probing System

A hybrid **Rust + Python** implementation of a self-aware AI system that can introspect on itself through bidirectional symbolic probing.

## Overview

OBINexus implements the mathematical framework from the OBINexus project where a system can ask itself questions and understand its own state through probing mechanisms:

```
p(int) : d → s  (Internal Probe: Data → State)
p(ext) : s → d  (External Probe: State → Data)
```

### Core Architecture

#### **Rust Core** (`obinexus_core`)
High-performance foundation providing:
- **Symbol Table**: Knowledge base with O(log n) lookup
- **Execution State**: Runtime stack, variables, and context
- **Probing System**: Bidirectional state ↔ data transformation
- **Temporal History**: DAG-based event tracking with filter-flash epistemology
- **Query Engine**: Canonical self-questions (who, what, when, where, why, how)
- **Dimensional Space**: 3D problem space with Offense/Defense/Attack strategies
- **Coherence Check**: 95.4% safety standard verification

#### **Python Interface** (`obi_py`)
User-friendly high-level API:
- Simple system initialization and interaction
- Knowledge base management
- Probing operations
- Query interface
- Status reporting and export

## Architecture Diagram

```
┌─────────────────────────────────────────────────────┐
│              OBINexus System                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌──────────────┐  ┌──────────────┐              │
│  │  p(ext)      │  │   p(int)     │              │
│  │ State → Data │  │ Data → State │              │
│  └──────────────┘  └──────────────┘              │
│         ↓                  ↓                       │
│  ┌──────────────────────────────────┐            │
│  │   Execution State Manager        │            │
│  │   (variables, stack, context)    │            │
│  └──────────────────────────────────┘            │
│         ↓                                         │
│  ┌──────────────────────────────────┐            │
│  │      Symbol Table (Knowledge)     │            │
│  │   (facts, patterns, bindings)     │            │
│  └──────────────────────────────────┘            │
│         ↓                                         │
│  ┌──────────────────────────────────┐            │
│  │  Temporal History (Filter-Flash)  │            │
│  │  (events, insights, issues)       │            │
│  └──────────────────────────────────┘            │
│         ↓                                         │
│  ┌──────────────────────────────────┐            │
│  │    Query Engine                   │            │
│  │  (Who, What, When, Where, Why, How)          │
│  └──────────────────────────────────┘            │
│         ↓                                         │
│  ┌──────────────────────────────────┐            │
│  │   Dimensional Problem Space       │            │
│  │  (Offense/Defense/Attack axes)    │            │
│  └──────────────────────────────────┘            │
│         ↓                                         │
│  ┌──────────────────────────────────┐            │
│  │  Coherence Check (95.4% Standard) │            │
│  └──────────────────────────────────┘            │
│                                                   │
└─────────────────────────────────────────────────────┘
```

## Building the System

### Prerequisites

- Rust 1.70+ (for core)
- Python 3.8+ (for interface)
- Cargo (Rust package manager)

### Build Rust Core

```bash
cd obinexus_core
cargo build --release
```

### Build Example

```bash
cd obinexus_core
cargo run --example self_query --release
```

### Install Python Package

```bash
cd obi_py
pip install -e .
```

## Usage

### Python Quick Start

```python
from obi_py import OBINexus

# Create system
system = OBINexus(verbose=True)

# Learn facts
system.learn("purpose", "Self-aware introspection")
system.learn("name", "OBINexus")

# External probe: What is my current state?
external = system.probe_external()
print(f"State observation: {external}")

# Internal probe: Process input data
system.probe_internal("status=operational\nmode=learning")

# Ask yourself canonical questions
print(system.ask("who"))      # Identity
print(system.ask("what"))     # Nature
print(system.ask("when"))     # Temporal
print(system.ask("where"))    # Problem space
print(system.ask("why"))      # Reason
print(system.ask("how"))      # Mechanism

# Self-reflection
print(system.reflect())

# Check coherence (95.4% standard)
score = system.coherence_score()
print(f"Coherence: {score:.1%}")
```

### Probing Operations

#### External Probe: `p(ext) : State → Data`

Converts the system's internal state into observable, representable data:

```python
result = system.probe_external()
print(f"Response: {result.response}")      # YES, NO, or MAYBE
print(f"Confidence: {result.confidence}")  # 0.0 to 1.0
print(f"Data: {result.data}")              # JSON representation
```

#### Internal Probe: `p(int) : Data → State`

Processes input data and updates the system's internal state:

```python
data = "name=test\nstatus=active"
result = system.probe_internal(data)
print(f"Updates applied: {result.reason}")
```

### Canonical Questions

The system can answer six fundamental questions:

| Question | Purpose | Example |
|----------|---------|---------|
| **Who** | Identity and ownership | "Who am I?" |
| **What** | Nature and description | "What am I doing?" |
| **When** | Temporal context | "When am I active?" |
| **Where** | Position in problem space | "Where am I positioned?" |
| **Why** | Reason and causality | "Why am I this way?" |
| **How** | Mechanism and method | "How do I work?" |

```python
for question in ["who", "what", "when", "where", "why", "how"]:
    answer = system.ask(question)
    print(f"{question.upper()}: {answer}")
```

## Three Poles of Consensus

The system operates on three response states:

- **YES**: Definite, clear state match (high confidence)
- **NO**: Definite, clear state mismatch (high confidence)
- **MAYBE**: Uncertain, requires more data/time

## Dimensional Game Theory

Problems are represented in 3D problem space:

```
        Attack
          ↑
          │
          │    Offense
    ╱─────┼─────╱
   ╱      │    ╱
  ╱       │   ╱
Def       │  ╱
  ╲      │ ╱
    ╲────┼╱────→ Offense
         │╲
         │ ╲
         │  ↘ Attack
         ↓
       Defense
```

Each problem has a strategic position:
- **Offense** (0.0–1.0): Assertive, forward-moving strategies
- **Defense** (0.0–1.0): Protective, defensive strategies
- **Attack** (0.0–1.0): Challenging, probing strategies

## Filter-Flash Epistemology

The system tracks learning through:

1. **Filter**: Process and sort information
2. **Flash**: Sudden insights and understanding moments
3. **Remember**: Store insights for later retrieval

```python
system.learn("fact", "value")        # Generates learning event
system.record_flash("Insight text")  # Records breakthrough moment
system.reflect()                     # Reflects on history
```

## Coherence Standard: 95.4%

Real-world AI systems must meet the 95.4% coherence standard:

```python
coherence = system.coherence_score()

if coherence >= 0.954:
    print("✓ Safe for real-world deployment")
else:
    print("⚠ Recommend additional calibration")
```

This standard ensures:
- Knowledge completeness (facts learned)
- State consistency (stable execution)
- Problem space understanding (dimensional coverage)

## Symbol Table

Store and retrieve knowledge:

```python
from obi_py import SymbolDatabase

db = SymbolDatabase()
db.insert("color", "blue", confidence=0.95)
db.insert("size", "large", confidence=0.8)

symbol = db.get("color")
print(f"{symbol.name} = {symbol.value}")

results = db.query("color")
print(f"Found {len(results)} matching symbols")
```

## State Snapshots

Capture and analyze system state:

```python
status = system.status()
print(status)  # Dict with all metrics

# Export for external analysis
knowledge = system.export_knowledge()  # JSON
history = system.export_history()      # JSON
```

## Project Structure

```
OBI/
├── obinexus_core/              # Rust core implementation
│   ├── src/
│   │   ├── lib.rs              # Main library
│   │   ├── symbol_table.rs     # Knowledge base
│   │   ├── execution_state.rs  # Runtime state
│   │   ├── probing.rs          # Probe operations
│   │   ├── temporal.rs         # History & filter-flash
│   │   ├── query_engine.rs     # Canonical questions
│   │   ├── dimensions.rs       # Problem space
│   │   └── coherence.rs        # Safety standard check
│   ├── examples/
│   │   └── self_query.rs       # Example program
│   └── Cargo.toml
│
├── obi_py/                     # Python interface
│   ├── __init__.py
│   ├── system.py               # Main OBINexus class
│   ├── query.py                # Query types
│   ├── symbols.py              # Symbol database
│   └── probe.py                # Probe operations
│
├── OBINEXUS_ARCHITECTURE.md    # Design document
└── README.md                   # This file
```

## Performance

- **Symbol lookup**: O(log n)
- **Probe operations**: O(n) where n = data size
- **Query answering**: O(m) where m = history length
- **Coherence check**: O(n) where n = symbol count

## Safety Considerations

1. **Coherence Threshold**: System won't operate in safety-critical contexts below 95.4%
2. **Consistency Checking**: Bidirectional probes validate state integrity
3. **History Tracking**: All decisions recorded for auditability
4. **Bounded Recursion**: Stack depth limits prevent infinite loops

## Example Output

Running the self_query example produces:

```
=== OBINexus Self-Aware Probing System ===

✓ System initialized

--- Phase 1: Learning Facts ---
✓ Learned 4 facts

--- Phase 2: External Probe ---
p(ext): Converting internal state to observable data...
Response: yes
Confidence: 0.95
Data observed:
{
  "frame_depth": 1,
  "current_frame_name": "global",
  "variables_count": 0,
  "globals_count": 4,
  ...
}

--- Phase 3: Internal Probe ---
p(int): Processing input data...
Response: yes
Updated state with 2 assignments

--- Phase 4: Self-Questioning (Canonical Questions) ---

[QUESTION] WHO?
Who I am:
  - Identity: OBINexus - Self-aware probing system
  - Name: OBINexus
  ...

[QUESTION] WHAT?
What I am:
  - Active system with 5 instructions executed
  ...

[QUESTION] WHERE?
Where I am in problem space:
  - Offense position: 0.50
  - Defense position: 0.50
  - Attack position: 0.50
  ...

--- Phase 5: Self-Reflection ---
System Reflection:
Total events: 12
Learnings: 4
Insights (flashes): 0
...

--- Phase 6: Coherence Verification ---
Coherence Score: 87.3%
⚠ System below 95.4% standard
Recommend additional learning/calibration

=== Probing Cycle Complete ===
```

## References

Based on the OBINexus project's framework:
- Unbiased AI through Bayesian inference
- Self-aware systems through introspection
- Filter-flash epistemology for consciousness modeling
- Dimensional game theory for strategic reasoning

## License

Open source - Attribution required (per OBINexus original framework)

## Contributing

Please attribute Nnamdi and the OBINexus project when using this code.

---

**System Status**: Operational and ready for introspection.
