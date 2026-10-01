# Khung Kiểm Chứng Thích Nghi (Adaptive Verification V4.1)

## 1. Hai Câu Hỏi Bắt Buộc Trước Khi Chạy Test
1. **Prompt hiện tại chịu trách nhiệm tạo ra loại output/quyết định nào?** (Router/Tool calling, Classifier, Extraction, Free-text).
2. **Ta đang có quyền truy cập đến mức nào:** model callable, schema tool, mock dependency, hay hệ thống E2E thật?

## 2. Các Tầng Kiểm Chứng (L0–L4)

| Tầng | Tên Gọi | Quyền Truy Cập Yêu Cầu | Phạm Vi Kiểm Chứng |
| :--- | :--- | :--- | :--- |
| **L0** | **Static Verification** | Không cần gọi model | Lược đồ IR, 100% độ phủ bất biến, xung đột, hợp đồng đầu ra. |
| **L1** | **Differential / Offline** | Model callable | So sánh hành vi giữa baseline và candidate trên cùng test inputs. |
| **L2** | **Contract Verification** | Tool schemas / JSON spec | Kiểm tra tool names, parameter schemas, enum, required fields mà không execute tool thật. |
| **L3** | **Mock Replay** | Mock runner | Kiểm tra chuỗi hội thoại / agent multi-turn với mock responses. |
| **L4** | **End-to-End** | Hệ thống tích hợp thật | Kiểm thử toàn diện trên staging / dev cluster. |

## 3. Phân Loại Ca Kiểm Thử (Test Taxonomy)
- **Golden**: Ca thực tế chuẩn đã được duyệt.
- **Representative**: Các ý định phổ biến nhất.
- **Boundary**: Ca sát ranh giới giữa hai quyết định (chống over-generalization).
- **Exception**: Ca kích hoạt ngoại lệ hoặc quy tắc ghi đè.
- **Adversarial Paraphrase**: Đổi cách diễn đạt mạnh nhưng giữ nguyên ý định.
- **Metamorphic**: Biến đổi thứ tự từ hoặc thêm thông tin rác không liên quan.
- **Output-Contract**: Kiểm tra tính hợp lệ của schema/format đầu ra.
