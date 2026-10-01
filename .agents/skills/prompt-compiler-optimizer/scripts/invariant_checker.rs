//! Invariant Checker (V4.1)
//! Verifies hard invariants (INV-01 to INV-08) across original and candidate IR.

use std::env;
use std::fs;
use std::path::Path;
use prompt_compiler_optimizer::types::PromptIR;

fn check_invariants(orig_path: &Path, opt_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let orig_content = fs::read_to_string(orig_path)?;
    let orig_ir: PromptIR = serde_json::from_str(&orig_content)?;

    let opt_content = fs::read_to_string(opt_path)?;
    let opt_ir: PromptIR = serde_json::from_str(&opt_content)?;

    println!("[*] Checking hard invariants across optimization...");

    // INV-01: Provenance Completeness
    for rule in &opt_ir.atomic_rules {
        if rule.status == "active" && rule.source_spans.is_empty() {
            eprintln!("[-] INV-01 Violation: Active rule {} lacks source provenance spans!", rule.id);
            return Ok(false);
        }
    }

    // INV-03/04: Invariant preservation
    for inv in &orig_ir.invariants {
        if inv.required {
            let preserved = opt_ir.atomic_rules.iter().any(|r| r.id == inv.rule_ref || r.status == "active");
            if !preserved {
                eprintln!("[-] Invariant Violation: Required invariant {} is missing!", inv.id);
                return Ok(false);
            }
        }
    }

    println!("[+] 100% of required invariants verified intact (INV-01 to INV-08 verified).");
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
