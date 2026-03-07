# Symbolic Gesture Interpreter — UCHE
## Constitutional Computing Symbolic AI for Hand Gesture Recognition

**Date:** March 7, 2026
**Project:** OBINexus Pet Classifier (uche/)
**Architecture:** Probe-based symbolic interpretation with ESG fixed-point construction

---

## Overview

The Symbolic Gesture Interpreter formalizes hand gesture recognition as a **linguistic system** where:

- **Data (D):** Raw hand landmarks + motion vectors from camera
- **State (S):** Semantic command intents (AUTHENTICATE, COMMAND_SIT, etc.)
- **Probe (P):** Bidirectional transformation between D and S

### The Probe Framework

```
P(ext): D -> S  (External Binding: Data implies State)
  D: Landmarks + motion
  ↓ LEXER (tokenize hand configuration + motion)
  ↓ PARSER (build AST from tokens)
  ↓ INTERPRETER (evaluate AST to semantic intent)
  S: CommandNode

P(int): S -> D  (Internal Binding: State implies Data)
  S: Command intent
  ↓ CODE GENERATOR (specification to hand pose)
  D: Gesture specification
```

### Constitutional Channels (CH_0/CH_1/CH_2)

The system applies trident governance:

| Channel | Confidence | Meaning | Action |
|---------|------------|---------|--------|
| **CH_0** | ≥ 0.9 | Observe | Immediate authentication |
| **CH_1** | 0.7-0.9 | Defer | Wait 60s for retry/confirmation |
| **CH_2** | < 0.7 | Collapse | Force to nearest known state |

---

## Architecture

### 1. Lexer (symbolic_interpreter.py: `GestureLexer`)

**Transforms:** Raw landmarks → Token sequence

Tokenizes hand configuration and motion:

```
Input:  landmarks=[21 MediaPipe points], motion=(v_toward=-0.71, v_ortho=0.34)

Tokens:
  [
    Token(TokenType.OK_SIGN, "ok", timestamp=123.0, confidence=0.95),
    Token(TokenType.MOVING_TOWARD, (-0.71, 0.34), timestamp=123.0, confidence=0.92),
  ]

Output: Token list
```

**Hand Configurations:**
- FIST: All fingers closed
- OPEN_PALM: All fingers extended
- POINTING: Index finger only
- PEACE: Index + middle fingers
- THUMBS_UP: Thumb vertical
- OK_SIGN: Thumb + index touching

**Motion Tokens:**
- MOVING_TOWARD: v_toward < -0.5
- MOVING_AWAY: v_toward > 0.5
- MOVING_ORTHO: |v_ortho| > 0.3
- STATIC: No motion

### 2. Parser (symbolic_interpreter.py: `GestureParser`)

**Transforms:** Token sequence → Abstract Syntax Tree

```
Grammar:
  sequence ::= gesture (gesture)*
  gesture  ::= config_token | motion_token | config_token motion_token
```

**AST Node Types:**
- `HandConfigNode`: Leaf node (hand pose)
- `MotionNode`: Leaf node (motion vector)
- `SequenceNode`: Temporal sequence of gestures
- `LogicalNode`: AND/OR/NOT combinations
- `CommandNode`: Semantic intent

**Example AST:**
```
SequenceNode(
  nodes=[
    HandConfigNode(OK_SIGN),
    MotionNode(MOVING_TOWARD, velocity=(-0.71, 0.34))
  ],
  duration=0.5s,
  confidence=0.92
)
```

### 3. Interpreter (symbolic_interpreter.py: `GestureInterpreter`)

**Transforms:** AST → Semantic CommandNode

Maps gesture patterns to **intents**:

```python
Intent Mapping:
  OK_SIGN              -> AUTHENTICATE_BIOMETRIC
  THUMBS_UP            -> CONFIRM_ACTION
  FIST                 -> CANCEL_OPERATION
  POINTING             -> COMMAND_ATTENTION
  OPEN_PALM            -> COMMAND_STOP
  PEACE                -> COMMAND_STAY
  OK_SIGN + MOVING_TOWARD -> AUTHENTICATE_WITH_PROXIMITY
```

