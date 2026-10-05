# Tài Liệu Các Lượt Tối Ưu Hóa (Optimization Passes V4.1)

## 1. Phân Hạng Rủi Ro & Chính Sách Mặc Định

| Lượt Tối Ưu | Tên Hàm | Mức Rủi Ro | Chính Sách & Điều Kiện Kích Hoạt |
| :--- | :--- | :--- | :--- |
| **Pass 1** | `normalize_terms` | **THẤP** | Tự động chạy khi có bảng từ đồng nghĩa; hoàn tác được. |
| **Pass 2** | `semantic_dedup` | **THẤP → TRUNG BÌNH** | Tự động chạy khi quan hệ `same_as` có độ tin cậy cao và cùng phạm vi/ngoại lệ; gộp `source_spans`. |
| **Pass 3** | `reorder_structure` | **THẤP** | Tự động sắp xếp quy tắc theo điểm ưu tiên `priority` và luồng nhận thức. |
| **Pass 4** | `preserve_and_structure_examples` | **THẤP** | Bảo toàn 100% toàn bộ ví dụ gốc (Zero Loss); gom nhóm và chuẩn hóa định dạng ví dụ (few-shot, JSON payload, dialogue sample) vào đúng phân mục rõ ràng, cấm xóa bỏ. |
| **Pass 5** | `generalization` | **CAO** | Rút bất biến chung từ nhiều quy tắc tương đồng (bảo toàn ví dụ đi kèm); bắt buộc chạy bộ kiểm thử chuẩn, rollback nếu ranh giới thay đổi. |
| **Pass 6** | `decision_tree` | **TRUNG BÌNH → CAO** | Chuyển logic if/else phân nhánh thành cây quyết định hoặc bảng tra cứu; bắt buộc kiểm tra thứ tự ưu tiên. |
| **Pass 7A** | `externalization` | **CAO (Vận Hành)** | Đề xuất tách tri thức lớn/hay đổi sang tài liệu RAG/tham chiếu ngoài; chỉ bật khi có cơ chế truy xuất khi chạy. |
| **Pass 7B** | `modularization` | **CAO (Vận Hành)** | Tách monolithic prompt thành phần Core (luôn nạp) + Domain Modules (nạp theo ý định khi chạy). |

## 2. Cơ Chế Snapshot & Rollback
Mỗi pass chạy như một giao dịch (transaction):
1. Chụp ảnh trạng thái (`Snapshot`).
2. Thực hiện biến đổi trên IR.
3. Kiểm tra điều kiện sau (`Post-conditions` & Invariants).
4. Nếu đạt: ghi nhận vào `ChangeLog` và chuyển tiếp; Nếu vi phạm: tự động hoàn tác (`Rollback`).
