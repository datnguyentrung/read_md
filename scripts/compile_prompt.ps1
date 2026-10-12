param(
    [Parameter(Mandatory = $true)]
    [string]$InputPath,

    [Parameter(Mandatory = $true)]
    [string]$OutputPath
)

$text = [System.IO.File]::ReadAllText($InputPath, [System.Text.Encoding]::UTF8)

$precedence = @'
    <decision_precedence priority="CRITICAL">
        Áp dụng theo thứ tự: Bảo mật & An toàn > Data Provenance/Data Fidelity > quy tắc đặc thù nghiệp vụ > schema UI dùng chung > Mode > Style.
        1. Quy tắc cụ thể là ngoại lệ của quy tắc tổng quát; không được làm mất ngoại lệ khi chọn Mode hoặc component.
        2. Truy vấn nhanh một thuộc tính/thông số đơn lẻ luôn dùng text, kể cả khi sau lọc chỉ còn một thẻ. Quy tắc "một thẻ dùng A2UICardDetail" chỉ áp dụng cho danh sách/chi tiết hoặc hành vi nghiệp vụ cần UI.
        3. Với khóa/mở khóa thẻ, action event `card_action` (`LOCK`/`UNLOCK`) là ngoại lệ đặc thù của quy tắc "Detail không có action". Ngoài ngoại lệ này, component Detail không có action.
        4. Trong `A2UICardList`, "chỉ chứa identifier.cardNo" nghĩa là không có field mô tả khác; item vẫn phải có `action` mang Carried Intent.
        5. Schema dùng chung ở phần CRITICAL RULES là chuẩn cuối: output tối giản không có `surfaceId`; `A2UISuggestionGroup` dùng `suggestions` và đứng trong thẻ `<a2ui-json>` độc lập cuối phản hồi. Các mẫu cũ có `surfaceId`/`data` được giữ nguyên để truy xuất nguồn nhưng là legacy, không dùng để render.
        6. Khi `overall_state = "ALL_EMPTY"`, phản hồi gồm thông báo thiếu dữ liệu và 2-3 gợi ý chuyển hướng, sau đó dừng; không bổ sung nội dung nghiệp vụ khác.
    </decision_precedence>

'@

$text = $text.Replace("<system_prompt>`n", "<system_prompt>`n$precedence")

# Remove the duplicated integration guide outside response_style. Its unique
# requirements are already covered by decision_precedence and the mode rules.
$text = [regex]::Replace(
    $text,
    '(?s)(</response_style>\s*)<style_integration_guide>.*?</style_integration_guide>\s*',
    '$1'
)

$text = $text.Replace('<opening_rules>', '<opening_pattern>')
$text = $text.Replace('<closing_rules>', '<closing_pattern>')
$text = $text.Replace('[style_block]', '<response_style name="default">')
$text = $text.Replace('Trình tự 3 bước:', 'Trình tự 4 phần:')

$directory = Split-Path -Parent $OutputPath
if ($directory) {
    [System.IO.Directory]::CreateDirectory($directory) | Out-Null
}
[System.IO.File]::WriteAllText($OutputPath, $text, [System.Text.UTF8Encoding]::new($false))
