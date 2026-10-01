#!/usr/bin/env python3
"""Graph Analyzer
Analyzes dependency and conflict relations across atomic rules.
"""
import argparse
import json
import sys
from pathlib import Path

def analyze_graph(ir_path: Path):
    with open(ir_path, "r", encoding="utf-8") as f:
        ir = json.load(f)
    rules = ir.get("atomic_rules", [])
    print(f"[*] Analyzing semantic graph for {len(rules)} atomic rules...")
    # Generate nodes & empty edge list for initial graph
    graph = {
        "nodes": [{"id": r["id"], "label": r.get("description", "")[:30], "category": r.get("type", "generic")} for r in rules],
        "edges": []
    }
    return graph

def main():
    parser = argparse.ArgumentParser(description="Analyze Semantic Dependency Graph")
    parser.add_argument("--ir", type=str, required=True, help="Path to prompt_ir.json")
    args = parser.parse_args()
    graph = analyze_graph(Path(args.ir))
    print(json.dumps(graph, indent=2))

if __name__ == "__main__":
    main()
