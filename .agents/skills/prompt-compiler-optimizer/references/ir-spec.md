# Prompt Intermediate Representation (IR) Specification

## Overview
The Prompt IR is a canonical, AST-like representation of natural language prompt instructions that decouples prompt semantics from specific formatting styles.

## Schema Hierarchy
1. **Metadata**: Global target configs (model profile, budget, author, version).
2. **Sections**: Top-level structural blocks (Role, Context, Guidelines, Output Format, Examples).
3. **Atomic Rules**: Finite units of logic, constraints, and instructions tagged with precedence.

## Key Properties
- Declarative rule breakdown
- Formal conditions and triggers
- Priority ordering (`CRITICAL` > `HIGH` > `MEDIUM` > `LOW`)
