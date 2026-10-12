# Host Contract Adapter V5

## Overview

Defines the interface between the Prompt Compiler & Optimizer and host runtime environments (Agent runners, LangChain, LlamaIndex, Semantic Kernel, custom API wrappers). Hosts must pass evidence explicitly and must not infer support from a missing capability value.

## Compile

`compile(raw_prompt: str, config: CompileConfig) -> PipelineReport`

`CompileConfig` accepts optional `profile`, `goal`, `tier`, `token_budget`, `latency_budget_ms`, `min_assurance`, `runtime_capabilities`, `static_evidence`, `differential_evidence`, and `manual_override`. Missing profile/capability evidence resolves to `UNKNOWN`; missing goal falls back to `BALANCED_PRESERVATION` when contextual inference is inconclusive.

The return value separates:

- `planning.execution_status`
- `verification.verification_status`
- `verification.assurance_level`
- `verification.release_decision`

## Supporting Methods

- `validate(ir_payload: dict) -> ValidationReport`
- `inspect_graph(ir_payload: dict) -> SemanticGraph`
- `capabilities() -> PassCapability[]`

## Persistence

Compilation is in-memory by default. A host may persist the returned report itself or request an audit bundle through the CLI `--output-dir`; absence of that option must not create a file or directory.
