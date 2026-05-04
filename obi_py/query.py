"""
Query Engine: Canonical Questions

Implements the five canonical question types that form the basis of self-inquiry.
"""

from enum import Enum
from typing import Optional, Dict, Any


class QueryType(Enum):
    """Types of canonical questions"""
    WHO = "who"
    WHAT = "what"
    WHEN = "when"
    WHERE = "where"
    WHY = "why"
    HOW = "how"


class Query:
    """Represents a canonical query"""

    def __init__(self, question_type: QueryType, context: Optional[Dict[str, Any]] = None):
        """
        Initialize a query

        Args:
            question_type: Type of canonical question
            context: Additional context for the query
        """
        self.question_type = question_type
        self.context = context or {}

    def __str__(self) -> str:
        return f"Query({self.question_type.value})"

    def __repr__(self) -> str:
        return f"Query(type={self.question_type.value}, context={self.context})"

    @property
    def description(self) -> str:
        """Get human-readable description of the query"""
        descriptions = {
            QueryType.WHO: "Identity and ownership: Who am I?",
            QueryType.WHAT: "Nature and description: What am I?",
            QueryType.WHEN: "Temporal context: When am I active?",
            QueryType.WHERE: "Problem space position: Where am I?",
            QueryType.WHY: "Reason and causality: Why am I this way?",
            QueryType.HOW: "Mechanism and method: How do I work?",
        }
        return descriptions[self.question_type]
