#!/usr/bin/env python3
"""Run Pipeline
Main entrypoint to run the multi-pass prompt compiler and optimizer pipeline.
"""
import argparse
import json
import os
import sys
from pathlib import Path

# Local imports
from passes import normalize_terms, semantic_dedup, reorder_structure

def main():
    parser = argparse.ArgumentParser(description="Prompt Compiler & Optimizer Pipeline")
    parser.add_argument("--input", "-i", type=str, required=True, help="Input prompt file (Markdown/Text)")
    parser.add_argument("--output-dir", "-o", type=str, default="./artifacts", help="Directory to save output artifacts")
    parser.add_argument("--profile", "-p", type=str, default="gpt-4o", help="Target model profile (e.g. gpt-4o, claude-3-7-sonnet)")
    args = parser.parse_args()

    input_path = Path(args.input)
    if not input_path.exists():
        print(f"Error: Input file not found: {input_path}", file=sys.stderr)
        sys.exit(1)

    with open(input_path, "r", encoding="utf-8") as f:
        raw_text = f.read()

    print(f"[*] Compiling prompt from {input_path} for profile: {args.profile}")

    # Simulated Front-End Parsing to IR
    ir = {
        "version": "1.0.0",
        "metadata": {
            "title": input_path.stem,
            "target_model": args.profile
        },
        "sections": [
            {"id": "s1", "title": "System Prompt", "order": 1, "content": raw_text}
        ],
        "atomic_rules": [
            {"id": "r1", "type": "behavior", "description": raw_text[:200].strip(), "priority": "HIGH"}
        ]
    }

    # Running Optimization Passes
    print("[*] Running optimization passes...")
    ir = normalize_terms.run_pass(ir)
    ir = semantic_dedup.run_pass(ir)
    ir = reorder_structure.run_pass(ir)

    out_dir = Path(args.output_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    ir_out = out_dir / "prompt_ir.json"
    
    with open(ir_out, "w", encoding="utf-8") as f:
        json.dump(ir, f, indent=2)

    print(f"[+] Compiled IR written to: {ir_out}")

if __name__ == "__main__":
    main()
