# Symbolic Gesture Interpreter - Quick Reference
## One-Page Cheat Sheet

---

## Pipeline Overview

```
D (Data)
  ↓ LEXER
Token[]
  ↓ PARSER
AST
  ↓ INTERPRETER
S (State)
  ↓ CODE GEN
D' (Reified Data)
```

---

## Hand Gestures

| Gesture | Config | Meaning | Intent |
|---------|--------|---------|--------|
| 👌 | OK_SIGN | Thumb + index touching | AUTHENTICATE_BIOMETRIC |
| ✊ | FIST | All fingers closed | CANCEL_OPERATION |
| ✋ | OPEN_PALM | All fingers extended | COMMAND_STOP |
| ☝️ | POINTING | Index finger only | COMMAND_ATTENTION |
| ✌️ | PEACE | Index + middle | COMMAND_STAY |
| 👍 | THUMBS_UP | Thumb vertical | CONFIRM_ACTION |

---

## Motion Tokens

| Motion | v_toward | v_ortho | State |
|--------|----------|---------|-------|
| MOVING_TOWARD | < -0.5 | any | GREEN |
| MOVING_AWAY | > 0.5 | any | RED |
| MOVING_ORTHO | -0.5 to 0.5 | > 0.3 | BLUE |
| STATIC | -0.1 to 0.1 | -0.1 to 0.1 | ORANGE |

---

## Constitutional Channels

| Channel | Confidence | Color | Action |
|---------|-----------|-------|--------|
| CH_0 | ≥ 0.9 | 🟢 GREEN | Execute immediately |
| CH_1 | 0.7-0.9 | 🟡 YELLOW | Defer 60s for confirmation |
| CH_2 | < 0.7 | 🔴 RED | Force reset / lock |

---

## Code Classes

### symbolic_interpreter.py

```python
# Lexer: Data → Tokens
lexer = GestureLexer(motion_threshold=0.5)
tokens = lexer.tokenize(landmarks, motion, timestamp, confidence)

# Parser: Tokens → AST
parser = GestureParser()
ast = parser.parse(tokens)

# Interpreter: AST → Intent
interpreter = GestureInterpreter()
command = interpreter.evaluate(ast)

# Code Generator: Intent → Data
generator = GestureCodeGenerator()
gesture_spec = generator.generate(command)

# Full Pipeline
interpreter = SymbolicGestureInterpreter()
command = interpreter.execute(landmarks, motion, timestamp, confidence)
```

### probe_symbolic.py

```python
# Probe: Bidirectional binding
probe = SymbolicProbe()

# P(ext): D → S (external binding)
state, channel, confidence = probe.p_ext(gesture_data)

# P(int): S → D (internal binding)
reified_data = probe.p_int(state, context)

# Composition: F = P(int) ∘ P(ext)
reified, distance = probe.compose(data)

# ESG: Fixed-point construction
esg = SymbolicESG(probe)
result = esg.construct(initial_data)
# → result['fixed_point_reached']
# → result['final_distance']
```

### integration_example.py

```python
# Full system
system = ConstitutionalGestureSystem()

# Process frame
result = system.process_frame(frame, motion, target_pet="Rocky")
# → result['gesture']
# → result['channel']
# → result['command']

# Execute
system.execute_command(result)
```

---

## Data Structures

### GestureData (D)
```python
@dataclass
class GestureData:
    landmarks: list        # 21 (x,y,z) tuples
    motion: (float, float) # (v_toward, v_ortho)
    timestamp: float       # epoch time
    sensor_confidence: float  # 0.0-1.0
```

### GestureState (S)
```python
@dataclass
class GestureState:
    intent: str            # "AUTHENTICATE_BIOMETRIC", etc.
    args: Dict[str, Any]   # gesture metadata
    confidence: float      # 0.0-1.0
    timestamp: float       # epoch time
```

### CommandNode
```python
@dataclass
class CommandNode(ASTNode):
    intent: str            # semantic action
    args: Dict[str, Any]   # parameters
    confidence: float      # 0.0-1.0
    timestamp: float       # epoch time
```

---

## Type Signatures

### Lexer
```
lexer: (landmarks, motion, timestamp, conf) → Token[]
```

### Parser
```
parser: Token[] → ASTNode
```

