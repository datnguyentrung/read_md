"""Tests for Prompt Compiler & Optimizer Pipeline"""
import unittest
from pathlib import Path
import sys

# Add scripts directory to path
scripts_dir = Path(__file__).parent.parent / "scripts"
sys.path.insert(0, str(scripts_dir))

from passes import normalize_terms, semantic_dedup, reorder_structure

class TestPromptCompiler(unittest.TestCase):
    def test_semantic_dedup(self):
        sample_ir = {
            "atomic_rules": [
                {"id": "1", "description": "Always answer in Vietnamese", "priority": "HIGH"},
                {"id": "2", "description": "always answer in vietnamese", "priority": "HIGH"},
                {"id": "3", "description": "Be polite", "priority": "LOW"}
            ]
        }
        res = semantic_dedup.run_pass(sample_ir)
        self.assertEqual(len(res["atomic_rules"]), 2)

    def test_normalize_terms(self):
        sample_ir = {
            "atomic_rules": [
                {"id": "1", "description": "Output in JSON format", "priority": "MEDIUM"}
            ]
        }
        res = normalize_terms.run_pass(sample_ir, {"synonyms": {"JSON format": "valid JSON object"}})
        self.assertEqual(res["atomic_rules"][0]["description"], "Output in valid JSON object")

if __name__ == "__main__":
    unittest.main()
