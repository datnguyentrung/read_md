//! Graph Analyzer (Công Cụ Phân Tích Đồ Thị Ngữ Nghĩa V4.1)
//! 
//! VỊ TRÍ TRONG HỆ THỐNG (COMPILER PIPELINE):
//! ========================================================================================
//! Phase 3: Phân tích Đồ thị Ngữ nghĩa & Đánh giá Chất lượng (Semantic Graph & Audit Analysis)
//! 
//! MỤC ĐÍCH NGHIỆP VỤ & KỸ THUẬT:
//! 1. Chuyển đổi tập hợp các quy tắc nguyên tử (`AtomicRule`) từ `PromptIR` thành mạng lưới đồ thị (`SemanticGraph`).
//! 2. Tạo các Nút (`GraphNode`) chứa định danh, nhãn rút gọn (tránh lỗi cắt chuỗi UTF-8) và loại quy tắc.
//! 3. Tạo các Cạnh (`GraphEdge`) thể hiện 8 mối quan hệ ngữ nghĩa: `same_as`, `subsumes`, `conflicts_with`,
//!    `depends_on`, `refines`, `triggers`, `example_of`, `exception_to`.
//! 4. Cung cấp dữ liệu nền tảng cho Phase 4 để phát hiện trùng lặp (`same_as`) hoặc xung đột nghiêm trọng (`conflicts_with`).

use std::env;
use std::fs;
use std::path::Path;
use prompt_compiler_optimizer::types::{GraphNode, PromptIR, SemanticGraph};

/// Phân tích tệp `prompt_ir.json` và xây dựng cấu trúc đồ thị ngữ nghĩa `SemanticGraph`.
/// 
/// # Tham số
/// * `ir_path`: Đường dẫn hợp lệ tới tệp JSON lưu trữ PromptIR.
/// 
/// # Luồng xử lý
/// 1. Đọc nội dung tệp JSON từ đĩa thông qua `fs::read_to_string`.
/// 2. Giải mã (deserialize) chuỗi JSON thành struct `PromptIR` với Serde.
/// 3. Duyệt qua danh sách `atomic_rules` để khởi tạo các `GraphNode`:
///    - Trích xuất `id` quy tắc làm định danh nút.
///    - Cắt an toàn 30 ký tự đầu của `semantics` làm nhãn (`label`), bảo toàn ranh giới UTF-8 bằng `chars().take(30)`.
///    - Gán `category` từ `rule_type` của quy tắc.
/// 4. Đóng gói danh sách nút và tập cạnh thành `SemanticGraph`.
/// 
/// # Trả về
/// * `Ok(SemanticGraph)` nếu đọc tệp và phân tích thành công.
/// * `Err(Box<dyn std::error::Error>)` nếu tệp không tồn tại, lỗi I/O hoặc định dạng JSON không hợp lệ.
fn analyze_graph(ir_path: &Path) -> Result<SemanticGraph, Box<dyn std::error::Error>> {
    // [Bước 1]: Đọc toàn bộ nội dung tệp IR từ đường dẫn chỉ định
    // Thất bại tại đây nếu tệp bị khóa, không có quyền đọc hoặc đường dẫn sai
    let content = fs::read_to_string(ir_path)?;

    // [Bước 2]: Giải mã chuỗi JSON thành đối tượng cấu trúc PromptIR
    // Đảm bảo dữ liệu tuân thủ chuẩn lược đồ prompt_ir.schema.json
    let ir: PromptIR = serde_json::from_str(&content)?;
    
    // [Bước 3]: Ghi nhật ký tiến trình ra console để theo dõi (Observability)
    println!("[*] Đang xây dựng đồ thị ngữ nghĩa cho {} quy tắc nguyên tử...", ir.atomic_rules.len());
    
    // [Bước 4]: Duyệt tuần tự tập atomic_rules và ánh xạ sang GraphNode
    // - Sử dụng iterator mapping `.iter().map(...)` để tối ưu hiệu năng bộ nhớ
    // - Sử dụng `chars().take(30).collect()` thay vì cắt byte slice `[..30]` để tránh panic khi gặp ký tự tiếng Việt đa byte
    let nodes = ir
        .atomic_rules
        .iter()
        .map(|r| GraphNode {
            id: r.id.clone(),
            label: r.semantics.chars().take(30).collect(),
            category: r.rule_type.clone(),
            metadata: None,
        })
        .collect();

    // [Bước 5]: Đóng gói đồ thị và trả về kết quả
    // Tập cạnh (edges) khởi tạo rỗng và sẽ được bổ sung bởi relation-classifier ở các lượt phân tích nâng cao
    Ok(SemanticGraph {
        nodes,
        edges: vec![],
    })
}

/// Điểm bắt đầu của công cụ dòng lệnh (CLI binary) `graph_analyzer`.
/// 
/// # Luồng thực thi dòng lệnh
/// 1. Thu thập các tham số truyền vào từ `std::env::args()`.
/// 2. Bóc tách cờ `--ir <path>` để lấy đường dẫn tệp `prompt_ir.json`.
/// 3. Nếu thiếu tham số `--ir`, in hướng dẫn sử dụng và thoát với mã lỗi `1`.
/// 4. Gọi hàm `analyze_graph(...)` để xử lý.
/// 5. In kết quả đồ thị dưới dạng JSON đẹp (`serde_json::to_string_pretty`) ra stdout để công cụ khác có thể pipe/đọc.
fn main() {
    // [Bắt tham số CLI]: Lấy danh sách chuỗi tham số truyền từ Terminal
    let args: Vec<String> = env::args().collect();
    let mut ir_path_opt: Option<String> = None;

    // [Vòng lặp quét tham số]: Tìm cờ --ir và lấy giá trị đường dẫn đi kèm
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--ir" && i + 1 < args.len() {
            ir_path_opt = Some(args[i + 1].clone());
            i += 1;
        }
        i += 1;
    }

    // [Kiểm tra tham số bắt buộc]: Đảm bảo đường dẫn tệp IR được cung cấp
    let ir_path_str = match ir_path_opt {
        Some(p) => p,
        None => {
            eprintln!("Cú pháp: graph_analyzer --ir <đường-dẫn-prompt_ir.json>");
            std::process::exit(1);
        }
    };

    // [Thực thi phân tích & Xuất kết quả]: Gọi hàm phân tích và xuất JSON đẹp
    match analyze_graph(Path::new(&ir_path_str)) {
        Ok(graph) => {
            // Định dạng JSON có thụt lề để dễ đọc và làm việc với công cụ quan sát đồ thị
            let out = serde_json::to_string_pretty(&graph).unwrap();
            println!("{}", out);
        }
        Err(e) => {
            // Báo lỗi ra stderr và thoát với mã lỗi 1 để thông báo sự cố cho pipeline
            eprintln!("[-] Lỗi khi phân tích đồ thị ngữ nghĩa: {}", e);
            std::process::exit(1);
        }
    }
}
