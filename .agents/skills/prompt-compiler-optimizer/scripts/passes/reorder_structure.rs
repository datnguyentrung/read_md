//! Pass 3: Sắp Xếp Cấu Trúc Bố Cục (Reorder Structure Pass)
//!
//! THUỘC PHASE 4: TỐI ƯU HÓA BỐ CỤC (STRUCTURAL REORDER PASS)
//! ========================================================================================
//! Tác dụng: Sắp xếp lại thứ tự các quy tắc theo điểm số ưu tiên `priority` (0 = Cao nhất/Cốt lõi).
//!           Giúp mô hình AI chú ý vào các quy tắc quan trọng nhất trước (Primacy Effect).
//! Đầu vào:  Đối tượng `PromptIR`.
//! Đầu ra:   `PromptIR` đã sắp xếp lại và danh sách `ChangeLogEntry`.

use crate::types::{ChangeLogEntry, PromptIR};

/// Hàm `run_pass`: Thực thi lượt sắp xếp lại thứ tự ưu tiên của quy tắc
pub fn run_pass(mut ir: PromptIR) -> (PromptIR, Vec<ChangeLogEntry>) {
    let before: Vec<String> = ir.atomic_rules.iter().map(|r| r.id.clone()).collect();
    // Sắp xếp danh sách atomic_rules tăng dần theo giá trị `priority` (giá trị càng nhỏ ưu tiên càng cao)
    ir.atomic_rules.sort_by_key(|r| r.priority);

    let after: Vec<String> = ir.atomic_rules.iter().map(|r| r.id.clone()).collect();
    if before == after {
        return (ir, vec![]);
    }
    let changes = vec![ChangeLogEntry {
        change_id: "CH-REORDER-01".to_string(),
        pass: "reorder_structure".to_string(),
        action: "reorder".to_string(),
        source_rule_ids: ir.atomic_rules.iter().map(|r| r.id.clone()).collect(),
        target_rule_ids: ir.atomic_rules.iter().map(|r| r.id.clone()).collect(),
        reason: "Sắp xếp lại thứ tự quy tắc theo độ ưu tiên giảm dần (Priority Score)".to_string(),
        risk: "low".to_string(),
        before_hash: crate::pipeline::sha256_hex(before.join("|").as_bytes()),
        after_hash: crate::pipeline::sha256_hex(after.join("|").as_bytes()),
        provenance_preserved: true,
        verification_refs: vec![],
    }];

    (ir, changes)
}
