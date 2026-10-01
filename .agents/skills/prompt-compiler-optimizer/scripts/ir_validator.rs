//! IR Validator (Công Cụ Kiểm Tra Chuẩn IR V4.1 & Bất Biến Lĩnh Vực)
//! 
//! THUỘC PHASE 6: KIỂM CHỨNG TĨNH (STATIC VERIFICATION L0)
//! ========================================================================================
//! Tác dụng: Kiểm tra tính hợp lệ của tệp JSON PromptIR theo đúng JSON Schema v4.1.
//!           Đặc biệt kiểm tra Bất biến INV-01: Mọi quy tắc đang hoạt động (`status == "active"`)
//!           bắt buộc phải có `source_spans` liên kết về đoạn prompt gốc.

use std::env;
use std::fs;
use std::path::Path;
use prompt_compiler_optimizer::types::PromptIR;

/// Hàm `validate_ir`: Thực thi đọc tệp JSON IR và kiểm tra các điều kiện bất biến tĩnh
fn validate_ir(ir_path: &Path) -> bool {
    match fs::read_to_string(ir_path) {
        Ok(content) => match serde_json::from_str::<PromptIR>(&content) {
            Ok(ir) => {
                // Kiểm tra INV-01: Mọi quy tắc active đều phải có thông tin nguồn truy vết
                for rule in &ir.atomic_rules {
                    if rule.status == "active" && rule.source_spans.is_empty() {
                        eprintln!("[-] Vi phạm bất biến (INV-01): Quy tắc {} không có đoạn nguồn source_spans!", rule.id);
                        return false;
                    }
                }
                println!("[+] Prompt IR hợp lệ theo chuẩn V4.1. Tổng số quy tắc: {}, Bất biến: {}", 
                         ir.atomic_rules.len(), ir.invariants.len());
                true
            }
            Err(e) => {
                eprintln!("[-] Lỗi kiểm tra lược đồ Schema: {}", e);
                false
            }
        },
        Err(e) => {
            eprintln!("[-] Lỗi khi đọc tệp IR: {}", e);
            false
        }
    }
}

/// Điểm bắt đầu công cụ CLI ir_validator
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ir_path_opt: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--ir" && i + 1 < args.len() {
            ir_path_opt = Some(args[i + 1].clone());
            i += 1;
        }
        i += 1;
    }

    let ir_path_str = match ir_path_opt {
        Some(p) => p,
        None => {
            eprintln!("Cú pháp: ir_validator --ir <đường-dẫn-prompt_ir.json>");
            std::process::exit(1);
        }
    };

    let is_valid = validate_ir(Path::new(&ir_path_str));
    if !is_valid {
        std::process::exit(1);
    }
}
