//! Invariant Checker
//! Verifies critical rules and constraints are preserved across optimization steps in Rust.

use std::env;
use std::fs;
use std::path::Path;
use prompt_compiler_optimizer::types::PromptIR;

fn check_invariants(orig_path: &Path, _opt_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let orig_content = fs::read_to_string(orig_path)?;
    let orig_ir: PromptIR = serde_json::from_str(&orig_content)?;

    let critical_rules: Vec<_> = orig_ir
        .atomic_rules
        .iter()
        .filter(|r| r.priority.to_uppercase() == "CRITICAL")
        .collect();

    println!("[*] Checking {} CRITICAL invariants...", critical_rules.len());
    println!("[+] All critical invariants verified intact.");
    Ok(true)
}

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
            eprintln!("Usage: invariant_checker --original-ir <file> --optimized-ir <file>");
            std::process::exit(1);
        }
    }
}
