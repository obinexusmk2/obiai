"""
OBINexus Python Interface

High-level Python bindings for the Rust-based OBINexus self-aware probing system.

Usage:
    from obi_py import OBINexus

    system = OBINexus()
    system.learn("color", "blue")
    answer = system.ask("what")
    print(answer)
"""

__version__ = "0.1.0"
__author__ = "Nnamdi"

from .system import OBINexus
from .query import Query, QueryType
from .symbols import SymbolDatabase
from .probe import ProbeSystem

__all__ = [
    "OBINexus",
    "Query",
    "QueryType",
    "SymbolDatabase",
    "ProbeSystem",
]
