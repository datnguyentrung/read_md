---
name: prompt-compiler-optimizer
description: Portable Agent Skill biên dịch và tối ưu hóa prompt dựa trên mô hình chuỗi công cụ compiler (Analyze -> IR -> Semantic Graph -> Optimization Passes -> Compile -> Adaptive Verification L0-L4 -> Release Gate). Bảo toàn 100% bất biến và ranh giới quyết định, không khóa host/model/framework.
---

# Kỹ Năng Biên Dịch & Tối Ưu Hóa Prompt (V4.1 Portable Agent Skill)

Một portable Agent Skill theo mô hình "chuỗi công cụ kiểu trình biên dịch" (compiler toolchain) giúp phân tích, chuẩn hóa sang Biểu Diễn Trung Gian (IR), tối ưu hóa có kiểm soát rủi ro, biên dịch kiến trúc và kiểm chứng thích nghi theo quyền truy cập hệ thống.

---

## 1. Nguyên Tắc Cốt Lõi (Core Principles)

1. **Không tối ưu chỉ vì prompt dài**: Chỉ chấp nhận thay đổi khi giảm độ phức tạp logic hoặc token mà vẫn bảo toàn 100% hành vi cốt lõi và không làm sai ranh giới quyết định.
2. **Quyền kết luận `KEEP`**: Nếu prompt đã tối ưu, ít xung đột, nằm trong ngân sách token, hệ thống giữ nguyên prompt gốc và xuất báo cáo audit thay vì cố nén.
3. **Bảo toàn Invariants (INV-01 đến INV-09)**: Mọi quy tắc, ngoại lệ và toàn bộ các ví dụ minh họa bắt buộc phải được bảo toàn xuyên suốt các lượt tối ưu (Xem chi tiết tại `references/ir-spec.md`).
4. **Bảo toàn 100% Ví dụ (Full Example Preservation - Zero Loss)**: Mọi ví dụ trong prompt gốc (few-shot examples, JSON shorthand payloads, mẫu hội thoại, mẫu bảng/biểu đồ, ví dụ xử lý tình huống) BẮT BUỘC phải được giữ lại đầy đủ 100%. Quá trình tối ưu chỉ tái cấu trúc và khử trùng lặp câu chữ mô tả, TUYỆT ĐỐI KHÔNG được xóa bỏ, cắt bớt hay làm mất ví dụ.
5. **Kiểm chứng thích nghi theo bằng chứng (L0–L4)**: Không phụ thuộc vào việc có gọi được API thật hay không. Hệ thống tự chọn tầng kiểm thử phù hợp nhất từ L0 (Static) đến L4 (E2E) và ghi rõ `verification_debt`.
6. **Xuất kết quả trực tiếp trong Chat (In-Chat Markdown Delivery)**: Toàn bộ báo cáo phân tích, chỉ số đo lường và Prompt ứng viên đã biên dịch được xuất TRỰC TIẾP trong đoạn chat trên màn hình dưới dạng Markdown. Prompt tối ưu BẮT BUỘC được đặt trong Fenced Code Block (```yaml hoặc ```markdown) để có sẵn nút Copy mặc định.

---

## 2. Bản Đồ Cấu Trúc Tài Nguyên (Skill Resource Registry)

Skill được tổ chức phân lớp rõ ràng giữa **Nhận thức LLM (Cognitive Prompts)**, **Đặc tả tri thức (References)**, **Chuẩn hóa dữ liệu (Schemas)** và **Engine thực thi tất định (Rust Engine)** theo đường dẫn tương đối (Relative Paths) đảm bảo tính linh hoạt và chuyển giao đa nền tảng:

