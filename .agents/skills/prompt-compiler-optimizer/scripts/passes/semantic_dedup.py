"""Pass: Semantic Deduplication
Eliminates redundant atomic rules with duplicate or identical semantic intent.
"""
from typing import Dict, Any, List

def run_pass(ir: Dict[str, Any], options: Dict[str, Any] = None) -> Dict[str, Any]:
    rules = ir.get("atomic_rules", [])
    seen_descriptions = set()
    deduped_rules: List[Dict[str, Any]] = []

    for rule in rules:
        normalized = rule.get("description", "").strip().lower()
        if normalized not in seen_descriptions:
            seen_descriptions.add(normalized)
            deduped_rules.append(rule)

    ir["atomic_rules"] = deduped_rules
    return ir
