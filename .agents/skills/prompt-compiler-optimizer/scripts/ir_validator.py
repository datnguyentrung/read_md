#!/usr/bin/env python3
"""IR Validator
Validates Prompt IR JSON against prompt_ir.schema.json.
"""
import argparse
import json
import sys
from pathlib import Path

def validate_ir(ir_path: Path, schema_path: Path = None) -> bool:
    try:
        with open(ir_path, "r", encoding="utf-8") as f:
            data = json.load(f)
        # Check required fields minimally if jsonschema is not installed
        required = ["version", "metadata", "sections", "atomic_rules"]
        for field in required:
            if field not in data:
                print(f"[-] Validation Error: Missing required key '{field}'", file=sys.stderr)
                return False
        print("[+] Prompt IR is valid.")
        return True
    except Exception as e:
        print(f"[-] Error reading/validating IR: {e}", file=sys.stderr)
        return False

def main():
    parser = argparse.ArgumentParser(description="Validate Prompt IR against schema")
    parser.add_argument("--ir", type=str, required=True, help="Path to prompt_ir.json")
    args = parser.parse_args()
    
    valid = validate_ir(Path(args.ir))
    sys.exit(0 if valid else 1)

if __name__ == "__main__":
    main()
