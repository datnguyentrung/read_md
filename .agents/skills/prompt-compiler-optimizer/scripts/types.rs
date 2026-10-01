use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// PHASE 1 & 2: DỮ LIỆU ĐẦU VÀO VÀ BIỂU DIỄN TRUNG GIAN (PROMPT IR V4.1)
// ============================================================================

/// Đoạn nguồn trích xuất từ Prompt gốc (Source Span) - Bảo toàn 100% khả năng truy vết (INV-01)
/// Bước: Phân tích cú pháp -> Gắn thẻ vị trí nguồn để kiểm tra bất biến.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceSpan {
    /// Tên mục hoặc phần trong prompt gốc (ví dụ: "system_instructions")
    pub section: String,
    /// Dải dòng trong tệp gốc (ví dụ: "Dòng 1-15")
    #[serde(default)]
    pub line_range: Option<String>,
    /// Đoạn văn bản thô trích xuất trực tiếp từ prompt gốc
    pub text: String,
}

/// Định nghĩa Quy tắc Nguyên tử (AtomicRule) - Đơn vị logic nhỏ nhất của Prompt
/// Thuộc Phase 1 & 2: Phân tách prompt thành các quy tắc độc lập có thể tối ưu hóa.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AtomicRule {
    /// Định danh duy nhất cho quy tắc (ví dụ: "R-001")
    pub id: String,
    /// Loại quy tắc: behavior (hành vi), constraint (ràng buộc), decision (quyết định), knowledge (tri thức), example (ví dụ), output (đầu ra)
    #[serde(rename = "type")]
    pub rule_type: String,
    /// Ngữ nghĩa chuẩn hóa của quy tắc (nội dung câu lệnh đã làm sạch)
    pub semantics: String,
    /// Tác nhân thực thi quy tắc (ví dụ: "Assistant", "User")
    #[serde(default)]
    pub actor: Option<String>,
    /// Hành động governed/prescribed (ví dụ: "respond", "block")
    #[serde(default)]
    pub action: Option<String>,
    /// Đối tượng của hành động (ví dụ: "account_info")
    #[serde(default)]
    pub object: Option<String>,
    /// Điều kiện tiên quyết để quy tắc có hiệu lực
    #[serde(default)]
    pub conditions: Vec<String>,
    /// Ngoại lệ điều chỉnh hoặc ghi đè quy tắc (bảo toàn theo INV-03)
    #[serde(default)]
    pub exceptions: Vec<String>,
    /// Phạm vi hoạt động của quy tắc (ví dụ: ["global"], ["banking_service"])
    pub scope: Vec<String>,
    /// Độ ưu tiên xử lý (0 = Cao nhất/Cốt lõi, 1000 = Thấp nhất)
    pub priority: i32,
    /// Danh sách đoạn nguồn đối chiếu về prompt thô (Bảo đảm INV-01)
    pub source_spans: Vec<SourceSpan>,
    /// Độ tin cậy trích xuất từ mô hình/parser (0.0 -> 1.0)
    #[serde(default)]
    pub confidence: Option<f64>,
    /// Trạng thái vòng đời quy tắc: active (đang chạy), externalized (đã tách ra ngoài), pruned (đã lược bỏ), needs_review (cần duyệt)
    #[serde(default = "default_rule_status")]
    pub status: String,
}

fn default_rule_status() -> String {
    "active".to_string()
}

/// Đơn vị ngữ nghĩa (Semantic Unit) - Khối văn bản được chia theo ý nghĩa
/// Bước: Tiếp nhận text -> Chia thành các đơn vị ý nghĩa trước khi trích quy tắc nguyên tử.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticUnit {
    /// Định danh đơn vị (ví dụ: "u1")
    pub id: String,
    /// Văn bản của đơn vị ngữ nghĩa
    pub text: String,
    /// Vị trí ký tự bắt đầu trong prompt gốc
    pub start: usize,
    /// Vị trí ký tự kết thúc trong prompt gốc
    pub end: usize,
    /// Đường dẫn mục chứa đơn vị ngữ nghĩa này
    #[serde(default)]
    pub section_path: Vec<String>,
    /// Gợi ý vai trò (objective, rules, examples, knowledge, output)
    #[serde(default)]
    pub role_hint: Option<String>,
}

/// Mục cấu trúc của Prompt (Section)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Section {
    pub id: String,
    pub title: String,
    pub order: i32,
    #[serde(default)]
    pub role_hint: Option<String>,
    pub content: String,
}

/// Bất biến cứng (Invariant) - Điều kiện bắt buộc không được làm sai lệch qua mọi pass tối ưu
/// Thuộc Phase 6: Dùng cho Gate verifier kiểm tra độ phủ 100%.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Invariant {
    pub id: String,
    pub description: String,
    pub rule_ref: String,
    #[serde(default = "default_true")]
    pub required: bool,
}

fn default_true() -> bool {
    true
}

