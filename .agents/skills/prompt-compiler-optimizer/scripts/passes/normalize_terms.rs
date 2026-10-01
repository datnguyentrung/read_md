//! Pass 1: Chuẩn Hóa Thuật Ngữ (Normalize Terms Pass)
//! 
//! THUỘC PHASE 4: TỐI ƯU HÓA RỦI RO THẤP (LOW-RISK OPTIMIZATION PASS)
//! ========================================================================================
//! MỤC ĐÍCH KĨ THUẬT:
//! 1. Loại bỏ biến thể từ ngữ: Thay thế các biệt ngữ, từ viết tắt, từ đồng nghĩa ngẫu nhiên trong prompt 
//!    thành từ vựng chuẩn hóa duy nhất (Canonical Vocabulary).
//! 2. Tăng độ tập trung cho mô hình AI: Giúp LLM dễ dàng chú ý vào các khái niệm chính mà không bị phân tâm bởi các từ đồng nghĩa.
//! 3. Đảm bảo bất biến INV-02: Mọi thao tác chuẩn hóa từ ngữ phải được ghi nhận vết vào `ChangeLogEntry` với mức rủi ro `low`.

use std::collections::HashMap;
use crate::types::{ChangeLogEntry, PromptIR};

/// Thực thi lượt tối ưu chuẩn hóa thuật ngữ trên đối tượng `PromptIR`.
/// 
/// # Tham số
/// * `ir`: Đối tượng `PromptIR` đầu vào chứa tập hợp các quy tắc nguyên tử.
/// * `synonyms`: Bảng tra cứu từ đồng nghĩa dạng `HashMap<Từ_Cũ, Từ_Mới>` (Tùy chọn).
/// 
/// # Thuật toán & Quy trình xử lý
/// 1. Khởi tạo danh sách theo dõi lịch sử biến đổi `changes`.
/// 2. Nếu `synonyms` được truyền vào, duyệt qua từng quy tắc `AtomicRule` trong `ir.atomic_rules`.
/// 3. Lưu lại nội dung `semantics` ban đầu làm mốc so sánh (`orig_semantics`).
/// 4. Thực hiện thay thế tất cả xuất hiện của các khóa từ cũ `k` bằng từ chuẩn `v`.
/// 5. Nếu chuỗi `updated` khác với `orig_semantics`:
///    - Gán chuỗi mới vào `rule.semantics`.
///    - Tạo một bản ghi `ChangeLogEntry` ghi rõ `change_id`, `action: "normalize"`, `risk: "low"`, 
///      mã băm trước/sau và cờ `provenance_preserved: true`.
/// 6. Trả về cặp `(PromptIR_đã_cập_nật, Danh_sách_ChangeLogEntry)`.
pub fn run_pass(mut ir: PromptIR, synonyms: Option<&HashMap<String, String>>) -> (PromptIR, Vec<ChangeLogEntry>) {
    let mut changes = Vec::new();

    // [Bước 1]: Kiểm tra sự tồn tại của bảng tra cứu từ đồng nghĩa
    if let Some(synonym_map) = synonyms {
        // [Bước 2]: Duyệt mutable iterator qua từng quy tắc nguyên tử để chỉnh sửa trực tiếp (In-place mutation)
        for rule in &mut ir.atomic_rules {
            let orig_semantics = rule.semantics.clone();
            let mut updated = orig_semantics.clone();

            // [Bước 3]: Duyệt qua từng cặp từ đồng nghĩa và tiến hành thay thế chuỗi
            for (k, v) in synonym_map {
                updated = updated.replace(k, v);
            }

            // [Bước 4]: Phát hiện sự thay đổi nội dung ngữ nghĩa
            if updated != orig_semantics {
                rule.semantics = updated.clone();

                // [Bước 5]: Đóng gói vết thay đổi vào ChangeLogEntry (INV-02 compliance)
                changes.push(ChangeLogEntry {
                    change_id: format!("CH-NORM-{}", rule.id),
                    pass: "normalize_terms".to_string(),
                    action: "normalize".to_string(),
                    source_rule_ids: vec![rule.id.clone()],
                    target_rule_ids: vec![rule.id.clone()],
                    reason: "Chuẩn hóa thuật ngữ đồng nghĩa về dạng từ vựng chuẩn (Canonical Vocabulary)".to_string(),
                    risk: "low".to_string(),
                    // Sử dụng độ dài chuỗi làm checksum băm đơn giản cho trước/sau biến đổi
                    before_hash: format!("{:x}", orig_semantics.len()),
                    after_hash: format!("{:x}", updated.len()),
                    provenance_preserved: true,
                    verification_refs: vec![],
                });
            }
        }
    }

    // [Bước 6]: Trả về IR mới cùng với danh sách nhật ký thay đổi
    (ir, changes)
}
