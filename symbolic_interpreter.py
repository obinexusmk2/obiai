"""
Symbolic Gesture Interpreter - OBINexus UCHE
P(ext): D -> S via Lexer -> Parser -> AST Interpreter
P(int): S -> D via Code Generator

Date: 2026-03-07 | Constitutional Computing
"""

from __future__ import annotations
from typing import List, Dict, Optional, Tuple, Union, Any
from dataclasses import dataclass, field
from enum import Enum, auto
import numpy as np
from abc import ABC, abstractmethod

# ============================================================================
# ABSTRACT SYNTAX TREE (AST) DEFINITIONS
# ============================================================================

class TokenType(Enum):
    """Lexer tokens for gesture language"""
    # Hand configuration tokens
    FIST = auto()          # Fingers closed
    OPEN_PALM = auto()     # All fingers extended
    POINTING = auto()      # Index finger extended
    PEACE = auto()         # Index + middle extended
    THUMBS_UP = auto()     # Thumb extended (vertical)
    OK_SIGN = auto()       # Thumb + index touching

    # Motion tokens
    MOVING_TOWARD = auto()     # v_toward < -0.5
    MOVING_AWAY = auto()       # v_toward > 0.5
    MOVING_ORTHO = auto()      # |v_ortho| > 0.3
    STATIC = auto()            # No motion

    # Modifiers
    SUSTAINED = auto()     # 4+ frames same gesture
    QUICK = auto()         # <2 frames

    # Logical operators
    AND = auto()
    OR = auto()
    NOT = auto()

    # Delimiters
    LPAREN = auto()
    RPAREN = auto()
    SEMICOLON = auto()

    EOF = auto()

@dataclass
class Token:
    """Lexeme: type + value"""
    type: TokenType
    value: Any
    timestamp: float
    confidence: float

@dataclass
class ASTNode(ABC):
    """Base AST node"""
    timestamp: float
    confidence: float

    @abstractmethod
    def evaluate(self, context: Dict[str, Any]) -> Any:
        pass

@dataclass
class HandConfigNode(ASTNode):
    """Leaf: hand configuration"""
    config: TokenType  # FIST, OPEN_PALM, etc.

    def evaluate(self, context: Dict[str, Any]) -> Dict[str, Any]:
        return {
            "type": "hand_config",
            "config": self.config.name,
            "confidence": self.confidence,
            "timestamp": self.timestamp
        }

@dataclass
class MotionNode(ASTNode):
    """Leaf: motion vector"""
    motion: TokenType  # MOVING_TOWARD, MOVING_AWAY, MOVING_ORTHO, STATIC
    velocity: Tuple[float, float]  # (v_toward, v_ortho)

    def evaluate(self, context: Dict[str, Any]) -> Dict[str, Any]:
        return {
            "type": "motion",
            "motion": self.motion.name,
            "velocity": self.velocity,
            "confidence": self.confidence
        }

@dataclass
class SequenceNode(ASTNode):
    """Compound: temporal sequence of gestures"""
    nodes: List[ASTNode]
    duration: float

    def evaluate(self, context: Dict[str, Any]) -> Dict[str, Any]:
        return {
            "type": "sequence",
            "nodes": [n.evaluate(context) for n in self.nodes],
            "duration": self.duration,
            "confidence": min(n.confidence for n in self.nodes)
        }

@dataclass
class LogicalNode(ASTNode):
    """Compound: AND/OR/NOT combinations"""
    operator: TokenType  # AND, OR, NOT
    operands: List[ASTNode]

    def evaluate(self, context: Dict[str, Any]) -> Dict[str, Any]:
        if self.operator == TokenType.AND:
            result = all(n.evaluate(context) for n in self.operands)
        elif self.operator == TokenType.OR:
            result = any(n.evaluate(context) for n in self.operands)
        elif self.operator == TokenType.NOT:
            result = not self.operands[0].evaluate(context)
        else:
            result = False

        return {
            "type": "logical",
            "operator": self.operator.name,
            "result": result,
            "confidence": self.confidence
        }

@dataclass
class CommandNode(ASTNode):
    """Semantic: recognized intent/command"""
    intent: str  # "AUTHENTICATE", "UNLOCK", "COMMAND_SIT", etc.
    args: Dict[str, Any] = field(default_factory=dict)

    def evaluate(self, context: Dict[str, Any]) -> Dict[str, Any]:
        return {
            "type": "command",
            "intent": self.intent,
            "args": self.args,
            "confidence": self.confidence
        }