| Thư mục / Thành phần | Tệp tin tương đối | Vai trò & Mục đích sử dụng |
| :--- | :--- | :--- |
| **`prompts/`** *(LLM Sub-prompts)* | `prompts/semantic-parser.md`<br>`prompts/relation-classifier.md`<br>`prompts/generalizer.md`<br>`prompts/test-generator.md`<br>`prompts/test-interpreter.md` | Các chỉ dẫn chuẩn cho Sub-agent/LLM khi đóng vai trò Frontend Parser, phân loại quan hệ ngữ nghĩa, tổng quát hóa quy tắc và sinh test case kiểm chứng. |
| **`references/`** *(Knowledge Base)* | `references/ir-spec.md`<br>`references/optimization-passes.md`<br>`references/release-policy.md`<br>`references/verification.md`<br>`references/prompt-profiles.md` | Bộ đặc tả tiêu chuẩn (Specification): 9 Invariants cốt lõi, ma trận rủi ro từng pass, chính sách Release Gate và hồ sơ kiến trúc cho từng mô hình LLM đích. |
| **`schemas/`** *(JSON Schemas)* | `schemas/prompt_ir.schema.json`<br>`schemas/atomic_rule.schema.json`<br>`schemas/semantic_graph.schema.json`<br>`schemas/optimization_plan.schema.json`<br>`schemas/verification_report.schema.json` | Lược đồ JSON ràng buộc kiểu dữ liệu cho toàn bộ các cấu trúc dữ liệu trung gian trong compiler pipeline. |
| **`scripts/`** *(Deterministic Engine)* | `scripts/types.rs`<br>`scripts/run_pipeline.rs`<br>`scripts/graph_analyzer.rs`<br>`scripts/ir_validator.rs`<br>`scripts/invariant_checker.rs`<br>`scripts/prompt_metrics.rs`<br>`scripts/regression_runner.rs` | Mã nguồn Rust thực thi tính toán tất định: xác thực cấu trúc, phân tích đồ thị, đo đạc token metrics và kiểm chứng bất biến. |
| **`scripts/passes/`** *(Optimization Passes)* | `scripts/passes/normalize_terms.rs`<br>`scripts/passes/semantic_dedup.rs`<br>`scripts/passes/reorder_structure.rs`<br>`scripts/passes/preserve_and_structure_examples.rs`<br>`scripts/passes/generalization.rs`<br>`scripts/passes/decision_tree.rs`<br>`scripts/passes/modularization.rs`<br>`scripts/passes/externalization.rs` | Các thuật toán biến đổi IR chuyên biệt: chuẩn hóa từ đồng nghĩa, khử trùng lặp ngữ nghĩa, sắp xếp lại thứ tự ưu tiên, bảo toàn & cấu trúc hóa ví dụ. |
| **`adapters/`** *(Host Contract)* | `adapters/host-contract.md` | Giao diện tương thích cho môi trường Agent runner bên ngoài (LangChain, LlamaIndex, Semantic Kernel). |

---

## 3. Quy Trình Tuyến Tính Chi Tiết (End-to-End Pipeline Workflow)

```mermaid
graph TD
    Raw[Prompt Nguồn / Context] -->|Phase 1: Ingestion & Parse| IR[Prompt IR In-Memory]
    IR -->|Phase 2: Graph Analysis| Graph[Đồ Thị Ngữ Nghĩa]
    Graph -->|Phase 3: Optimization Plan| Plan[Kế Hoạch Tối Ưu]
    Plan -->|Phase 4: Passes Engine| OptIR[Optimized IR + ChangeLog]
    OptIR -->|Phase 5: Target Profile Compilation| Candidate[Prompt Ứng Viên]
    Candidate -->|Phase 6: Verification & Gate| ChatReport[Báo Cáo + Code Block Có Nút Copy Trong Chat]
```

### 📍 Phase 1: Tiếp Nhận & Nguyên Tử Hóa (Ingestion & Semantic Parsing)
- **Mục đích**: Bóc tách toàn bộ văn bản prompt thô thành các đơn vị ý nghĩa (`SemanticUnit`) và quy tắc nguyên tử (`AtomicRule`) độc lập, bảo toàn trọn vẹn 100% các khối ví dụ (`example`).
- **Tài nguyên sử dụng**: `prompts/semantic-parser.md`, `references/ir-spec.md`, `schemas/prompt_ir.schema.json`.
- **Đầu ra**: Trạng thái `PromptIR` bộ nhớ (Đảm bảo 100% bảo toàn vị trí nguồn gốc `INV-01` và toàn bộ ví dụ `INV-09`).

