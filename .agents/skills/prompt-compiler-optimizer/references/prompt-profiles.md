# Target Model Prompt Profiles

## Profiles

### 0. `GENERIC_UNKNOWN`
- Chỉ sử dụng transformation model-agnostic đã kiểm chứng.
- Capability chưa có bằng chứng là `UNKNOWN`, không phải `SUPPORTED`.
- Không cam kết tương thích model-specific và phải ghi profile resolution debt.

### 1. `claude-3-7-sonnet`
- XML tag structure preference (`<instructions>`, `<constraints>`).
- High attention retention across middle context.
- Native thinking block support.

### 2. `gpt-4o`
- Markdown-focused structure.
- Strong prefix/suffix primacy weighting.
- Explicit schema definitions via JSON/Markdown tables.

### 3. `gemini-1.5-pro` / `gemini-2.0`
- Massive context capability.
- Multi-modal and structured reasoning markers.
- Robust handling of long nested instructions.
