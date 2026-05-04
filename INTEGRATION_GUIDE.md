# OBINexus Integration Guide

Complete guide to integrating the OBINexus self-aware probing system into your projects.

## System Overview

OBINexus implements bidirectional probing for AI self-introspection:

```
┌────────────────────────────────────────────┐
│         OBINexus System                    │
│                                            │
│   Knowledge Base (Symbol Table)           │
│   ├─ Facts learned                        │
│   ├─ Patterns recognized                  │
│   └─ Confidence levels                    │
│                                            │
│   Execution State                         │
│   ├─ Variable bindings                    │
│   ├─ Call stack                           │
│   └─ Execution history                    │
│                                            │
│   Probing Cycle                           │
│   ├─ p(ext): State → Data (observe)      │
│   └─ p(int): Data → State (learn)        │
│                                            │
│   Self-Questioning                        │
│   ├─ Who? (identity)                      │
│   ├─ What? (nature)                       │
│   ├─ When? (temporal)                     │
│   ├─ Where? (problem space)               │
│   ├─ Why? (reason)                        │
│   └─ How? (mechanism)                     │
│                                            │
│   Validation                              │
│   └─ Coherence Check (95.4% standard)    │
│                                            │
└────────────────────────────────────────────┘
```

## Mathematical Framework

### Probe Definitions

**External Probe** - Observe the system's current state:
```
p(ext) : State → Data
Given: Current execution state S
Returns: Observable data representation D
Purpose: Convert internal state to external representation
```

**Internal Probe** - Process data to update state:
```
p(int) : Data → State
Given: Input data D
Returns: Updated execution state S
Purpose: Learn from input and modify behavior
```

### Bidirectional Consistency

The system maintains consistency through:
```
p(int) ∘ p(ext) : State → Data → State'
```

Where State' should be coherent with the original State.

### Three Poles of Consensus

Responses are constrained to three values:
- **YES** (1): Definite positive
- **NO** (0): Definite negative
- **MAYBE** (0.5): Uncertain

## Core Concepts

### 1. Symbol Table (Knowledge Base)

A key-value store with confidence weighting:

```
Symbol("color", "blue", confidence=0.95)
Symbol("size", "large", confidence=0.8)
Symbol("purpose", "introspection", confidence=1.0)
```

- **O(log n)** lookup time
- Confidence scores track certainty
- Supports pattern queries

### 2. Execution State

Tracks the runtime context:

```
Stack Frame 0 (global):
  - Variable: "system_name" = "OBINexus"
  - Variable: "status" = "operational"

Stack Frame 1 (function_call):
  - Variable: "local_x" = 42
  - Variable: "local_y" = 3.14
```

- Maintains call stack
- Global and local scopes
- Instruction counter

### 3. Temporal History (Filter-Flash)

Records all events in a DAG:

```
Event 1: Learning("fact1", "value1") → t=1000
Event 2: Learning("fact2", "value2") → t=1001
Event 3: Flash("Insight about fact1") → t=1002
Event 4: Filter("Contradiction in state") → t=1003
Event 5: Resolution("Corrected fact1") → t=1004
```

**Filter**: Problem detection
**Flash**: Insight moments

### 4. Dimensional Problem Space

3D strategic positioning:

```
Point P = (0.7, 0.3, 0.5) representing:
  Offense:  0.7 (aggressive approach)
  Defense:  0.3 (minimal protection)
  Attack:   0.5 (balanced challenging)
```

Used for strategic reasoning about problems.

### 5. Canonical Questions

Six fundamental self-queries:

| Q | Asks | Measures |
|---|------|----------|
| Who | Identity | Self-recognition |
| What | Nature | Self-description |
| When | Timing | Temporal context |
| Where | Position | Problem space location |
| Why | Reason | Causal understanding |
| How | Method | Operational mechanics |

## Usage Patterns

### Pattern 1: Knowledge Acquisition

