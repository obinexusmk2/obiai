"""
Probing System: Bidirectional State ↔ Data Transformation

Implements p(int) and p(ext) operations.
"""

from typing import Optional, Dict, Any
from dataclasses import dataclass
from enum import Enum
import json


class ProbeResponse(Enum):
    """Three poles of consensus"""
    YES = "yes"
    NO = "no"
    MAYBE = "maybe"


@dataclass
class ProbeOperation:
    """Result of a probe operation"""
    response: ProbeResponse
    reason: str
    data: str
    confidence: float
    metadata: str

    def __str__(self) -> str:
        return f"Probe({self.response.value}, confidence={self.confidence:.2f})"

    def to_dict(self) -> Dict[str, Any]:
        return {
            "response": self.response.value,
            "reason": self.reason,
            "data": self.data,
            "confidence": self.confidence,
            "metadata": self.metadata,
        }


class ProbeSystem:
    """
    Probing system for bidirectional state transformation

    p(ext) : State → Data  (External probe)
    p(int) : Data → State  (Internal probe)
    """

    def __init__(self):
        """Initialize the probing system"""
        self._external_probes: List[ProbeOperation] = []
        self._internal_probes: List[ProbeOperation] = []

    def external(self, state: Dict[str, Any]) -> ProbeOperation:
        """
        External Probe: State → Data

        Converts internal execution state to representable data

        Args:
            state: Current execution state

        Returns:
            ProbeOperation with observed data
        """
        # Convert state to observable data
        data = json.dumps(state, indent=2)

        # Determine response based on state richness
        if not state:
            response = ProbeResponse.MAYBE
            confidence = 0.3
        elif len(state) > 5:
            response = ProbeResponse.YES
            confidence = 0.9
        else:
            response = ProbeResponse.YES
            confidence = 0.7

        operation = ProbeOperation(
            response=response,
            reason="External probe observed current state",
            data=data,
            confidence=confidence,
            metadata=f"State size: {len(state)}, keys: {list(state.keys())}",
        )

        self._external_probes.append(operation)
        return operation

    def internal(self, data: str) -> ProbeOperation:
        """
        Internal Probe: Data → State

        Processes input data and updates internal state

        Args:
            data: Input data to process

        Returns:
            ProbeOperation with processed result
        """
        # Parse the data
        parsed = {}
        for line in data.strip().split('\n'):
            if '=' in line:
                key, value = line.split('=', 1)
                parsed[key.strip()] = value.strip()

        response = ProbeResponse.YES if parsed else ProbeResponse.MAYBE
        confidence = min(1.0, len(parsed) / 5.0) if parsed else 0.5

        operation = ProbeOperation(
            response=response,
            reason=f"Internal probe processed {len(parsed)} assignments",
            data=json.dumps(parsed, indent=2),
            confidence=confidence,
            metadata=f"Parsed {len(parsed)} key-value pairs",
        )

        self._internal_probes.append(operation)
        return operation

    def consistency_check(self) -> float:
        """
        Check consistency of external and internal probes

        Returns:
            Consistency score (0.0 to 1.0)
        """
        if not self._external_probes or not self._internal_probes:
            return 0.5

        # Simple consistency metric: matching responses
        matching = 0
        total = min(len(self._external_probes), len(self._internal_probes))

        for i in range(total):
            if self._external_probes[i].response == self._internal_probes[i].response:
                matching += 1

        return matching / total if total > 0 else 0.5

    def external_history(self) -> list:
        """Get history of external probes"""
        return [op.to_dict() for op in self._external_probes]

    def internal_history(self) -> list:
        """Get history of internal probes"""
        return [op.to_dict() for op in self._internal_probes]

    def clear_history(self) -> None:
        """Clear probing history"""
        self._external_probes.clear()
        self._internal_probes.clear()

    def __repr__(self) -> str:
        return (f"ProbeSystem(external_probes={len(self._external_probes)}, "
                f"internal_probes={len(self._internal_probes)})")


# Type hints for clarity
from typing import List
