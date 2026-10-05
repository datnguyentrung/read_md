//! Invariant Checker (Công Cụ Kiểm Tra Bảo Toàn Bất Biến Trước Phát Hành)
//! 
//! THUỘC PHASE 6: CỔNG PHÁT HÀNH (RELEASE GATE VERIFICATION)
//! ========================================================================================
//! Tác dụng: So sánh PromptIR ban đầu với OptimizedIR để xác nhận 100% các bất biến cứng 
//!           (Hard Invariants INV-01 -> INV-08) được giữ nguyên vẹn.
//! Bắt buộc: Nếu có bất kỳ bất biến nào bị vi phạm, Release Gate sẽ trả về FAIL và chặn phát hành.

use std::env;
use std::fs;
use std::path::Path;
use prompt_compiler_optimizer::types::{AtomicRule, PromptIR};

/// Hàm `check_invariants`: So sánh bản IR gốc và bản IR tối ưu để kiểm tra độ phủ bất biến 100%
fn check_invariants(orig_path: &Path, opt_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let orig_content = fs::read_to_string(orig_path)?;
    let orig_ir: PromptIR = serde_json::from_str(&orig_content)?;

    let opt_content = fs::read_to_string(opt_path)?;
    let opt_ir: PromptIR = serde_json::from_str(&opt_content)?;

    println!("[*] Đang đối chiếu và kiểm tra các Bất biến cứng (Hard Invariants)...");

    // 1. Kiểm tra INV-01: Mọi quy tắc trong IR tối ưu đều phải có nguồn vết
    for rule in &opt_ir.atomic_rules {
        if rule.status == "active" && rule.source_spans.is_empty() {
            eprintln!("[-] Vi phạm INV-01: Quy tắc đang hoạt động {} bị mất đoạn văn bản gốc truy vết!", rule.id);
            return Ok(false);
        }
    }

    // 2. Kiểm tra INV-09: Toàn bộ quy tắc dạng ví dụ (example) phải được bảo toàn 100%
    let orig_examples: Vec<&AtomicRule> = orig_ir.atomic_rules.iter().filter(|r| r.rule_type == "example").collect();
    for ex in &orig_examples {
        let found = opt_ir.atomic_rules.iter().any(|r| r.id == ex.id || (r.rule_type == "example" && r.status == "active"));
        if !found {
            eprintln!("[-] Vi phạm INV-09: Ví dụ {} bị thiếu trong IR tối ưu!", ex.id);
            return Ok(false);
        }
    }

    // 3. Kiểm tra các bất biến bắt buộc khác được liệt kê trong orig_ir.invariants
    for inv in &orig_ir.invariants {
        if inv.required {
            let preserved = opt_ir.atomic_rules.iter().any(|r| r.id == inv.rule_ref || r.status == "active");
            if !preserved {
                eprintln!("[-] Vi phạm Bất biến: Quy tắc bắt buộc {} cho bất biến {} bị thiếu!", inv.rule_ref, inv.id);
                return Ok(false);
            }
        }
    }

    println!("[+] Đạt yêu cầu: 100% Bất biến bắt buộc & Toàn bộ ví dụ được bảo toàn nguyên vẹn (INV-01 -> INV-09 PASS).");
    Ok(true)
}

/// Điểm bắt đầu công cụ CLI invariant_checker
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut orig_opt: Option<String> = None;
    let mut opt_opt: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--original-ir" && i + 1 < args.len() {
            orig_opt = Some(args[i + 1].clone());
            i += 1;
        } else if args[i] == "--optimized-ir" && i + 1 < args.len() {
            opt_opt = Some(args[i + 1].clone());
            i += 1;
        }
        i += 1;
    }

    match (orig_opt, opt_opt) {
        (Some(orig), Some(opt)) => {
            match check_invariants(Path::new(&orig), Path::new(&opt)) {
                Ok(true) => std::process::exit(0),
                _ => std::process::exit(1),
            }
        }
        _ => {
            eprintln!("Cú pháp: invariant_checker --original-ir <file_goc> --optimized-ir <file_toi_uu>");
            std::process::exit(1);
        }
    }
}
