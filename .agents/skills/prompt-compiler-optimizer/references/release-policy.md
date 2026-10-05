# Chính Sách & Cổng Phát Hành (Release Policy V4.1)

## 1. Cổng Kiểm Duyệt Phát Hành (Release Gate)

Bản ứng viên được phát hành (**`RELEASED`**) khi và chỉ khi:

```text
unresolved_critical_conflicts == 0       # 0 xung đột nghiêm trọng
AND required_invariant_coverage == 100%  # 100% bất biến bắt buộc được bảo toàn (INV-01 đến INV-09)
AND example_preservation_rate == 100%    # 100% ví dụ minh họa gốc được bảo toàn đầy đủ
AND critical_regressions == 0            # 0 lỗi hồi quy nghiêm trọng
AND output_contract_preserved == true    # Hợp đồng đầu ra giữ nguyên vẹn
AND token_budget_ok == true              # Không vượt ngân sách token
```

> [!CAUTION]
> **Nguyên tắc bất di bất dịch**: Giảm token KHÔNG BAO GIỜ được dùng để bỏ qua hoặc thỏa hiệp một cổng kiểm chứng thất bại.

## 2. Quản Lý Nợ Kiểm Chứng (Verification Debt)
Mỗi báo cáo phát hành phải công khai:
- `assurance_level`: Mức bằng chứng thực tế đạt được (`VERIFIED_STATIC`, `VERIFIED_OFFLINE`, `VERIFIED_CONTRACT`, `VERIFIED_MOCK`, `VERIFIED_E2E`).
- `verification_scope`: Phạm vi đã được chứng minh.
- `verification_debt`: Danh sách các khía cạnh chưa thể kiểm chứng do thiếu quyền truy cập hệ thống thật.
