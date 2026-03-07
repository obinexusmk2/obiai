# Symbolic Gesture Interpreter - Setup & Run Guide

## Installation

### Step 1: Install Python Dependencies

```bash
pip install numpy opencv-python mediapipe
```

### Step 2: Verify Installation

```bash
python -m pip list | grep -E "numpy|opencv|mediapipe"
```

### Step 3: Verify Core Modules Work

```bash
# Test symbolic_interpreter.py (no external dependencies)
python symbolic_interpreter.py

# Test probe_symbolic.py (numpy only)
python probe_symbolic.py

# Test integration_example.py (requires opencv + mediapipe)
python integration_example.py
```

---

## Quick Test Without Live Camera

If you don't have a camera or want to test the logic first:

```bash
# This runs the full pipeline without camera input
python -c "
from probe_symbolic import SymbolicProbe, SymbolicESG, GestureData

probe = SymbolicProbe()

# Simulated gesture data
data = GestureData(
    landmarks=[(0.5, 0.5)] * 21,
    motion=(-0.71, 0.34),
    timestamp=0.0,
    sensor_confidence=0.92
)

# P(ext): Data -> State
state, channel, conf = probe.p_ext(data)
print(f'Intent: {state.intent}')
print(f'Confidence: {conf}')
print(f'Channel: {channel.name}')

# ESG construction
esg = SymbolicESG(probe)
result = esg.construct(data)
print(f'Fixed Point Reached: {result[\"fixed_point_reached\"]}')
"
```

---

## Files & Their Purpose

| File | Requires | Purpose |
|------|----------|---------|
| `symbolic_interpreter.py` | numpy | Lexer → Parser → AST → Interpreter → CodeGen |
| `probe_symbolic.py` | numpy | Probe wrapper + ESG fixed-point |
| `integration_example.py` | opencv, mediapipe | Live demo with hand tracking |
| `QUICK_REFERENCE.md` | — | One-page cheat sheet |
| `SYMBOLIC_INTERPRETER_README.md` | — | Technical docs |

---

## Running the System

### Mode 1: Pure Symbolic (Test Logic)

```bash
python symbolic_interpreter.py
```

**Output:** Shows P(ext) and P(int) execution on sample data
**Time:** <1 second
**Requirements:** numpy only

### Mode 2: Probe + ESG (Test Fixed-Point)

```bash
python probe_symbolic.py
```

**Output:** Shows ESG construction with convergence
**Time:** ~2 seconds
**Requirements:** numpy only

### Mode 3: Live Demo (Full System)

```bash
python integration_example.py
```

**Output:** Live camera window with gesture recognition
**Time:** Runs until 'q' pressed
**Requirements:** OpenCV, MediaPipe, camera

---

## Troubleshooting

### Error: `ModuleNotFoundError: No module named 'mediapipe'`

**Solution:**
```bash
pip install mediapipe
```

Or if you don't need live demo:
```bash
# Run symbolic_interpreter.py and probe_symbolic.py instead
# They don't require mediapipe
```

### Error: `No module named 'cv2'` (OpenCV)

**Solution:**
```bash
pip install opencv-python
```

### Error: `No module named 'numpy'`

**Solution:**
```bash
pip install numpy
```

### Camera not detected in integration_example.py

**Solution:**
1. Check if camera is connected: `ls /dev/video*` (Linux) or Camera Settings (Windows)
2. Verify no other app is using camera
3. Run in privileged mode if needed

---

## Expected Output Examples

### symbolic_interpreter.py

```
P(ext) Result:
  Intent: SEQUENCE:
  Args: {'type': 'sequence', 'nodes': [...], 'duration': 0.0, 'confidence': 0.92}
  Confidence: 0.92

P(int) Result:
  Gesture Spec: {'intent': 'SEQUENCE:', 'hand_config': 'OPEN_PALM', ...}
```

### probe_symbolic.py

```
======================================================================
ESG: Fixed-Point Construction
======================================================================

[ESG] Construction starting...
[ESG] Initial data confidence: 0.920

[ESG] Iteration 1
[ESG]   Semantic distance: 0.540696
[ESG]   Threshold: 0.010000

[ESG] Iteration 2
[ESG]   Semantic distance: 0.000000
[ESG] ✓ FIXED POINT REACHED
[ESG] ✓ Final semantic distance: 0.000000

ESG Summary
Vertices: 5
Edges: 4
Fixed Point: True
Final Distance: 0.000000
```

### integration_example.py

```
======================================================================
Constitutional Gesture System - Live Demo
======================================================================
Gestures:
  OK_SIGN: Authenticate
  FIST: Cancel
  OPEN_PALM: Stop
  POINTING: Attention
  PEACE: Stay
  THUMBS_UP: Confirm

[Live camera window opens with HUD showing:]
- Motion: RED/GREEN/BLUE/ORANGE
- Gesture: OK_SIGN/FIST/etc.
- Confidence: 0.92
- Channel: CH_0/CH_1/CH_2
- Command: sit/stay/etc.
```

---

## Configuration

### Lexer Thresholds

```python
# In symbolic_interpreter.py
lexer = GestureLexer(
    motion_threshold=0.5,        # Velocity threshold
    confidence_threshold=0.7      # MediaPipe confidence minimum
)
```

### Channel Governance

```python
# Automatic based on confidence:
confidence >= 0.9    → CH_0 (GREEN, immediate)
0.7 <= conf < 0.9   → CH_1 (YELLOW, defer 60s)
confidence < 0.7    → CH_2 (RED, reset)
```

### ESG Convergence

```python
# In probe_symbolic.py
esg = SymbolicESG(probe)
esg.convergence_threshold = 0.01     # Distance threshold
esg.max_iterations = 10               # Maximum iterations
```

---

## Next Steps After Installation

1. **Run Mode 1** (symbolic_interpreter.py) - verify logic works
2. **Run Mode 2** (probe_symbolic.py) - verify ESG converges
3. **Run Mode 3** (integration_example.py) - verify live system works
4. **Collect gestures** - create dataset for Rocky/Riley
5. **Train ESGs** - build gesture profiles
6. **Deploy** - integrate with MMUKO fluid system

---

## Help & Documentation

- **Quick Start:** See `QUICK_REFERENCE.md`
- **Full Docs:** See `SYMBOLIC_INTERPRETER_README.md`
- **Architecture:** See `IMPLEMENTATION_SUMMARY.md`
- **Code Comments:** Read docstrings in .py files

---

**Status:** All core files ready to run
**Next:** Run symbolic_interpreter.py to verify installation