# ============================================================================
# LEXER: Raw Data -> Tokens
# ============================================================================

class GestureLexer:
    """
    Tokenizes hand landmark data into gesture tokens.
    Maps: raw hand pose + motion -> TokenType sequence
    """

    def __init__(self, motion_threshold: float = 0.5, confidence_threshold: float = 0.7):
        self.motion_threshold = motion_threshold
        self.confidence_threshold = confidence_threshold

    def tokenize(
        self,
        landmarks: List[tuple],  # 21 MediaPipe landmarks
        motion: Tuple[float, float],  # (v_toward, v_ortho)
        timestamp: float,
        confidence: float
    ) -> List[Token]:
        """
        Lex hand landmarks + motion into tokens
        """
        tokens = []

        if confidence < self.confidence_threshold:
            return tokens

        # 1. Classify hand configuration
        config_token = self._classify_config(landmarks, timestamp, confidence)
        if config_token:
            tokens.append(config_token)

        # 2. Classify motion
        motion_token = self._classify_motion(motion, timestamp, confidence)
        if motion_token:
            tokens.append(motion_token)

        return tokens

    def _classify_config(
        self,
        landmarks: List[tuple],
        timestamp: float,
        confidence: float
    ) -> Optional[Token]:
        """Classify hand configuration from 21 landmarks"""

        # Helper: is finger extended?
        def is_extended(tip_idx, pip_idx):
            if len(landmarks) > pip_idx:
                return landmarks[tip_idx][1] < landmarks[pip_idx][1]  # y coordinate
            return False

        # Extract finger states
        thumb_ext = len(landmarks) > 4
        index_ext = is_extended(8, 6)
        middle_ext = is_extended(12, 10)
        ring_ext = is_extended(16, 14)
        pinky_ext = is_extended(20, 18)

        fingers = [thumb_ext, index_ext, middle_ext, ring_ext, pinky_ext]
        extended_count = sum(fingers)

        # OK sign: thumb-index touching
        if len(landmarks) > 8:
            thumb_idx_dist = np.linalg.norm(
                np.array(landmarks[4][:2]) - np.array(landmarks[8][:2])
            )
            if thumb_idx_dist < 0.05 and not middle_ext:
                return Token(TokenType.OK_SIGN, "ok", timestamp, 0.95)

        # FIST: all fingers closed
        if extended_count == 0:
            return Token(TokenType.FIST, "fist", timestamp, 0.95)

        # OPEN_PALM: all fingers extended
        elif extended_count == 5:
            return Token(TokenType.OPEN_PALM, "open", timestamp, 0.95)

        # PEACE: index + middle only
        elif fingers[1] and fingers[2] and not fingers[3] and not fingers[4]:
            return Token(TokenType.PEACE, "peace", timestamp, 0.90)

        # THUMBS_UP: thumb only
        elif fingers[0] and not any(fingers[1:]):
            return Token(TokenType.THUMBS_UP, "thumbsup", timestamp, 0.90)

        # POINTING: index only
        elif fingers[1] and not any([fingers[i] for i in [0, 2, 3, 4]]):
            return Token(TokenType.POINTING, "point", timestamp, 0.85)

        return None

    def _classify_motion(
        self,
        motion: Tuple[float, float],
        timestamp: float,
        confidence: float
    ) -> Optional[Token]:
        """Classify motion from velocity vectors"""

        v_toward, v_ortho = motion

        if abs(v_toward) < 0.1 and abs(v_ortho) < 0.1:
            return Token(TokenType.STATIC, (0.0, 0.0), timestamp, 0.95)

        elif v_toward < -self.motion_threshold:
            return Token(TokenType.MOVING_TOWARD, (v_toward, v_ortho), timestamp, confidence)

        elif v_toward > self.motion_threshold:
            return Token(TokenType.MOVING_AWAY, (v_toward, v_ortho), timestamp, confidence)

        elif abs(v_ortho) > 0.3:
            return Token(TokenType.MOVING_ORTHO, (v_toward, v_ortho), timestamp, confidence)

        return None

# ============================================================================
# PARSER: Tokens -> AST
# ============================================================================

