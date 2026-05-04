"""
Symbol Database: Knowledge Base Management

Provides high-level access to the symbol table.
"""

from typing import Optional, List, Dict, Any
from dataclasses import dataclass
import json


@dataclass
class Symbol:
    """Represents a symbol in the knowledge base"""
    name: str
    value: Any
    confidence: float = 1.0  # 0.0 to 1.0
    metadata: Dict[str, str] = None

    def __post_init__(self):
        if self.metadata is None:
            self.metadata = {}

    def __str__(self) -> str:
        return f"Symbol({self.name}={self.value}, confidence={self.confidence:.2f})"

    def to_dict(self) -> Dict[str, Any]:
        return {
            "name": self.name,
            "value": self.value,
            "confidence": self.confidence,
            "metadata": self.metadata,
        }


class SymbolDatabase:
    """High-level interface to the symbol table"""

    def __init__(self):
        """Initialize the symbol database"""
        self._symbols: Dict[str, Symbol] = {}
        self._history: List[tuple] = []

    def insert(self, name: str, value: Any, confidence: float = 1.0) -> None:
        """
        Insert a symbol into the database

        Args:
            name: Symbol name
            value: Symbol value
            confidence: Confidence level (0.0 to 1.0)
        """
        symbol = Symbol(name, value, confidence)
        self._symbols[name] = symbol
        self._history.append(("insert", name, value))

    def get(self, name: str) -> Optional[Symbol]:
        """Get a symbol by name"""
        return self._symbols.get(name)

    def contains(self, name: str) -> bool:
        """Check if a symbol exists"""
        return name in self._symbols

    def remove(self, name: str) -> bool:
        """Remove a symbol"""
        if name in self._symbols:
            del self._symbols[name]
            self._history.append(("remove", name, None))
            return True
        return False

    def query(self, pattern: str) -> List[Symbol]:
        """
        Query symbols by pattern (substring match)

        Args:
            pattern: Search pattern

        Returns:
            List of matching symbols
        """
        return [s for s in self._symbols.values()
                if pattern.lower() in s.name.lower() or pattern.lower() in str(s.value).lower()]

    def all(self) -> List[Symbol]:
        """Get all symbols"""
        return list(self._symbols.values())

    def size(self) -> int:
        """Get number of symbols"""
        return len(self._symbols)

    def average_confidence(self) -> float:
        """Get average confidence across all symbols"""
        if not self._symbols:
            return 0.0
        return sum(s.confidence for s in self._symbols.values()) / len(self._symbols)

    def export(self) -> str:
        """Export database as JSON"""
        data = {
            "symbols": [s.to_dict() for s in self._symbols.values()],
            "count": len(self._symbols),
            "average_confidence": self.average_confidence(),
        }
        return json.dumps(data, indent=2)

    def __len__(self) -> int:
        return len(self._symbols)

    def __contains__(self, name: str) -> bool:
        return name in self._symbols

    def __getitem__(self, name: str) -> Optional[Symbol]:
        return self.get(name)

    def __repr__(self) -> str:
        return f"SymbolDatabase(size={len(self._symbols)}, avg_confidence={self.average_confidence():.2f})"
