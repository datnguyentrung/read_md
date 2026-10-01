#!/usr/bin/env python3
"""Prompt Metrics
Calculates token length estimates, readability metrics, and structural complexity.
"""
import argparse
import sys
from pathlib import Path

def calculate_metrics(text: str):
    words = len(text.split())
    # Rough estimate ~ 1.3 tokens per word for English / code
    estimated_tokens = int(words * 1.3)
    lines = len(text.splitlines())
    return {
        "lines": lines,
        "words": words,
        "estimated_tokens": estimated_tokens
    }

def main():
    parser = argparse.ArgumentParser(description="Calculate prompt metrics")
    parser.add_argument("--file", "-f", type=str, required=True, help="Path to prompt file")
    args = parser.parse_args()
    
    with open(args.file, "r", encoding="utf-8") as f:
        content = f.read()
        
    metrics = calculate_metrics(content)
    print(f"[Metrics for {args.file}]")
    for k, v in metrics.items():
        print(f"  {k}: {v}")

if __name__ == "__main__":
    main()
