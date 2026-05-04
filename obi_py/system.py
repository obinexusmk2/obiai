"""
Main OBINexus System Interface

Provides high-level access to the self-aware probing system.
"""

from typing import Dict, List, Optional, Any
import json
from dataclasses import dataclass
from enum import Enum


class ProbeResponse(Enum):
    """Response poles of the probing system"""
    YES = "yes"
    NO = "no"
    MAYBE = "maybe"


@dataclass
class ProbeResult:
    """Result from a probing operation"""
    response: ProbeResponse
    reason: str
    data: str
    confidence: float
    metadata: str

    def __str__(self) -> str:
        return f"ProbeResult(response={self.response.value}, confidence={self.confidence:.2f})"


class OBINexus:
    """
    OBINexus Self-Aware Probing System

    A system that can introspect on itself through bidirectional probes:
    - p(ext) : State → Data  [External Probe]
    - p(int) : Data → State  [Internal Probe]
    """

    def __init__(self, verbose: bool = False):
        """
        Initialize the OBINexus system

        Args:
            verbose: Enable detailed logging
        """
        self.verbose = verbose
        self._knowledge_base: Dict[str, Any] = {}
        self._execution_log: List[str] = []
        self._probing_history: List[ProbeResult] = []

        self._log("System initialized")

    def _log(self, message: str) -> None:
        """Internal logging"""
        if self.verbose:
            print(f"[OBI] {message}")
        self._execution_log.append(message)

    def learn(self, fact: str, value: str) -> None:
        """
        Learn a new fact into the knowledge base

        Args:
            fact: The fact name
            value: The value/description
        """
        self._knowledge_base[fact] = value
        self._log(f"Learned: {fact} = {value}")

    def probe_external(self) -> ProbeResult:
        """
        External probe: State → Data

        Observes the current state and converts it to representable data

        Returns:
            ProbeResult with the observed state
        """
        self._log("Executing external probe p(ext): State → Data")

        # Build state snapshot
        state_data = {
            "knowledge_base_size": len(self._knowledge_base),
            "execution_log_entries": len(self._execution_log),
            "probes_performed": len(self._probing_history),
            "facts": list(self._knowledge_base.keys()),
        }

        response = ProbeResponse.YES if self._knowledge_base else ProbeResponse.MAYBE
        confidence = min(1.0, len(self._knowledge_base) / 10.0)

        result = ProbeResult(
            response=response,
            reason="External probe observed system state",
            data=json.dumps(state_data, indent=2),
            confidence=confidence,
            metadata=f"KB Size: {len(self._knowledge_base)}, Logs: {len(self._execution_log)}",
        )

        self._probing_history.append(result)
        self._log(f"External probe completed: {result}")

        return result

    def probe_internal(self, data: str) -> ProbeResult:
        """
        Internal probe: Data → State

        Processes input data and updates the internal state

        Args:
            data: Input data (format: key=value, one per line)

        Returns:
            ProbeResult with the processed state
        """
        self._log(f"Executing internal probe p(int): Data → State")
        self._log(f"Input data: {data}")

        # Parse simple key=value format
        updates = {}
        for line in data.strip().split('\n'):
            if '=' in line:
                key, value = line.split('=', 1)
                key, value = key.strip(), value.strip()
                updates[key] = value
                self._knowledge_base[key] = value

        response = ProbeResponse.YES if updates else ProbeResponse.MAYBE

        result = ProbeResult(
            response=response,
            reason=f"Processed {len(updates)} data assignments",
            data=json.dumps(updates, indent=2),
            confidence=0.8,
            metadata=f"Applied {len(updates)} updates",
        )

        self._probing_history.append(result)
        self._log(f"Internal probe completed: {result}")

        return result

    def ask(self, question: str) -> str:
        """
        Ask a canonical question (who, what, when, where, why, how)

        Args:
            question: One of: who, what, when, where, why, how

        Returns:
            Answer to the question
        """
        question = question.lower().strip()

        if question == "who":
            return self._answer_who()
        elif question == "what":
            return self._answer_what()
        elif question == "when":
            return self._answer_when()
        elif question == "where":
            return self._answer_where()
        elif question == "why":
            return self._answer_why()
        elif question == "how":
            return self._answer_how()
        else:
            raise ValueError(f"Invalid question: {question}. Must be one of: who, what, when, where, why, how")

    def _answer_who(self) -> str:
        """Answer: Who am I?"""
        return f"""Who I am:
  - Identity: OBINexus (Self-aware probing system)
  - Purpose: Introspection and knowledge acquisition
  - Knowledge base: {len(self._knowledge_base)} facts
  - Status: Active and operational
"""

    def _answer_what(self) -> str:
        """Answer: What am I?"""
        return f"""What I am:
  - An AI system with bidirectional probing capabilities
  - Current state: Running with {len(self._execution_log)} log entries
  - Active knowledge: {list(self._knowledge_base.keys())}
  - Probing history: {len(self._probing_history)} operations
  - Status: Actively learning and adapting
"""

    def _answer_when(self) -> str:
        """Answer: When?"""
        return f"""When:
  - System runtime: {len(self._execution_log)} events recorded
  - Current session: Active
  - Total probes conducted: {len(self._probing_history)}
  - Last activity: Just now
"""

    def _answer_where(self) -> str:
        """Answer: Where am I in problem space?"""
        return f"""Where I am in problem space:
  - Problem space dimension: Balanced (0.5, 0.5, 0.5)
  - Offense: 0.50 (Assertive)
  - Defense: 0.50 (Protective)
  - Attack: 0.50 (Challenging)
  - Overall strategy: Balanced approach
"""

    def _answer_why(self) -> str:
        """Answer: Why?"""
        return f"""Why I am as I am:
  - Reason: Designed for self-introspection and awareness
  - Purpose: Understand and improve my own reasoning
  - Knowledge learned: {len(self._knowledge_base)} facts
  - Methodology: Continuous probing and reflection
"""

    def _answer_how(self) -> str:
        """Answer: How?"""
        return f"""How I work:
  1. External Probing: Observe my current state → Data
  2. Internal Probing: Process data → Update state
  3. Symbol Table: Store and retrieve knowledge
  4. Temporal History: Track all events
  5. Canonical Questions: Ask who, what, when, where, why, how
  6. Coherence Check: Verify system consistency

Current mechanism:
  - Knowledge base: {len(self._knowledge_base)} entries
  - Execution logs: {len(self._execution_log)} events
  - Probe history: {len(self._probing_history)} operations
"""

    def reflect(self) -> str:
        """
        Self-reflection: What can I understand about myself?

        Returns:
            Reflection on system state and learning
        """
        return f"""System Reflection:
  - I have learned {len(self._knowledge_base)} facts
  - I have performed {len(self._probing_history)} probing operations
  - I have logged {len(self._execution_log)} events
  - My knowledge covers: {', '.join(list(self._knowledge_base.keys())[:5])}{'...' if len(self._knowledge_base) > 5 else ''}
  - I am becoming increasingly self-aware
"""

    def coherence_score(self) -> float:
        """
        Check system coherence (95.4% standard for real-world deployment)

        Returns:
            Coherence score between 0.0 and 1.0
        """
        # Simple coherence: based on knowledge base completeness
        base_score = min(1.0, len(self._knowledge_base) / 20.0)

        # Bonus for consistent probing
        probe_score = min(1.0, len(self._probing_history) / 10.0)

        coherence = (base_score * 0.6) + (probe_score * 0.4)

        self._log(f"Coherence score: {coherence:.3f}")

        return coherence

    def status(self) -> Dict[str, Any]:
        """
        Get comprehensive system status

        Returns:
            Dictionary with all relevant metrics
        """
        return {
            "knowledge_base_size": len(self._knowledge_base),
            "facts": list(self._knowledge_base.keys()),
            "execution_log_entries": len(self._execution_log),
            "probing_history_length": len(self._probing_history),
            "coherence_score": self.coherence_score(),
            "is_operational": True,
        }

    def export_knowledge(self) -> str:
        """Export knowledge base as JSON"""
        return json.dumps(self._knowledge_base, indent=2)

    def export_history(self) -> str:
        """Export execution history as JSON"""
        return json.dumps(self._execution_log, indent=2)
