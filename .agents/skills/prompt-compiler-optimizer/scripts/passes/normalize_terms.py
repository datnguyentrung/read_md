"""Pass: Normalize Terms
Standardizes naming conventions, technical terminology, and casing across prompt rules.
"""
from typing import Dict, Any

def run_pass(ir: Dict[str, Any], options: Dict[str, Any] = None) -> Dict[str, Any]:
    synonym_map = (options or {}).get("synonyms", {})
    rules = ir.get("atomic_rules", [])
    
    for rule in rules:
        desc = rule.get("description", "")
        for k, v in synonym_map.items():
            desc = desc.replace(k, v)
        rule["description"] = desc
        
    ir["atomic_rules"] = rules
    return ir
