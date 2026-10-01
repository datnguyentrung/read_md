//! Prompt Metrics (Công Cụ Đo Lường Chỉ Số Token V4.1)
//! 
//! THUỘC PHASE 6: ĐO LƯỜNG VÀ CHẤM ĐIỂM (METRICS & AUDIT DELTA)
//! ========================================================================================
//! Tác dụng: Tính toán chính xác số dòng, số từ và số lượng token ước tính của văn bản prompt.
//!           Giúp đo lường tỷ lệ giảm token (% token reduction) giữa prompt gốc và candidate prompt.

use std::env;
use std::fs;
use std::path::Path;

struct Metrics {
    lines: usize,
    words: usize,
    estimated_tokens: usize,
}

/// Hàm `calculate_metrics`: Phân tích thống kê dung lượng prompt
fn calculate_metrics(text: &str) -> Metrics {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    // Ước tính hệ số token 1.3 cho tiếng Anh / code
    let estimated_tokens = (words as f64 * 1.3) as usize;

    Metrics {
        lines,
        words,
        estimated_tokens,
    }
}

/// Điểm bắt đầu công cụ CLI prompt_metrics
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_opt: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        if (args[i] == "--file" || args[i] == "-f") && i + 1 < args.len() {
            file_opt = Some(args[i + 1].clone());
            i += 1;
        }
        i += 1;
    }

    let file_str = match file_opt {
        Some(f) => f,
        None => {
            eprintln!("Cú pháp: prompt_metrics --file <prompt-file>");
            std::process::exit(1);
        }
    };

    match fs::read_to_string(Path::new(&file_str)) {
        Ok(content) => {
            let m = calculate_metrics(&content);
            println!("[Chỉ số đo lường cho tệp '{}']", file_str);
            println!("  Số dòng (lines): {}", m.lines);
            println!("  Số từ (words): {}", m.words);
            println!("  Ước tính token (estimated_tokens): {}", m.estimated_tokens);
        }
        Err(e) => {
            eprintln!("[-] Lỗi khi đọc tệp: {}", e);
            std::process::exit(1);
        }
    }
}
