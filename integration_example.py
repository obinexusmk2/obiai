"""
Integration Example: Symbolic Gesture Interpreter + MMUKO + Pet Classifier
Shows full pipeline: camera → gesture → command → action

Date: 2026-03-07 | OBINexus Constitutional Computing
"""

from typing import Optional, Dict, Any, Tuple
from dataclasses import dataclass
from enum import Enum, auto
import cv2
import numpy as np
import mediapipe as mp

from symbolic_interpreter import SymbolicGestureInterpreter
from probe_symbolic import SymbolicProbe, GestureData, SymbolicESG

# ============================================================================
# PET CLASSIFIER (Rocky/Riley)
# ============================================================================

@dataclass
class PetProfile:
    """Pet profile for classifier"""
    name: str
    breed: str
    phenotypes: list  # Distinguishing features
    gesture_commands: Dict[str, str]  # gesture_intent -> pet_command

# Rocky: Beagle with eye patch
ROCKY = PetProfile(
    name="Rocky",
    breed="beagle",
    phenotypes=["eye_patch", "floppy_ears", "brown_white"],
    gesture_commands={
        "COMMAND_ATTENTION": "look_at_camera",
        "COMMAND_STOP": "stay",
        "COMMAND_SIT": "sit",
        "CONFIRM_ACTION": "good_dog"
    }
)

# Riley: German Shepherd
RILEY = PetProfile(
    name="Riley",
    breed="german_shepherd",
    phenotypes=["pointed_ears", "dark_coat", "alert_posture"],
    gesture_commands={
        "COMMAND_ATTENTION": "attention",
        "COMMAND_STOP": "halt",
        "COMMAND_SIT": "sit",
        "CONFIRM_ACTION": "excellent"
    }
)

PETS = {rocky.name: ROCKY, "Riley": RILEY}

# ============================================================================
# MMUKO MOTION CLASSIFIER
# ============================================================================

class MotionState(Enum):
    """MMUKO drift states"""
    RED = auto()      # Moving away
    BLUE = auto()     # Orthogonal motion
    GREEN = auto()    # Moving toward
    ORANGE = auto()   # Static

class MMUKOMotionClassifier:
    """Classify motion from optical flow"""

    def __init__(self, threshold: float = 0.5):
        self.threshold = threshold

    def classify(self, v_toward: float, v_ortho: float) -> MotionState:
        """Classify motion into MMUKO state"""

        if abs(v_toward) < 0.1 and abs(v_ortho) < 0.1:
            return MotionState.ORANGE  # Static

        if v_toward < -self.threshold:
            return MotionState.GREEN   # Toward

        elif v_toward > self.threshold:
            return MotionState.RED     # Away

        elif abs(v_ortho) > 0.3:
            return MotionState.BLUE    # Orthogonal

        return MotionState.ORANGE

# ============================================================================
# INTEGRATED SYSTEM: Gesture + Motion + Command
# ============================================================================