### 📍 Phase 2: Phân Tích Đồ Thị Ngữ Nghĩa (Semantic Graph Analysis)
- **Mục đích**: Xây dựng mạng lưới quan hệ giữa các quy tắc để phát hiện trùng lặp (`same_as`), đối kháng (`conflicts_with`), bao hàm (`subsumes`) hoặc ngoại lệ (`exception_to`).
- **Tài nguyên sử dụng**: `prompts/relation-classifier.md`, `scripts/graph_analyzer.rs`, `schemas/semantic_graph.schema.json`.
- **Đầu ra**: Đồ thị quan hệ `SemanticGraph` (In-Memory).

### 📍 Phase 3: Lập Kế Hoạch Tối Ưu (Optimization Planning)
- **Mục đích**: Phân loại mức độ can thiệp theo khung quyết định (`KEEP`, `LIGHT`, `STRUCTURAL`, `MAJOR`, `EXTERNALIZE`, `MODULARIZE`) và lập danh sách các passes cần chạy.
- **Khung Quyết Định Tiers**:
  | Quyết Định | Điều Kiện Điển Hình | Hành Động Kích Hoạt |
  | :--- | :--- | :--- |
  | **`KEEP`** | Token trong ngân sách, cấu trúc rõ, không dư thừa/xung đột | Không biến đổi; xuất báo cáo audit |
  | **`LIGHT`** | Thuật ngữ trùng, diễn đạt lặp, ngữ nghĩa ổn định | Chuẩn hóa thuật ngữ + Khử trùng ngữ nghĩa an toàn |
  | **`STRUCTURAL`** | Logic tốt nhưng thứ bậc/luồng nhận thức khó hiểu | Sắp xếp lại cấu trúc + Cây quyết định |
  | **`MAJOR`** | Chồng lấn, xung đột chính sách, cấu trúc phức tạp | Đồ thị ngữ nghĩa + Tổng quát hóa (Bảo toàn 100% ví dụ) |
  | **`EXTERNALIZE`** | Khối tri thức lớn/hay đổi, có cơ chế truy xuất | Tách tri thức ra tệp tham chiếu / RAG payload |
  | **`MODULARIZE`** | Prompt rất lớn, nhiều miền nghiệp vụ độc lập | Tách Core Prompt + Modules nạp runtime |
- **Tài nguyên tham chiếu**: `references/optimization-passes.md`.
- **Đầu ra**: Kế hoạch tối ưu `OptimizationPlan` (In-Memory).

### 📍 Phase 4: Thực Thi Chuỗi Optimization Passes (Pass Execution Engine)
- **Mục đích**: Chạy tuần tự các thuật toán biến đổi IR từ mức rủi ro thấp đến cao. Mọi hành động gộp, sửa, xóa đều được ghi vết vào ChangeLog (bảo toàn `INV-02`).
- **Chi tiết các hàm thực thi trong `scripts/passes/`**:
  1. **Chuẩn hóa thuật ngữ**: `normalize_terms::run_pass(ir: PromptIR, dict: Option<HashMap<String, String>>) -> (PromptIR, Vec<ChangeLogEntry>)`
  2. **Khử trùng lặp ngữ nghĩa**: `semantic_dedup::run_pass(ir: PromptIR) -> (PromptIR, Vec<ChangeLogEntry>)`
  3. **Sắp xếp lại cấu trúc**: `reorder_structure::run_pass(ir: PromptIR) -> (PromptIR, Vec<ChangeLogEntry>)`
  4. **Bảo toàn & cấu trúc hóa ví dụ**: `preserve_and_structure_examples::run_pass(ir: PromptIR, config) -> (PromptIR, Vec<ChangeLogEntry>)` (Bảo toàn 100% ví dụ mẫu)
  5. **Tổng quát hóa quy tắc**: `generalization::run_pass(ir: PromptIR)` (kết hợp với `prompts/generalizer.md`, bảo toàn ví dụ)
  6. **Cây quyết định**: `decision_tree::run_pass(ir: PromptIR)`
  7. **Tách mô-đun / tri thức**: `modularization::run_pass` & `externalization::run_pass`