class GestureParser:
    """
    Parses token sequence into AST.
    Grammar:
        gesture ::= config_token motion_token
        sequence ::= gesture (gesture)*
        command ::= sequence (':' intent)?
    """

    def __init__(self):
        self.tokens: List[Token] = []
        self.pos = 0

    def parse(self, tokens: List[Token]) -> Optional[ASTNode]:
        """Parse token sequence into AST"""
        self.tokens = tokens
        self.pos = 0

        if not tokens:
            return None

        return self._parse_sequence()

    def _parse_sequence(self) -> Optional[ASTNode]:
        """sequence ::= gesture (gesture)*"""
        nodes = []
        start_time = self.tokens[self.pos].timestamp if self.pos < len(self.tokens) else 0.0

        while self.pos < len(self.tokens):
            node = self._parse_gesture()
            if node:
                nodes.append(node)
            else:
                break

        if not nodes:
            return None

        end_time = self.tokens[self.pos - 1].timestamp if self.pos > 0 else start_time
        duration = end_time - start_time
        min_conf = min(n.confidence for n in nodes)

        return SequenceNode(
            nodes=nodes,
            duration=duration,
            timestamp=start_time,
            confidence=min_conf
        )

    def _parse_gesture(self) -> Optional[ASTNode]:
        """gesture ::= config_token | motion_token | config_token motion_token"""

        if self.pos >= len(self.tokens):
            return None

        token = self.tokens[self.pos]

        # Configuration gesture
        if token.type in [TokenType.FIST, TokenType.OPEN_PALM, TokenType.POINTING,
                         TokenType.PEACE, TokenType.THUMBS_UP, TokenType.OK_SIGN]:
            self.pos += 1
            return HandConfigNode(
                config=token.type,
                timestamp=token.timestamp,
                confidence=token.confidence
            )

        # Motion gesture
        elif token.type in [TokenType.MOVING_TOWARD, TokenType.MOVING_AWAY,
                           TokenType.MOVING_ORTHO, TokenType.STATIC]:
            self.pos += 1
            velocity = token.value if isinstance(token.value, tuple) else (0.0, 0.0)
            return MotionNode(
                motion=token.type,
                velocity=velocity,
                timestamp=token.timestamp,
                confidence=token.confidence
            )

        return None

# ============================================================================
# INTERPRETER: AST -> Semantic State
# ============================================================================

class GestureInterpreter:
    """
    Evaluates AST into semantic state (commands, intents).
    P(ext) semantic layer: AST -> intent classification
    """

    def __init__(self):
        # Gesture pattern -> intent mapping
        self.intent_patterns: Dict[str, str] = {
            # Authentication
            "OK_SIGN": "AUTHENTICATE_BIOMETRIC",
            "THUMBS_UP": "CONFIRM_ACTION",
            "FIST": "CANCEL_OPERATION",

            # Commands (Rocky/Riley)
            "POINTING": "COMMAND_ATTENTION",
            "OPEN_PALM": "COMMAND_STOP",
            "PEACE": "COMMAND_STAY",
        }

    def evaluate(self, ast: ASTNode, context: Optional[Dict[str, Any]] = None) -> CommandNode:
        """
        Evaluate AST to produce command state.
        P(ext): AST -> CommandNode
        """
        if context is None:
            context = {}

        # Recursively evaluate AST
        result = ast.evaluate(context)

        # Determine intent from result
        intent = self._classify_intent(ast, result)

        return CommandNode(
            intent=intent,
            args=result,
            timestamp=ast.timestamp,
            confidence=ast.confidence
        )

    def _classify_intent(self, ast: ASTNode, result: Dict[str, Any]) -> str:
        """Map AST patterns to semantic intents"""

        if isinstance(ast, HandConfigNode):
            intent_key = ast.config.name
            return self.intent_patterns.get(intent_key, "GESTURE_RECOGNIZED")

        elif isinstance(ast, SequenceNode):
            # Compound gesture: combine intents
            intents = []
            for node in ast.nodes:
                if isinstance(node, HandConfigNode):
                    intents.append(node.config.name)

            sequence_str = "+".join(intents)
            return f"SEQUENCE:{sequence_str}"

        return "GESTURE_RECOGNIZED"

# ============================================================================
# SYMBOLIC INTERPRETER (Full Pipeline)
# ============================================================================