### 4. Code Generator (symbolic_interpreter.py: `GestureCodeGenerator`)

**Transforms:** CommandNode → Gesture Specification

Inverse of P(ext): generates expected gesture from intent.

```python
Intent: "AUTHENTICATE_BIOMETRIC"
↓
Spec: {
  "hand_config": "OK_SIGN",
  "motion": "STATIC",
  "duration": 1.0s
}
↓
Data: GestureData(landmarks=template, motion=(0, 0))
```

---

## Probe Integration (probe_symbolic.py)

### SymbolicProbe Class

Wraps the full pipeline as a bidirectional probe:

```python
probe = SymbolicProbe()

# P(ext): D -> S
state, channel, confidence = probe.p_ext(gesture_data)

# P(int): S -> D
reified_data = probe.p_int(state, context=gesture_data)

# F = P(int) ∘ P(ext)
reified_data, distance = probe.compose(gesture_data)
```

**Output Channels Based on Confidence:**

```python
if confidence >= 0.9:
    channel = CH_OBSERVE    # ✓ Green border: authenticated immediately
elif confidence >= 0.7:
    channel = CH_DEFER      # ⚠ Yellow border: wait for confirmation
else:
    channel = CH_COLLAPSE   # ✗ Red border: force to known state
```

---

## Fixed-Point ESG Construction (probe_symbolic.py)

### ESG = μX.F(X)

The **Eidetic Semantic Graph** is the stable structure where iterating F converges:

```
F(D) = P(int)(P(ext)(D)) ≈ D   (fixed point reached)
```

### Iteration Process

```
Iteration 1:
  D₀ (rocky gesture)
  ↓ P(ext)
  S (AUTHENTICATE_BIOMETRIC, conf=0.92)
  ↓ P(int)
  D₁ (reified gesture)
  Distance: ε₁ = 0.034

Iteration 2:
  D₁
  ↓ P(ext)
  S' (AUTHENTICATE_BIOMETRIC, conf=0.93)
  ↓ P(int)
  D₂ (reified gesture)
  Distance: ε₂ = 0.008  ← CONVERGED (< 0.01 threshold)
```

### ESG Vertices & Edges

**Vertices:**
- D: Raw data (gesture input)
- S: Semantic state (intent)
- D': Reified data (reconstructed gesture)

**Edges:**
- ext: D → S (external binding via lexer-parser-interpreter)
- int: S → D' (internal binding via code generator)

**Weights:**
- w = 1.0 - ε (confidence = inverse of distance)

---

## Usage Examples

### Basic Execution: P(ext)

```python
from symbolic_interpreter import SymbolicGestureInterpreter
from probe_symbolic import GestureData

interpreter = SymbolicGestureInterpreter()

# Gesture data
data = GestureData(
    landmarks=[...],  # 21 MediaPipe landmarks
    motion=(-0.71, 0.34),  # toward, ortho
    timestamp=1234567890.0,
    sensor_confidence=0.92
)

# Execute P(ext): D -> S
command = interpreter.execute(
    landmarks=data.landmarks,
    motion=data.motion,
    timestamp=data.timestamp,
    confidence=data.sensor_confidence
)

print(f"Intent: {command.intent}")  # AUTHENTICATE_BIOMETRIC
print(f"Confidence: {command.confidence:.2f}")  # 0.92
```

### Full Probe Cycle

```python
from probe_symbolic import SymbolicProbe, GestureData

probe = SymbolicProbe()

# P(ext): D -> S
state, channel, conf = probe.p_ext(data)
print(f"Channel: {channel.name}")  # CH_OBSERVE, CH_DEFER, or CH_COLLAPSE

# P(int): S -> D
reified = probe.p_int(state)

# Semantic distance
distance = probe._semantic_distance(data, reified)
print(f"Fixed point distance: {distance:.6f}")
```

### ESG Construction