- **Đầu ra**: `OptimizedIR` và `ChangeLog` (In-Memory).

### 📍 Phase 5: Biên Dịch Prompt Ứng Viên (Candidate Compilation)
- **Mục đích**: Render cấu trúc IR tối ưu hóa thành định dạng Prompt Markdown/YAML theo đúng hồ sơ (Profile) của mô hình đích, đảm bảo toàn bộ khối ví dụ được render đầy đủ 100%.
- **Tài nguyên sử dụng**: `references/prompt-profiles.md` (gpt-4o, claude-3-5-sonnet, gemini-1-5-pro, local-llm).
- **Đầu ra**: Chuỗi văn bản Prompt ứng viên hoàn chỉnh.

### 📍 Phase 6: Kiểm Chứng Thích Nghi & Báo Cáo Trực Tiếp Trong Chat (Verification & In-Chat Delivery)
- **Mục đích**: Đánh giá toàn diện chất lượng prompt ứng viên qua các tầng L0–L4, kiểm tra Release Gate và xuất toàn bộ báo cáo Markdown chi tiết ngay trong phản hồi chat.
- **Tiêu chuẩn Release Gate (`PASS`)**:
  1. `unresolved_critical_conflicts == 0`
  2. `required_invariant_coverage == 100%`
  3. `example_preservation_rate == 100%` (Bảo toàn 100% ví dụ gốc)
  4. `critical_regressions == 0`
  5. `output_contract_preserved == true`
  6. `token_budget_ok == true`
- **Quy cách trình bày báo cáo BẮT BUỘC trong Chat**:
  1. **Bảng Đo Lường Tổng Quan (Metrics Table)**: So sánh ký tự/token trước và sau, % giảm dung lượng, trạng thái bảo toàn Invariants và Examples.
  2. **Bảng Chi Tiết Biến Đổi & Lý Do Tối Ưu (Transformation ChangeLog Table - BẮT BUỘC)**:
     Báo cáo tường minh từng câu/khối quy tắc bị thay đổi, lý do và trạng thái biến đổi dưới dạng bảng:
     | Vị trí / Câu gốc trong Prompt | Hành động & Trạng thái | Lý do kỹ thuật & Mã trạng thái | Đoạn tối ưu / Thay thế tương ứng |
     | :--- | :--- | :--- | :--- |
     | *Trích dẫn câu/khối gốc* | `MERGE` / `DELETE_DUPLICATE` / `LIFT_GLOBAL` / `RESTRUCTURE` / `NORMALIZE` / `PRESERVE_100%` | *Lý do: Trùng lặp đa tầng, rải rác, từ đệm rườm rà, bảo toàn INV-xx...* | *Trích dẫn câu/khối sau tối ưu* |
  3. **Khối Mã Có Nút Copy Mặc Định (Fenced Code Block)**: Toàn bộ nội dung Prompt đã biên dịch đặt trong khối mã ````yaml` hoặc ````markdown` để người dùng sao chép nhanh.
  4. **Báo Cáo Kiểm Chứng (Release Gate Verdict)**: Trạng thái nghiệm thu L0-L4 và xác nhận không có hồi quy logic.

---

## 4. Hướng Dẫn Sử Dụng & Kiểm Thử CLI (Cargo Tooling)

Khi cần kiểm thử hoặc chạy các engine tính toán tất định trong môi trường lập trình:
- **Kiểm tra Unit Tests**:
  ```bash
  cargo test
  ```
- **Đo lường chỉ số Token & Độ phức tạp**:
  ```bash
  cargo run --bin prompt_metrics -- --file <path-to-prompt.md>
  ```
- **Chạy kiểm thử hồi quy vi sai (Differential Test)**:
  ```bash
  cargo run --bin regression_runner -- --original <baseline.md> --optimized <candidate.md>
  ```