```python
system = OBINexus()

# Learn new facts
system.learn("color", "blue")
system.learn("shape", "square")
system.learn("size", "large")

# Query knowledge
status = system.status()
print(f"Learned {status['knowledge_base_size']} facts")
```

### Pattern 2: Probing Cycle

```python
# 1. Observe current state
external_result = system.probe_external()
print(f"Current state: {external_result.data}")

# 2. Process input
input_data = "mode=learning\nstatus=active"
internal_result = system.probe_internal(input_data)
print(f"Processed: {internal_result}")

# 3. Verify consistency
consistency = internal_result.confidence
if consistency > 0.8:
    print("✓ State update successful")
```

### Pattern 3: Self-Reflection

```python
# Ask about self
print(system.ask("who"))     # Identity
print(system.ask("what"))    # Current state
print(system.ask("where"))   # Problem positioning
print(system.ask("why"))     # Reason for state
print(system.ask("how"))     # Operational details

# Get deep reflection
reflection = system.reflect()
print(reflection)
```

### Pattern 4: Coherence Verification

```python
# Check if system meets 95.4% standard
score = system.coherence_score()

if score >= 0.954:
    print(f"✓ Coherence {score:.1%} - Safe for deployment")
else:
    print(f"⚠ Coherence {score:.1%} - Needs calibration")
```

### Pattern 5: State Export/Import

```python
# Export for analysis or persistence
knowledge = system.export_knowledge()  # JSON
history = system.export_history()      # JSON

# Can be saved to file for later restoration
with open("state.json", "w") as f:
    f.write(knowledge)
```

## Integration Examples

### Example 1: Chatbot with Self-Awareness

```python
from obi_py import OBINexus

class SelfAwareBot:
    def __init__(self):
        self.obi = OBINexus(verbose=True)
        self.obi.learn("role", "conversational agent")
        
    def respond(self, user_input):
        # Learn from interaction
        self.obi.learn("last_input", user_input)
        
        # Probe state
        state = self.obi.probe_external()
        
        # Answer based on self-understanding
        if "who" in user_input.lower():
            return self.obi.ask("who")
        elif "what" in user_input.lower():
            return self.obi.ask("what")
        else:
            return f"I understood: {user_input}"
        
    def explain_self(self):
        return self.obi.reflect()
```

### Example 2: Adaptive Learning System

```python
from obi_py import OBINexus

class AdaptiveSystem:
    def __init__(self):
        self.obi = OBINexus()
        self.performance = []
        
    def train(self, examples):
        for example, label in examples:
            self.obi.learn(f"example_{len(self.performance)}", str(example))
            self.performance.append(label)
            
            # Check coherence during training
            coherence = self.obi.coherence_score()
            if coherence < 0.8:
                print("⚠ Coherence dropping - may need regularization")
                
    def predict(self, input_data):
        # Consult self-knowledge
        status = self.obi.status()
        knowledge_size = status['knowledge_base_size']
        
        if knowledge_size < 10:
            return "UNCERTAIN"  # Not enough learned
        else:
            return "CONFIDENT"  # Well calibrated
```

### Example 3: Debugging Assistant

```python
from obi_py import OBINexus

class DebugAssistant:
    def __init__(self):
        self.obi = OBINexus()
        
    def debug_session(self, errors):
        # Record what went wrong
        for i, error in enumerate(errors):
            self.obi.learn(f"error_{i}", str(error))
            
        # Ask probing questions
        print("=== Debug Analysis ===")
        print("Where did this occur?")
        print(self.obi.ask("where"))
        
        print("\nWhy might this have happened?")
        print(self.obi.ask("why"))
        
        print("\nHow should we fix it?")
        print(self.obi.ask("how"))
```

## Performance Characteristics

### Complexity Analysis

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Symbol insert | O(log n) | IndexMap with order preservation |
| Symbol lookup | O(log n) | O(1) average with hash |
| Probe external | O(n) | Iterate all variables |
| Probe internal | O(m) | m = input data size |
| Query answer | O(h) | h = history length |
| Coherence check | O(n+m) | Full state analysis |