### Interpreter
```
interpreter: ASTNode → CommandNode
```

### Code Generator
```
generator: CommandNode → Dict[str, Any]
```

### Probe (P)
```
p_ext: D → (S, Channel, float)
p_int: S → D
F: D → D where F = p_int ∘ p_ext
```

### ESG
```
construct: D → {vertices, edges, fixed_point_reached, final_distance}
```

---

## Usage Patterns

### Pattern 1: Single Gesture Classification
```python
interpreter = SymbolicGestureInterpreter()
command = interpreter.execute(landmarks, motion, timestamp, confidence)
print(f"Intent: {command.intent}, Confidence: {command.confidence}")
```

### Pattern 2: Channel-Based Routing
```python
probe = SymbolicProbe()
state, channel, conf = probe.p_ext(data)

if channel == Channel.CH_0:
    execute_immediately(state.intent)
elif channel == Channel.CH_1:
    defer_60s(state.intent)
else:  # CH_2
    force_reset()
```

### Pattern 3: ESG Construction (Learning)
```python
esg = SymbolicESG(probe)
result = esg.construct(gesture_data)

if result['fixed_point_reached']:
    save_gesture_model(gesture_data, result)
```

### Pattern 4: Pet Command Routing
```python
system = ConstitutionalGestureSystem()
result = system.process_frame(frame, motion, target_pet="Rocky")

pet = PETS[result['target_pet']]
pet_cmd = pet.gesture_commands[result['gesture']]
send_command(pet, pet_cmd)
```

---

## Common Issues & Fixes

| Issue | Cause | Fix |
|-------|-------|-----|
| No gesture detected | Landmarks too low confidence | Check `sensor_confidence >= 0.7` |
| Wrong intent | Intent mapping missing | Add to `GestureInterpreter.intent_patterns` |
| CH_2 collapse | Confidence < 0.7 | Improve hand pose tracking |
| ESG not converging | Gesture too noisy | Increase `convergence_threshold` |
| No motion detected | MMUKO not tracking | Check optical flow calculation |

---

## Configuration Parameters

```python
# GestureLexer
motion_threshold = 0.5        # Velocity threshold for motion detection
confidence_threshold = 0.7    # MediaPipe confidence minimum

# SymbolicESG
convergence_threshold = 0.01  # Fixed-point distance threshold
max_iterations = 10           # Maximum iterations before timeout

# Channel Selection (Automatic)
CH_0 if confidence >= 0.9
CH_1 if 0.7 <= confidence < 0.9
CH_2 if confidence < 0.7
```

---

## Files

| File | Lines | Purpose |
|------|-------|---------|
| symbolic_interpreter.py | 1200 | Lexer, Parser, AST, Interpreter, CodeGen |
| probe_symbolic.py | 500 | Probe wrapper, ESG construction |
| integration_example.py | 400 | Live demo with MMUKO + pets |
| SYMBOLIC_INTERPRETER_README.md | 400 | Full technical documentation |
| IMPLEMENTATION_SUMMARY.md | 300 | This session's work summary |
| QUICK_REFERENCE.md | 200 | This quick reference |

---

## Phase Progress

### Phase 1: Core Implementation ✓
- [x] Lexer
- [x] Parser
- [x] Interpreter
- [x] Code Generator
- [x] Probe Framework
- [x] ESG Construction

### Phase 2: Integration (TODO)
- [ ] Connect MMUKO motion tracking
- [ ] Load Rocky/Riley gesture sets
- [ ] Live camera processing
- [ ] Channel governance in production
- [ ] Command execution to pets
- [ ] Gesture learning (ESG)

### Phase 3: Constitutional Compliance (TODO)
- [ ] Audit trail logging
- [ ] Bias parameter tracking
- [ ] Deferment timeout handling
- [ ] Emergency protocols

---

## Remember

1. **P(ext) and P(int) are dual, not inverse** → information can be lost
2. **ESG is fixed-point** → gestures "stabilize" after iteration
3. **CH_0/CH_1/CH_2 = governance** → channels are constitutional
4. **Confidence is everything** → drives channel selection
5. **Session continuity matters** → don't reset OBINexus context

---

**Last Updated:** 2026-03-07
**Status:** Phase 1 Complete | Phase 2 Ready
**Next:** Integration with live MMUKO + pet system

