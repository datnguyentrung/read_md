# Verification & Regression Protocols

## Principles
1. **Semantic Invariant Guarantee**: Optimization passes must never violate critical business rules or constraints marked as `CRITICAL`.
2. **Deterministic Output Checks**: Test cases must verify formatting conformity, refusal behaviors, and guardrails.
3. **Token & Latency Delta**: Measure exact token savings against potential quality degradation.
