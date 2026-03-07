"""
Probe Wrapper: Symbolic Interpreter Integration with ESG
P(ext): D -> S (via Lexer-Parser-AST)
P(int): S -> D (via Code Generator)

Maps symbolic interpretation into fixed-point probe adjunction.
Date: 2026-03-07 | OBINexus Constitutional Computing
"""

from __future__ import annotations
from typing import TypeVar, Generic, Tuple, Optional, Dict, Any
from dataclasses import dataclass
from enum import Enum, auto
import numpy as np

from symbolic_interpreter import (
    SymbolicGestureInterpreter,
    GestureCodeGenerator,
    CommandNode,
    Token
)

# ============================================================================
# CORE PROBE TYPES
# ============================================================================

D = TypeVar('D')  # Data type
S = TypeVar('S')  # State type

class Binding(Enum):
    """Probe binding direction"""
    EXTERNAL = auto()  # P(ext): D -> S
    INTERNAL = auto()  # P(int): S -> D

class Channel(Enum):
    """Constitutional channels"""
    CH_OBSERVE = 0      # Immediate abstraction (confidence > 0.9)
    CH_DEFER = 1        # Deferred (0.7 < confidence < 0.9)
    CH_COLLAPSE = 2     # Force collapse (confidence < 0.7)

@dataclass
class GestureData:
    """Raw data D: hand landmarks + motion"""
    landmarks: list  # 21 MediaPipe landmarks
    motion: Tuple[float, float]  # (v_toward, v_ortho)
    timestamp: float
    sensor_confidence: float
    raw_tokens: Optional[list] = None  # Lexer output

@dataclass
class GestureState:
    """Semantic state S: command intent"""
    intent: str
    args: Dict[str, Any]
    tokens_used: Optional[list] = None
    timestamp: float = 0.0
    confidence: float = 0.0

@dataclass
class ProbeResult:
    """Result of probe application"""
    binding: Binding
    channel: Channel
    input_confidence: float
    output_confidence: float
    semantic_distance: float
    data_or_state: D | S

# ============================================================================
# SYMBOLIC PROBE
# ============================================================================