```python
from probe_symbolic import SymbolicProbe, SymbolicESG, GestureData

probe = SymbolicProbe()
esg = SymbolicESG(probe)

# Build ESG via fixed-point iteration
result = esg.construct(initial_gesture_data)

print(f"Fixed point reached: {result['fixed_point_reached']}")
print(f"Final distance: {result['final_distance']:.6f}")
print(f"Vertices: {result['vertices']}")
print(f"Edges: {result['edges']}")
```

---

## Integration with MMUKO & Pet Classifier

### Gesture → Pet Recognition Pipeline

```
Camera Input (MMUKO Fluid System)
  ↓
Hand Gesture + Motion Data
  ↓ [Symbolic Interpreter P(ext)]
Command Intent (AUTHENTICATE, COMMAND_SIT, etc.)
  ↓
Route to Handler:
  - AUTHENTICATE_BIOMETRIC → Rocky/Riley classifier
  - COMMAND_SIT → Send signal to dog
  - CONFIRM_ACTION → Proceed with queued operation
```

### Constitutional Governance

```
If command.confidence >= 0.9:
    CH_0: Immediate execution (red border = locked on target)

If 0.7 <= confidence < 0.9:
    CH_1: Defer 60s for confirmation (yellow border = uncertain)
    Send gesture confirmation prompt to user

If confidence < 0.7:
    CH_2: Collapse to default (red border = error)
    Lock system / require retry
```

---

## Formal Specification

### Lexer Type Signature

```
lexer: (landmarks, motion, timestamp, conf) → Token[]
```

### Parser Type Signature

```
parser: Token[] → ASTNode
```

### Interpreter Type Signature

```
interpreter: ASTNode → CommandNode
```

### Probe Adjunction

```
P(ext): D → S  (data-bound)
P(int): S → D  (state-bound)

Adjoint Laws:
  P(ext) ∘ P(int) ∘ P(ext) = P(ext)  (idempotent)
  P(int) ∘ P(ext) ∘ P(int) = P(int)  (idempotent)
```

### Fixed Point

```
ESG = μX.F(X) where F(X) = P(int)(P(ext)(X))

Convergence: ‖F^(n+1)(D) - F^n(D)‖ < δ (threshold)
```

---

## File Structure

```
uche/
├── symbolic_interpreter.py      # Lexer, Parser, AST, Interpreter, CodeGenerator
├── probe_symbolic.py            # Probe wrapper, ESG construction
├── SYMBOLIC_INTERPRETER_README.md  # This file
└── integration_example.py       # [TODO] Full MMUKO + Pet Classifier integration
```

---

## Next Steps

### Phase 1: Validation ✓
- [x] Lexer implementation
- [x] Parser implementation
- [x] Interpreter implementation
- [x] Code generator implementation
- [x] Probe wrapper

### Phase 2: Integration (IN PROGRESS)
- [ ] Connect to MMUKO Fluid camera system
- [ ] Implement Rocky/Riley gesture training set
- [ ] ESG construction for pet-specific gestures
- [ ] CH_0/CH_1/CH_2 channel governance in live system

### Phase 3: Constitutional Compliance
- [ ] Audit trail for all gesture interpretations
- [ ] Bias parameter tracking (φ)
- [ ] Deferred judgment (CH_1) timeout handling
- [ ] Emergency lock (CH_2) protocols

### Phase 4: Deployment
- [ ] Integrate with RiftLang (rift.exe compilation)
- [ ] Build orchestration (nlink → polybuild)
- [ ] Continuous compliance verification

---

## References

- **Probe Theory:** Formal adjoint functor between Data and State domains
- **ESG Theory:** Fixed-point computation via Knaster-Tarski theorem
- **Constitutional Computing:** CH_0/CH_1/CH_2 trident governance
- **RiftLang:** Token architecture (memory, type, value, binding)
- **MMUKO:** Motion classification (RED = away, BLUE = ortho, GREEN = toward, ORANGE = static)

---

## Contact

**Project Lead:** Nnamdi Okpala (okpalan@protonmail.com)
**Framework:** OBINexus Constitutional Computing
**Status:** In Recovery (Momentum Phase)