class ConstitutionalGestureSystem:
    """
    Full system integrating:
    1. MMUKO motion tracking
    2. Symbolic gesture interpretation (P(ext): D -> S)
    3. Pet classifier
    4. Command execution with CH_0/CH_1/CH_2 governance
    """

    def __init__(self):
        # Initialize symbolic interpreter
        self.symbolic_interpreter = SymbolicGestureInterpreter()
        self.probe = SymbolicProbe()

        # Initialize motion classifier
        self.motion_classifier = MMUKOMotionClassifier(threshold=0.5)

        # Initialize MediaPipe hand detection
        self.mp_hands = mp.solutions.hands
        self.hands = self.mp_hands.Hands(
            static_image_mode=False,
            max_num_hands=1,
            min_detection_confidence=0.7,
            min_tracking_confidence=0.5
        )
        self.mp_drawing = mp.solutions.drawing_utils

        # Command queue
        self.command_queue = []
        self.last_command = None
        self.last_pet = None

        # ESG for learning
        self.esg_factory = {}  # pet_name -> ESG

    def process_frame(
        self,
        frame: np.ndarray,
        motion: Tuple[float, float],
        target_pet: Optional[str] = None
    ) -> Dict[str, Any]:
        """
        Process single frame with full pipeline

        Args:
            frame: Input image (BGR)
            motion: (v_toward, v_ortho) from MMUKO
            target_pet: Which pet to target (None = auto-detect)

        Returns:
            {
                "motion_state": MotionState,
                "gesture": str (intent),
                "gesture_confidence": float,
                "channel": str (CH_0/CH_1/CH_2),
                "command": str (pet command),
                "target_pet": str,
                "action": str (what to do)
            }
        """

        result = {
            "motion_state": None,
            "gesture": None,
            "gesture_confidence": 0.0,
            "channel": None,
            "command": None,
            "target_pet": target_pet or "unknown",
            "action": "idle"
        }

        # STAGE 1: Motion classification (MMUKO)
        motion_state = self.motion_classifier.classify(motion[0], motion[1])
        result["motion_state"] = motion_state.name

        # STAGE 2: Hand detection
        rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
        hand_results = self.hands.process(rgb)

        if not hand_results.multi_hand_landmarks:
            result["action"] = "no_hand_detected"
            return result

        # Get landmarks
        landmarks = hand_results.multi_hand_landmarks[0].landmark
        landmarks_list = [(lm.x, lm.y, lm.z) for lm in landmarks]

        # STAGE 3: Symbolic interpretation (P(ext): D -> S)
        gesture_data = GestureData(
            landmarks=landmarks_list,
            motion=motion,
            timestamp=cv2.getTickCount() / cv2.getTickFrequency(),
            sensor_confidence=0.85  # Simulate mediapipe confidence
        )

        state, channel, gesture_confidence = self.probe.p_ext(gesture_data)
        result["gesture"] = state.intent
        result["gesture_confidence"] = gesture_confidence
        result["channel"] = channel.name

        # STAGE 4: Constitutional channel governance
        if channel.name == "CH_0":
            # Immediate: gesture is high confidence
            result["action"] = "execute_immediately"

        elif channel.name == "CH_1":
            # Defer: gesture is uncertain, queue for verification
            result["action"] = "defer_60s"

        elif channel.name == "CH_2":
            # Collapse: gesture is unreliable, reset
            result["action"] = "force_reset"
            return result

        # STAGE 5: Route to pet command
        if target_pet and target_pet in PETS:
            pet = PETS[target_pet]
            pet_command = pet.gesture_commands.get(state.intent, "unknown")
            result["command"] = pet_command
            result["target_pet"] = target_pet
            result["action"] = f"command_{pet_command}"

        return result

    def execute_command(self, result: Dict[str, Any]) -> bool:
        """
        Execute command based on result

        Returns: True if executed, False if deferred/failed
        """

        action = result["action"]

        if action == "idle" or action == "no_hand_detected":
            return False

        if action.startswith("command_"):
            pet_command = action.split("_", 1)[1]
            pet_name = result["target_pet"]
            print(f"✓ [{result['channel']}] Sending '{pet_command}' to {pet_name}")
            self.last_command = {
                "pet": pet_name,
                "command": pet_command,
                "gesture": result["gesture"],
                "confidence": result["gesture_confidence"]
            }
            return True

        elif action == "defer_60s":
            print(f"⚠ [{result['channel']}] Deferring gesture (waiting 60s for confirmation)")
            self.command_queue.append(result)
            return False

        elif action == "force_reset":
            print(f"✗ [{result['channel']}] Force reset (gesture too uncertain)")
            self.command_queue = []
            return False

        return False

    def learn_gesture(self, pet_name: str, gesture_sequence: list) -> Dict[str, Any]:
        """
        Train ESG on pet-specific gesture sequence
        """

        if pet_name not in PETS:
            return {"status": "error", "reason": "unknown_pet"}

        # Build ESG for this gesture sequence
        esg = SymbolicESG(self.probe)
        result = esg.construct(gesture_sequence[0])

        self.esg_factory[pet_name] = esg

        return {
            "status": "learned",
            "pet": pet_name,
            "esg_vertices": result["vertices"],
            "esg_edges": result["edges"],
            "fixed_point": result["fixed_point_reached"]
        }

