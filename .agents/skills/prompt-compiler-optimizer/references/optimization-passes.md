# Optimization Passes Reference

## Standard Optimization Pipeline

1. **Normalize Terms (`normalize_terms.py`)**:
   Standardizes synonym variations, jargon, formatting references, and naming conventions.

2. **Semantic Deduplication (`semantic_dedup.py`)**:
   Detects and eliminates redundant rules, duplicated prohibitions, and restatements.

3. **Reorder Structure (`reorder_structure.py`)**:
   Restructures prompt sections to maximize attention retention (e.g., priming role -> constraints -> format -> examples).

4. **Generalization (`generalization.py`)**:
   Synthesizes multiple ad-hoc rules into higher-level abstract instructions without loss of edge-case coverage.

5. **Decision Tree Extraction (`decision_tree.py`)**:
   Converts complex nested if/else conversational branching into compact decision trees or lookup tables.

6. **Externalization (`externalization.py`)**:
   Extracts dynamic context, few-shot examples, or lookup tables into external tool payloads / system documents.

7. **Modularization (`modularization.py`)**:
   Partitions monolithic prompts into modular sub-agent definitions or skill references.