class SymbolicProbe(Generic[D, S]):
    """
    P: Symbolic gesture interpreter as bidirectional probe

    P(ext): D -> S (external binding)
      landmarks + motion -> lexer -> parser -> AST -> interpreter -> command state

    P(int): S -> D (internal binding)
      command state -> code generator -> gesture specification
    """

    def __init__(self):
        self.interpreter = SymbolicGestureInterpreter()
        self.generator = GestureCodeGenerator()
        self.probe_count = 0
        self.history = []

    def p_ext(
        self,
        data: GestureData
    ) -> Tuple[GestureState, Channel, float]:
        """
        P(ext): D -> S (external binding)
        Gesture data -> semantic command state

        Returns: (state, channel, confidence)
        """
        self.probe_count += 1

        # Execute symbolic interpretation pipeline
        command_node = self.interpreter.execute(
            landmarks=data.landmarks,
            motion=data.motion,
            timestamp=data.timestamp,
            confidence=data.sensor_confidence
        )

        # Map to GestureState
        state = GestureState(
            intent=command_node.intent,
            args=command_node.args,
            timestamp=command_node.timestamp,
            confidence=command_node.confidence
        )

        # Channel selection based on confidence (constitutional governance)
        if state.confidence >= 0.9:
            channel = Channel.CH_OBSERVE
        elif state.confidence >= 0.7:
            channel = Channel.CH_DEFER
        else:
            channel = Channel.CH_COLLAPSE

        # Record for ESG construction
        self.history.append({
            "type": "ext",
            "data": data,
            "state": state,
            "channel": channel
        })

        return state, channel, state.confidence

    def p_int(
        self,
        state: GestureState,
        context: Optional[GestureData] = None
    ) -> GestureData:
        """
        P(int): S -> D (internal binding)
        Semantic state -> gesture specification (reification)

        Returns reconstructed data (expected gesture)
        """

        # Create CommandNode for code generation
        command = CommandNode(
            intent=state.intent,
            args=state.args,
            timestamp=state.timestamp,
            confidence=state.confidence
        )

        # Generate gesture specification
        gesture_spec = self.generator.generate(command)

        # Reconstruct as GestureData
        # (In practice: hand pose template + expected motion)
        reified = GestureData(
            landmarks=self._generate_landmarks(gesture_spec),
            motion=self._generate_motion(gesture_spec),
            timestamp=state.timestamp,
            sensor_confidence=state.confidence
        )

        # Record for ESG
        self.history.append({
            "type": "int",
            "state": state,
            "data": reified,
            "gesture_spec": gesture_spec
        })

        return reified

    def compose(
        self,
        data: GestureData,
        context: Optional[GestureData] = None
    ) -> Tuple[GestureData, float]:
        """
        F = P(int) ∘ P(ext)
        Apply full probe composition: D -> S -> D'

        Returns: (reified_data, semantic_distance)
        """

        # Stage 1: P(ext) - abstract to state
        state, channel, ext_confidence = self.p_ext(data)

        # Stage 2: P(int) - reify to data
        reified = self.p_int(state, context or data)

        # Stage 3: measure semantic distance
        distance = self._semantic_distance(data, reified)

        return reified, distance

    def _semantic_distance(self, data1: GestureData, data2: GestureData) -> float:
        """
        Measure information loss: ‖D - F(D)‖
        Lower distance = better fixed point candidate
        """

        # Extract feature vectors
        vec1 = np.concatenate([
            np.array(data1.landmarks[:3]),  # first 3 landmarks
            np.array(data1.motion)
        ]).flatten()

        vec2 = np.concatenate([
            np.array(data2.landmarks[:3]),
            np.array(data2.motion)
        ]).flatten()

        # Euclidean distance
        distance = np.linalg.norm(vec1 - vec2)
        normalized = distance / (np.linalg.norm(vec1) + 1e-8)

        return float(normalized)

    def _generate_landmarks(self, gesture_spec: Dict[str, Any]) -> list:
        """Generate canonical landmarks from gesture specification"""
        # Template-based: retrieve or synthesize landmark template
        # For now: placeholder template for each hand config

        config = gesture_spec["hand_config"]
        template = {
            "OK_SIGN": [(0.5, 0.5)] * 21,
            "FIST": [(0.5, 0.5)] * 21,
            "OPEN_PALM": [(0.5, 0.5)] * 21,
            "THUMBS_UP": [(0.5, 0.5)] * 21,
            "POINTING": [(0.5, 0.5)] * 21,
        }

        return template.get(config, [(0.5, 0.5)] * 21)

    def _generate_motion(self, gesture_spec: Dict[str, Any]) -> Tuple[float, float]:
        """Generate canonical motion from gesture specification"""
        motion_type = gesture_spec["motion"]

        motion_map = {
            "STATIC": (0.0, 0.0),
            "MOVING_TOWARD": (-0.7, 0.0),
            "MOVING_AWAY": (0.7, 0.0),
            "MOVING_ORTHO": (0.0, 0.5),
        }

        return motion_map.get(motion_type, (0.0, 0.0))

    def get_history(self) -> list:
        """Retrieve full probe execution history (for ESG construction)"""
        return self.history

# ============================================================================
# ESG INTEGRATION: Fixed-Point Construction
# ============================================================================

@dataclass
class ESGVertex:
    """Node in Eidetic Semantic Graph"""
    id: int
    data_or_state: D | S
    vertex_type: str  # "D", "S", "D'"
    confidence: float

@dataclass
class ESGEdge:
    """Edge representing probe transformation"""
    source: ESGVertex
    target: ESGVertex
    probe_type: Binding  # EXTERNAL or INTERNAL
    channel: Channel
    weight: float
    semantic_distance: float

