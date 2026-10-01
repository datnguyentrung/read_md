"""Pass: Reorder Structure
Reorders sections and rules to optimize attention weighting and follow model profile best practices.
"""
from typing import Dict, Any

PRIORITY_WEIGHT = {
    "CRITICAL": 0,
    "HIGH": 1,
    "MEDIUM": 2,
    "LOW": 3
}

def run_pass(ir: Dict[str, Any], options: Dict[str, Any] = None) -> Dict[str, Any]:
    rules = ir.get("atomic_rules", [])
    # Sort rules primarily by priority, preserving section alignment if requested
    rules.sort(key=lambda r: PRIORITY_WEIGHT.get(r.get("priority", "MEDIUM"), 2))
    ir["atomic_rules"] = rules
    return ir
