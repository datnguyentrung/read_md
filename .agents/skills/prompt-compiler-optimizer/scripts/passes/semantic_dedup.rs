//! Pass 2: Khử Trùng Ngữ Nghĩa (Semantic Deduplication Pass)
//! 
//! THUỘC PHASE 4: TỐI ƯU HÓA RỦI RO THẤP (LOW-RISK OPTIMIZATION PASS)
//! ========================================================================================
//! MỤC ĐÍCH KĨ THUẬT & BẢO TOÀN BẤT BIẾN:
//! 1. Khử trùng lặp nội dung: Phát hiện và loại bỏ các quy tắc nguyên tử có cùng ý nghĩa thực thi.
//! 2. BẢO TOÀN INV-01 (Hợp nhất Provenance): Khi quy tắc B bị gộp vào quy tắc A, toàn bộ `source_spans`
//!    (đoạn văn bản gốc) của B sẽ được cộng nối (extend) vào A. Không được làm mất vết nguồn gốc!
//! 3. BẢO TOÀN INV-02 (Nhật ký thay đổi): Mọi hành động gộp quy tắc (`action: "merge"`) được ghi nhận chi tiết vào `changes.json`.

use std::collections::HashSet;
use crate::types::{AtomicRule, ChangeLogEntry, PromptIR};

/// Thực thi lượt tối ưu khử trùng lặp ngữ nghĩa trên đối tượng `PromptIR`.
/// 
/// # Thuật toán & Quy trình xử lý
/// 1. Khởi tạo `HashSet` để lưu các chuỗi ngữ nghĩa đã được chuẩn hóa (loại bỏ khoảng trắng thừa, hạ chữ thường).
/// 2. Khởi tạo danh sách `deduped_rules` chứa các quy tắc giữ lại và `changes` để lưu nhật ký.
/// 3. Duyệt tuần tự qua từng `AtomicRule` trong `ir.atomic_rules`:
///    - Chuẩn hóa chuỗi ngữ nghĩa: `rule.semantics.trim().to_lowercase()`.
///    - Nếu chưa có trong `HashSet`: Thêm vào `seen` và đẩy `rule` vào `deduped_rules`.
///    - Nếu ĐÃ TỒN TẠI (trùng lặp):
///      + Tìm quy tắc gốc tương ứng trong `deduped_rules`.
///      + Hợp nhất danh sách `source_spans` của quy tắc trùng vào quy tắc gốc (`existing.source_spans.extend(...)`).
///      + Ghi vết thay đổi vào `ChangeLogEntry` với lý do gộp quy tắc và cờ `provenance_preserved: true`.
/// 4. Cập nhật `ir.atomic_rules = deduped_rules`.
/// 5. Trả về `(ir, changes)`.
pub fn run_pass(mut ir: PromptIR) -> (PromptIR, Vec<ChangeLogEntry>) {
    // Tập hợp HashSet dùng để tra cứu sự trùng lặp với độ phức tạp O(1)
    let mut seen = HashSet::new();
    let mut deduped_rules: Vec<AtomicRule> = Vec::new();
    let mut changes = Vec::new();

    // [Bước 1]: Duyệt qua từng quy tắc nguyên tử trong danh sách ban đầu
    for rule in ir.atomic_rules {
        // Chuẩn hóa chuỗi bằng cách cắt bớt khoảng trắng 2 đầu và đưa về chữ thường để so sánh chính xác
        let normalized = rule.semantics.trim().to_lowercase();

        // [Trường hợp 1]: Quy tắc hoàn toàn mới -> Thêm vào danh sách lọc
        if !seen.contains(&normalized) {
            seen.insert(normalized);
            deduped_rules.push(rule);
        } else {
            // [Trường hợp 2]: Quy tắc bị trùng lặp ngữ nghĩa -> Hợp nhất Vết Nguồn Gốc (INV-01)
            if let Some(existing) = deduped_rules.iter_mut().find(|r| r.semantics.trim().to_lowercase() == normalized) {
                // HỢP NHẤT VẾT NGUỒN (INV-01): Đảm bảo các đoạn text gốc từ cả 2 vị trí trùng lặp đều được giữ lại
                existing.source_spans.extend(rule.source_spans.clone());

                // GHI VẾT NHẬT KÝ (INV-02): Ghi nhận hành động merge hai ID quy tắc thành một quy tắc duy nhất
                changes.push(ChangeLogEntry {
                    change_id: format!("CH-DEDUP-{}", rule.id),
                    pass: "semantic_dedup".to_string(),
                    action: "merge".to_string(),
                    source_rule_ids: vec![existing.id.clone(), rule.id.clone()],
                    target_rule_ids: vec![existing.id.clone()],
                    reason: "Phát hiện nội dung trùng lặp ngữ nghĩa; tiến hành gộp quy tắc và hợp nhất vị trí nguồn gốc (Bảo toàn INV-01)".to_string(),
                    risk: "low".to_string(),
                    before_hash: format!("{:x}", rule.semantics.len()),
                    after_hash: format!("{:x}", existing.semantics.len()),
                    provenance_preserved: true,
                    verification_refs: vec![],
                });
            }
        }
    }

    // [Bước 2]: Gán lại danh sách quy tắc đã khử trùng vào đối tượng IR
    ir.atomic_rules = deduped_rules;
    (ir, changes)
}
