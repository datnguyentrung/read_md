# Đặc Tả Biểu Diễn Trung Gian (Prompt IR V4.1 Specification)

## 1. Tổng Quan
Prompt IR là mô hình dữ liệu chuẩn hóa (AST-like) đại diện cho toàn bộ ngữ nghĩa, quy tắc và cấu trúc của prompt nguồn, tách biệt logic nghiệp vụ khỏi phong cách diễn đạt câu chữ ngẫu nhiên.

## 2. Các Bất Biến Bắt Buộc (Mandatory Invariants)
- **INV-01 (Đầy đủ nguồn gốc - Provenance Completeness)**: Mọi quy tắc đang hoạt động hoặc đã tách ra ngoài phải có thông tin `source_spans` liên kết về đoạn văn bản gốc.
- **INV-02 (Không xóa im lặng - No Silent Deletions)**: Quy tắc chỉ biến mất nếu ChangeLog ghi rõ hành động (gộp, lược bỏ, tách ngoài) kèm lý do.
- **INV-03 (Bảo toàn ngoại lệ - Preserve Exceptions)**: Mối quan hệ ngoại lệ phải còn tương đương sau tối ưu hoặc được mã hóa rõ trong cây quyết định.
- **INV-04 (Bảo toàn hợp đồng đầu ra - Preserve Output Contract)**: Nhãn, schema JSON, tool name, required fields không được thay đổi.
- **INV-05 (Khép kín xung đột - Conflict Closure)**: Không phát hành nếu còn xung đột mức nghiêm trọng chưa rõ thứ tự ưu tiên (`NEEDS_REVIEW`).
- **INV-06 (Phát hành tất định - Deterministic Release)**: Cổng phát hành dựa trên kết quả kiểm tra tất định, không dựa vào một điểm số LLM đơn lẻ.
- **INV-07 (Toàn vẹn nguồn - Source Integrity)**: Lưu `source_hash` của prompt gốc để phát hiện thay đổi.
- **INV-08 (Tái lập được - Reproducibility)**: Lưu cấu hình, phiên bản optimizer, target profile và thứ tự passes.
- **INV-09 (Bảo toàn 100% Ví dụ - Example Completeness)**: Mọi quy tắc, mẫu dữ liệu, JSON payload, few-shot hoặc template mang bản chất là ví dụ (`type: example` hoặc section role `examples`) từ prompt nguồn BẮT BUỘC phải xuất hiện đầy đủ 100% trong prompt đích, tuyệt đối không được lược bỏ hay cắt bớt.

## 3. Cấu Trúc Lõi Của AtomicRule
```json
{
  "id": "R-001",
  "type": "behavior | constraint | decision | knowledge | example | output",
  "semantics": "Ngữ nghĩa chuẩn hóa của quy tắc",
  "actor": "Assistant",
  "action": "respond",
  "object": "account_info",
  "conditions": ["user_authenticated == true"],
  "exceptions": ["user_role == 'restricted'"],
  "scope": ["global", "banking_service"],
  "priority": 100,
  "source_spans": [
    {
      "section": "quy_dinh_chung",
      "line_range": "12-15",
      "text": "Đoạn văn bản gốc trích từ prompt..."
    }
  ],
  "confidence": 0.98,
  "status": "active"
}
```