/// Đối tượng Cấu trúc IR Trung tâm (PromptIR) - Hợp đồng dữ liệu chính xuyên suốt pipeline
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptIR {
    /// Phiên bản của schema (ví dụ: "4.1.0")
    pub version: String,
    /// Metadata tổng quan (source_hash, title, target_model, token_budget)
    pub metadata: HashMap<String, serde_json::Value>,
    /// Danh sách các section cấu trúc
    pub sections: Vec<Section>,
    /// Danh sách các semantic unit đã phân tách
    #[serde(default)]
    pub semantic_units: Vec<SemanticUnit>,
    /// Tập hợp tất cả atomic rules của prompt
    pub atomic_rules: Vec<AtomicRule>,
    /// Danh sách các bất biến bắt buộc phải bảo toàn
    #[serde(default)]
    pub invariants: Vec<Invariant>,
}

// ============================================================================
// PHASE 3: ĐỒ THỊ NGỮ NGHĨA (SEMANTIC GRAPH)
// ============================================================================

/// Nút trong Đồ thị Ngữ nghĩa
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub category: String,
    #[serde(default)]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Cạnh thể hiện mối quan hệ giữa 2 Quy tắc (Graph Edge)
/// Quan hệ: same_as, subsumes, conflicts_with, depends_on, refines, triggers, example_of, exception_to
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub relation: String,
    #[serde(default)]
    pub weight: Option<f64>,
    #[serde(default)]
    pub reason: Option<String>,
}

/// Đồ thị Ngữ nghĩa hoàn chỉnh dùng để phát hiện trùng lặp và xung đột
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

// ============================================================================
// PHASE 4: KẾ HOẠCH TỐI ƯU VÀ NHẬT KÝ THAY ĐỔI (OPTIMIZATION PLAN & CHANGELOG)
// ============================================================================

/// Đặc tả từng Lượt Tối ưu (PassSpec)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassSpec {
    pub name: String,
    pub enabled: bool,
    pub risk: String, // low, medium, high, operational_high
    #[serde(default)]
    pub order: Option<i32>,
    #[serde(default)]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub rationale: Option<String>,
}

/// Kế hoạch Tối ưu tổng thể (OptimizationPlan)
/// Cấp độ quyết định: KEEP, LIGHT, STRUCTURAL, MAJOR, EXTERNALIZE, MODULARIZE
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OptimizationPlan {
    #[serde(default = "default_plan_status")]
    pub status: String,
    pub decision: String,
    pub passes: Vec<PassSpec>,
    pub hard_invariants: Vec<String>,
    #[serde(default)]
    pub token_target: Option<HashMap<String, serde_json::Value>>,
}

fn default_plan_status() -> String {
    "PLANNED".to_string()
}

/// Nhật ký Thay đổi chi tiết (ChangeLogEntry) - Đảm bảo tính minh bạch và truy vết (INV-02)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChangeLogEntry {
    pub change_id: String,
    pub pass: String,
    pub action: String, // merge, generalize, prune, reorder, externalize, extract_tree
    pub source_rule_ids: Vec<String>,
    pub target_rule_ids: Vec<String>,
    pub reason: String,
    pub risk: String,
    pub before_hash: String,
    pub after_hash: String,
    pub provenance_preserved: bool,
    #[serde(default)]
    pub verification_refs: Vec<String>,
}

// ============================================================================
// PHASE 6: BÁO CÁO KIỂM CHỨNG VÀ GÓI PHÁT HÀNH (VERIFICATION & RELEASE)
// ============================================================================

/// Báo cáo Kiểm chứng Thích nghi (VerificationReport)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationReport {
    pub timestamp: String,
    pub status: String, // RELEASED, NEEDS_REVIEW, UNVERIFIED, ROLLED_BACK
    pub assurance_level: String, // VERIFIED_STATIC, VERIFIED_OFFLINE, VERIFIED_CONTRACT, VERIFIED_MOCK, VERIFIED_E2E
    pub verification_scope: String,
    #[serde(default)]
    pub verification_debt: Vec<String>,
    pub static_gate: String, // PASS, FAIL, NEEDS_REVIEW
    pub behavior_gate: String, // PASS, FAIL, SKIPPED, NEEDS_REVIEW
    pub metrics_delta: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub invariants: Vec<serde_json::Value>,
    #[serde(default)]
    pub test_results: Vec<serde_json::Value>,
}

/// Gói Phát hành Hoàn chỉnh (ReleaseBundle) - Kết quả cuối cùng của toàn bộ pipeline
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReleaseBundle {
    pub status: String,
    pub decision: String,
    pub assurance_level: String,
    pub verification_scope: String,
    #[serde(default)]
    pub verification_debt: Vec<String>,
    pub source: HashMap<String, String>,
    pub optimizer: HashMap<String, String>,
    pub analysis: HashMap<String, String>,
    pub candidate: HashMap<String, String>,
    pub metrics: HashMap<String, serde_json::Value>,
    pub verification: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub rollback: HashMap<String, String>,
}