### Memory Usage

- Symbol table: ~100 bytes per symbol
- Execution state: ~1KB + frame overhead
- History events: ~200 bytes per event
- Typical system: <10MB for 10k symbols

## Best Practices

### 1. Regular Coherence Checks

```python
# Check periodically during operation
if step % 100 == 0:
    coherence = system.coherence_score()
    if coherence < 0.954:
        # Trigger recalibration
        system.learn("calibration_needed", "true")
```

### 2. Bounded History

```python
# Prevent memory bloat
if len(system.history) > 10000:
    system.history.prune(keep_count=5000)
```

### 3. Confident Knowledge

```python
# Use confidence levels to distinguish strong vs weak beliefs
system.learn_with_confidence("certain_fact", value, confidence=1.0)
system.learn_with_confidence("probable_fact", value, confidence=0.7)
```

### 4. Strategic Positioning

```python
# Maintain balanced dimensional presence
current = system.dimensions.current()
avg = (current.offense + current.defense + current.attack) / 3
if abs(current.offense - avg) > 0.3:
    # System is too offense-biased, rebalance
    system.dimensions.move_to(balanced_point)
```

## Deployment Checklist

- [ ] Coherence >= 95.4%
- [ ] Symbol table size > 50
- [ ] No contradictions in knowledge
- [ ] History length > 100 events
- [ ] Stack depth < 100
- [ ] All canonical questions answerable
- [ ] Probing cycle consistent
- [ ] State snapshots valid

## Troubleshooting

### Problem: Low Coherence Score

**Cause**: Insufficient knowledge or inconsistencies

**Solution**:
```python
# Add more facts
for i in range(50):
    system.learn(f"fact_{i}", f"value_{i}")

# Check for contradictions
for symbol in system.symbol_database.all():
    if symbol.confidence < 0.7:
        # Investigate low-confidence facts
        print(f"⚠ Uncertain: {symbol}")
```

### Problem: Inconsistent Probes

**Cause**: Internal state corruption or malformed data

**Solution**:
```python
# Verify consistency
external = system.probe_external()
if external.confidence < 0.7:
    # Reset and reload
    system = OBINexus()
    # Relearn critical facts
```

### Problem: Stack Overflow

**Cause**: Excessive recursion or deep frame nesting

**Solution**:
```python
# Monitor stack depth
snapshot = state.snapshot()
if snapshot.frame_depth > 50:
    # Unwind frames
    while state.stack_depth() > 10:
        state.pop_frame()
```

## Advanced Topics

### Custom Query Types

Extend canonical questions with domain-specific queries:

```python
class ExtendedOBINexus(OBINexus):
    def ask_extended(self, question):
        if question == "what_next":
            return self._answer_what_next()
        return self.ask(question)
    
    def _answer_what_next(self):
        # Custom logic based on current state
        ...
```

### Bayesian Integration

Combine with probability models:

```python
from obi_py import OBINexus

system = OBINexus()

# Track probabilistic facts
system.learn_with_confidence("hypothesis", value, confidence=0.75)

# Update confidence as evidence accumulates
system.update_confidence("hypothesis", 0.85)
```

### Temporal Analysis

Analyze learning patterns:

```python
events = system.history.all_events()

# Find acceleration of learning
learning_events = [e for e in events if e.event_type == "Learning"]
print(f"Learning rate: {len(learning_events)} facts/session")
```

## References

- **OBINexus Project**: Original framework and philosophy
- **Filter-Flash Model**: Consciousness modeling through epistemology
- **Dimensional Game Theory**: Strategic problem decomposition
- **Bayesian Inference**: Unbiased reasoning mechanisms

## Support

For issues or questions:
1. Check the README.md
2. Review the example programs
3. Check existing issues/documentation
4. Attribute the OBINexus project in your work

---

**Version**: 0.1.0  
**Status**: Operational and ready for integration  
**Last Updated**: 2026-05