class SymbolicESG:
    """
    ESG construction via symbolic probe fixed-point iteration
    ESG = μX.F(X) where F = P(int) ∘ P(ext)
    """

    def __init__(self, probe: SymbolicProbe):
        self.probe = probe
        self.vertices = []
        self.edges = []
        self.convergence_threshold = 0.01
        self.max_iterations = 10
        self.fixed_point_reached = False
        self.final_distance = float('inf')

    def construct(self, initial_data: GestureData) -> Dict[str, Any]:
        """
        Build ESG from initial gesture data
        Iterate F = P(int) ∘ P(ext) until convergence
        """

        # Vertex 0: initial data (D₀)
        v0 = ESGVertex(id=0, data_or_state=initial_data, vertex_type="D", confidence=initial_data.sensor_confidence)
        self.vertices.append(v0)

        current_data = initial_data
        current_vertex = v0
        iteration = 0

        print(f"\n[ESG] Construction starting...")
        print(f"[ESG] Initial data confidence: {initial_data.sensor_confidence:.3f}")

        while iteration < self.max_iterations:
            iteration += 1
            print(f"\n[ESG] Iteration {iteration}")

            # Apply F = P(int) ∘ P(ext)
            reified_data, distance = self.probe.compose(current_data)

            print(f"[ESG]   Semantic distance: {distance:.6f}")
            print(f"[ESG]   Threshold: {self.convergence_threshold:.6f}")

            # Create intermediate vertices and edges

            # P(ext): D -> S
            state, channel, confidence = self.probe.p_ext(current_data)
            v_state = ESGVertex(
                id=len(self.vertices),
                data_or_state=state,
                vertex_type="S",
                confidence=confidence
            )
            self.vertices.append(v_state)

            edge_ext = ESGEdge(
                source=current_vertex,
                target=v_state,
                probe_type=Binding.EXTERNAL,
                channel=channel,
                weight=1.0 - distance,
                semantic_distance=distance
            )
            self.edges.append(edge_ext)
            print(f"[ESG]   -> P(ext): {current_vertex.vertex_type}#{current_vertex.id} -> S#{v_state.id}")

            # P(int): S -> D'
            reified = self.probe.p_int(state, current_data)
            v_reified = ESGVertex(
                id=len(self.vertices),
                data_or_state=reified,
                vertex_type="D'",
                confidence=confidence
            )
            self.vertices.append(v_reified)

            edge_int = ESGEdge(
                source=v_state,
                target=v_reified,
                probe_type=Binding.INTERNAL,
                channel=Channel.CH_OBSERVE,
                weight=1.0,
                semantic_distance=0.0
            )
            self.edges.append(edge_int)
            print(f"[ESG]   -> P(int): S#{v_state.id} -> D'#{v_reified.id}")

            # Check convergence: is reified ≈ current?
            if distance < self.convergence_threshold:
                self.fixed_point_reached = True
                self.final_distance = distance
                print(f"[ESG] ✓ FIXED POINT REACHED")
                print(f"[ESG] ✓ Final semantic distance: {distance:.6f}")
                break

            # Continue iteration
            current_data = reified_data
            current_vertex = v_reified

        else:
            print(f"[ESG] ! Max iterations ({self.max_iterations}) reached")

        return self.summary()

    def summary(self) -> Dict[str, Any]:
        """Return ESG summary"""
        return {
            "vertices": len(self.vertices),
            "edges": len(self.edges),
            "fixed_point_reached": self.fixed_point_reached,
            "final_distance": self.final_distance,
            "vertex_graph": [
                {
                    "id": v.id,
                    "type": v.vertex_type,
                    "confidence": v.confidence
                }
                for v in self.vertices
            ],
            "edge_graph": [
                {
                    "source": e.source.id,
                    "target": e.target.id,
                    "probe": e.probe_type.name,
                    "channel": e.channel.name,
                    "distance": e.semantic_distance
                }
                for e in self.edges
            ]
        }

# ============================================================================
# USAGE
# ============================================================================

if __name__ == "__main__":
    # Create probe
    probe = SymbolicProbe()

    # Simulate rocky's gesture
    rocky_data = GestureData(
        landmarks=[(0.5, 0.5)] * 21,
        motion=(-0.71, 0.34),  # moving toward, ortho
        timestamp=1234567890.0,
        sensor_confidence=0.92
    )

    print("=" * 70)
    print("P(ext): Data -> State")
    print("=" * 70)

    state, channel, conf = probe.p_ext(rocky_data)
    print(f"Intent: {state.intent}")
    print(f"Confidence: {conf:.3f}")
    print(f"Channel: {channel.name}")

    print("\n" + "=" * 70)
    print("P(int): State -> Data")
    print("=" * 70)

    reified = probe.p_int(state, rocky_data)
    print(f"Reified motion: {reified.motion}")
    print(f"Reified confidence: {reified.sensor_confidence:.3f}")

    print("\n" + "=" * 70)
    print("ESG: Fixed-Point Construction")
    print("=" * 70)

    esg = SymbolicESG(probe)
    esg_result = esg.construct(rocky_data)

    print("\n" + "=" * 70)
    print("ESG Summary")
    print("=" * 70)
    print(f"Vertices: {esg_result['vertices']}")
    print(f"Edges: {esg_result['edges']}")
    print(f"Fixed Point: {esg_result['fixed_point_reached']}")
    print(f"Final Distance: {esg_result['final_distance']:.6f}")
