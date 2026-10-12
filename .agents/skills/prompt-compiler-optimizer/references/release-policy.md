# Chính Sách & Cổng Phát Hành (Release Policy V5.0)

## 1. Cổng Kiểm Duyệt Phát Hành (Release Gate)

Bản ứng viên được phát hành (**`RELEASED`**) khi và chỉ khi:

```text
unresolved_critical_conflicts == 0       # 0 xung đột nghiêm trọng
AND required_invariant_coverage == 100%  # 100% bất biến bắt buộc được bảo toàn (INV-01 đến INV-09)
AND example_preservation_rate == 100%    # 100% ví dụ minh họa gốc được bảo toàn đầy đủ
AND critical_regressions == 0            # 0 lỗi hồi quy nghiêm trọng
AND output_contract_preserved == true    # Hợp đồng đầu ra giữ nguyên vẹn
AND token_budget_ok == true              # Không vượt ngân sách token
AND assurance_level >= min_assurance     # Production mặc định: VERIFIED_DIFFERENTIAL
```

> [!CAUTION]
> **Nguyên tắc bất di bất dịch**: Giảm token KHÔNG BAO GIỜ được dùng để bỏ qua hoặc thỏa hiệp một cổng kiểm chứng thất bại.

## 2. Trạng Thái Độc Lập
- `Execution Status`: `PLANNED`, `APPLIED`, `PARTIAL`, `BLOCKED`, `FAILED`, `COMPLETED`.
- `Verification Status`: `UNVERIFIED`, `PARTIALLY_VERIFIED`, `VERIFIED`, `FAILED`.
- `Assurance Level`: `UNVERIFIED`, `VERIFIED_STATIC`, `VERIFIED_DIFFERENTIAL`, `VERIFIED_CONTRACT`, `VERIFIED_MOCK`, `VERIFIED_E2E`.
- `Release Decision`: `RELEASED`, `NEEDS_REVIEW`, `BLOCKED_INSUFFICIENT_ASSURANCE`, `BLOCKED_UNSUPPORTED`, `FAILED`, `OVERRIDE_RELEASED`.

Static PASS chỉ chứng minh phạm vi L0. Tầng chưa chạy phải là `NOT_EXECUTED`, không được ghi PASS. Manual override chỉ được vượt thiếu assurance, phải có approver, reason, accepted risks và timestamp; không được vượt `FAILED` hoặc `BLOCKED_UNSUPPORTED`.

## 3. Quản Lý Nợ Kiểm Chứng (Verification Debt)
Mỗi báo cáo phát hành phải công khai:
- `assurance_level`: Mức bằng chứng thực tế đạt được (`VERIFIED_STATIC`, `VERIFIED_DIFFERENTIAL`, `VERIFIED_CONTRACT`, `VERIFIED_MOCK`, `VERIFIED_E2E`).
- `verification_scope`: Phạm vi đã được chứng minh.
- `verification_debt`: Danh sách các khía cạnh chưa thể kiểm chứng do thiếu quyền truy cập hệ thống thật.
