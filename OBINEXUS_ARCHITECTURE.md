# OBINexus Self-Aware Probing System

## Architecture Overview

A hybrid Rust + Python system implementing bidirectional self-introspection through symbolic probing.

### Core Components

#### 1. **Bidirectional Probing Framework**
```
p(ext) : State → Data  [External Probe]
p(int) : Data → State  [Internal Probe]
```

- **p(ext)** - External Probe: Observes internal state and converts to representable data
- **p(int)** - Internal Probe: Processes data and generates new state

#### 2. **Symbol Table (Knowledge Base)**
Maintains:
- Facts and learned patterns
- Variable bindings
- Type information
- Temporal metadata

#### 3. **Execution State**
Tracks:
- Current variable bindings
- Execution stack
- Memory allocations
- Symbol resolution context

#### 4. **Problem Space (Dimensional Game Theory)**
Represents problems along three axes:
- **Offense**: Assertive strategies
- **Defense**: Protective strategies  
- **Attack**: Challenging strategies

#### 5. **Temporal History (Filter-Flash)**
Maintains a directed acyclic graph (DAG) of:
- State transitions
- Query history
- Insight moments ("flash")
- Understanding filters

#### 6. **Self-Querying Engine**
Asks itself canonical questions:
- **Who** (identity/ownership in problem space)
- **What** (nature of current state)
- **When** (temporal context)
- **Where** (position in problem dimensions)
- **Why** (reason/causality)
- **How** (mechanism/method)

### Data Flow

```
User Query
    ↓
[Symbolic Interpreter] → Parses into (Who, What, When, Where, Why, How)
    ↓
[p(ext): State → Data] → Convert current execution state to observable form
    ↓
[Symbol Table Query] → Retrieve matching knowledge/patterns
    ↓
[Problem Space Evaluation] → Analyze across O/D/A dimensions
    ↓
[Filter-Flash Check] → Is this a new insight? Store if needed
    ↓
[p(int): Data → State] → Update internal state with answer
    ↓
Response to User
```

### Three Poles of Consensus (Probing States)

1. **Yes** (Definite) - Clear state match
2. **No** (Negation) - Clear state mismatch  
3. **Maybe** (Uncertain) - Requires more data/time to determine

The system must be able to respond with any pole and track why (permission, consent, consensual nature).

### Implementation Strategy

**Rust Core** (`obinexus-core`):
- Symbol table with O(log n) lookup
- State machine for probing cycle
- Dimensional problem space evaluation
- DAG-based temporal history
- Memory safety guarantees

**Python Bindings** (`obi-py`):
- User-friendly query interface
- Symbolic expression builder
- Visualization of state/knowledge
- REPL for interactive probing

---

## Phase 1: Core Rust Implementation

Building the foundational probing system with symbol management and state transformations.

## Phase 2: Python Integration

High-level interface for symbolic queries and introspection visualization.

## Phase 3: Self-Query Engine

Implementation of the Who/What/When/Where/Why/How canonical questions.