# ============================================================================
# LIVE DEMO
# ============================================================================

def main():
    """Live demo: process camera input with constitutional gesture system"""

    system = ConstitutionalGestureSystem()

    cap = cv2.VideoCapture(0)

    print("\n" + "=" * 70)
    print("Constitutional Gesture System - Live Demo")
    print("=" * 70)
    print("Gestures:")
    print("  OK_SIGN: Authenticate")
    print("  FIST: Cancel")
    print("  OPEN_PALM: Stop")
    print("  POINTING: Attention")
    print("  PEACE: Stay")
    print("  THUMBS_UP: Confirm")
    print("\nPress 'q' to quit")
    print("=" * 70 + "\n")

    frame_count = 0
    gesture_history = []

    while True:
        ret, frame = cap.read()
        if not ret:
            break

        frame_count += 1

        # Simulate MMUKO motion vectors (in real: from optical flow)
        # Vary motion based on frame count for demo
        if frame_count % 100 < 25:
            motion = (-0.7, 0.2)  # Moving toward (GREEN)
        elif frame_count % 100 < 50:
            motion = (0.7, 0.1)   # Moving away (RED)
        elif frame_count % 100 < 75:
            motion = (0.1, 0.5)   # Orthogonal (BLUE)
        else:
            motion = (0.0, 0.0)   # Static (ORANGE)

        # Process frame with constitutional system
        result = system.process_frame(frame, motion, target_pet="Rocky")

        # Execute command if appropriate
        system.execute_command(result)

        gesture_history.append(result)

        # Draw HUD
        frame_display = frame.copy()
        h, w = frame_display.shape[:2]

        # Channel color coding
        channel_colors = {
            "CH_0": (0, 255, 0),       # Green
            "CH_1": (0, 255, 255),     # Yellow
            "CH_2": (0, 0, 255),       # Red
            "None": (128, 128, 128)    # Gray
        }
        color = channel_colors.get(result["channel"], (128, 128, 128))

        # Draw border
        cv2.rectangle(frame_display, (0, 0), (w, h), color, 20)

        # Draw info
        cv2.putText(frame_display, f"Motion: {result['motion_state']}", (30, 50),
                   cv2.FONT_HERSHEY_SIMPLEX, 0.8, (255, 255, 255), 2)
        cv2.putText(frame_display, f"Gesture: {result['gesture']}", (30, 100),
                   cv2.FONT_HERSHEY_SIMPLEX, 0.8, (255, 255, 255), 2)
        cv2.putText(frame_display, f"Confidence: {result['gesture_confidence']:.2f}", (30, 150),
                   cv2.FONT_HERSHEY_SIMPLEX, 0.7, (255, 255, 255), 2)
        cv2.putText(frame_display, f"Channel: {result['channel']}", (30, 200),
                   cv2.FONT_HERSHEY_SIMPLEX, 0.8, color, 2)
        cv2.putText(frame_display, f"Command: {result['command']}", (30, 250),
                   cv2.FONT_HERSHEY_SIMPLEX, 0.8, (255, 255, 255), 2)

        cv2.imshow("Constitutional Gesture System", frame_display)

        if cv2.waitKey(1) & 0xFF == ord('q'):
            break

    cap.release()
    cv2.destroyAllWindows()

    # Print summary
    print("\n" + "=" * 70)
    print("Session Summary")
    print("=" * 70)
    print(f"Frames processed: {frame_count}")
    print(f"Last command: {system.last_command}")
    print(f"Deferred commands: {len(system.command_queue)}")

if __name__ == "__main__":
    main()