class SymbolicGestureInterpreter:
    """
    Complete P(ext): D -> S pipeline

    D: Raw hand landmarks + motion vectors
    ↓ LEXER
    Tokens: hand_config + motion
    ↓ PARSER
    AST: structured gesture representation
    ↓ INTERPRETER
    S: CommandNode (semantic state)
    """

    def __init__(self):
        self.lexer = GestureLexer(motion_threshold=0.5, confidence_threshold=0.7)
        self.parser = GestureParser()
        self.interpreter = GestureInterpreter()

    def execute(
        self,
        landmarks: List[tuple],
        motion: Tuple[float, float],
        timestamp: float,
        confidence: float
    ) -> CommandNode:
        """
        P(ext): D -> S
        Execute full symbolic interpretation pipeline
        """

        # STAGE 1: LEXER (D -> Tokens)
        tokens = self.lexer.tokenize(landmarks, motion, timestamp, confidence)

        if not tokens:
            return CommandNode(
                intent="NO_GESTURE",
                args={"reason": "insufficient_confidence"},
                timestamp=timestamp,
                confidence=0.0
            )

        # STAGE 2: PARSER (Tokens -> AST)
        ast = self.parser.parse(tokens)

        if not ast:
            return CommandNode(
                intent="PARSE_ERROR",
                args={"tokens": [str(t.type) for t in tokens]},
                timestamp=timestamp,
                confidence=0.0
            )

        # STAGE 3: INTERPRETER (AST -> CommandNode)
        command = self.interpreter.evaluate(ast)

        return command

# ============================================================================
# CODE GENERATOR (Inverse: S -> D)
# ============================================================================

class GestureCodeGenerator:
    """
    P(int): S -> D
    Code generation: semantic state -> gesture specification
    """

    def __init__(self):
        # Intent -> gesture spec mapping
        self.intent_specs = {
            "AUTHENTICATE_BIOMETRIC": {
                "hand_config": TokenType.OK_SIGN,
                "motion": TokenType.STATIC,
                "duration": 1.0
            },
            "CONFIRM_ACTION": {
                "hand_config": TokenType.THUMBS_UP,
                "motion": TokenType.MOVING_UP,
                "duration": 0.5
            },
            "CANCEL_OPERATION": {
                "hand_config": TokenType.FIST,
                "motion": TokenType.STATIC,
                "duration": 1.0
            },
            "COMMAND_ATTENTION": {
                "hand_config": TokenType.POINTING,
                "motion": TokenType.STATIC,
                "duration": 1.0
            },
        }

    def generate(self, command: CommandNode) -> Dict[str, Any]:
        """
        P(int): CommandNode -> gesture specification
        Generates hand pose + motion configuration
        """

        spec = self.intent_specs.get(command.intent, {
            "hand_config": TokenType.OPEN_PALM,
            "motion": TokenType.STATIC,
            "duration": 1.0
        })

        return {
            "intent": command.intent,
            "hand_config": spec["hand_config"].name,
            "motion": spec["motion"].name,
            "duration": spec["duration"],
            "timestamp": command.timestamp,
            "confidence": command.confidence
        }

# ============================================================================
# USAGE EXAMPLE
# ============================================================================

if __name__ == "__main__":
    # Initialize symbolic interpreter
    interpreter = SymbolicGestureInterpreter()

    # Simulate hand landmark data
    # 21 MediaPipe landmarks: (x, y, z, confidence)
    fake_landmarks = [
        (0.5, 0.5, 0.0, 0.99),  # 0: wrist
        (0.52, 0.48, 0.0, 0.99),  # 1-4: thumb
        (0.51, 0.47, 0.0, 0.99),
        (0.50, 0.46, 0.0, 0.99),
        (0.49, 0.45, 0.0, 0.99),
        (0.53, 0.48, 0.0, 0.98),  # 5-8: index
        (0.54, 0.46, 0.0, 0.98),
        (0.55, 0.44, 0.0, 0.98),
        (0.56, 0.42, 0.0, 0.98),
    ] + [(0.5, 0.5, 0.0, 0.90)] * 12  # remaining landmarks

    # Simulate motion: moving toward, static ortho
    motion = (-0.71, 0.34)  # toward=-0.71, ortho=0.34

    # Execute P(ext): D -> S
    command = interpreter.execute(
        landmarks=fake_landmarks,
        motion=motion,
        timestamp=1234567890.0,
        confidence=0.92
    )

    print(f"P(ext) Result:")
    print(f"  Intent: {command.intent}")
    print(f"  Args: {command.args}")
    print(f"  Confidence: {command.confidence}")

    # Execute P(int): S -> D
    generator = GestureCodeGenerator()
    gesture_spec = generator.generate(command)

    print(f"\nP(int) Result:")
    print(f"  Gesture Spec: {gesture_spec}")
