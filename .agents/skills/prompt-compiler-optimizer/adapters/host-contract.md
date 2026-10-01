# Host Contract Adapter

## Overview
Defines the interface between the Prompt Compiler & Optimizer and host runtime environments (Agent runners, LangChain, LlamaIndex, Semantic Kernel, custom API wrappers).

## Methods
- `compile(raw_prompt: str, profile: str) -> OptimizedPromptResult`
- `validate(ir_payload: dict) -> ValidationReport`
- `inspect_graph(ir_payload: dict) -> SemanticGraph`
