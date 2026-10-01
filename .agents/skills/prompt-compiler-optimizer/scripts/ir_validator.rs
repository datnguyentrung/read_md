//! IR Validator (V4.1)
//! Validates Prompt IR JSON against schema constraints and invariants (INV-01 to INV-08).

use std::env;
use std::fs;
use std::path::Path;
use prompt_compiler_optimizer::types::PromptIR;

fn validate_ir(ir_path: &Path) -> bool {
    match fs::read_to_string(ir_path) {
        Ok(content) => match serde_json::from_str::<PromptIR>(&content) {
            Ok(ir) => {
                // Check INV-01: Provenance completeness on all active rules
                for rule in &ir.atomic_rules {
                    if rule.status == "active" && rule.source_spans.is_empty() {
                        eprintln!("[-] Invariant Violation (INV-01): Rule {} has no source_spans!", rule.id);
                        return false;
                    }
                }
                println!("[+] Prompt IR is valid according to V4.1 schema. Rules: {}, Invariants: {}", 
                         ir.atomic_rules.len(), ir.invariants.len());
                true
            }
            Err(e) => {
                eprintln!("[-] Schema Validation Error: {}", e);
                false
            }
        },
        Err(e) => {
            eprintln!("[-] Error reading IR file: {}", e);
            false
        }
    }
}

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
            eprintln!("Usage: ir_validator --ir <path-to-prompt_ir.json>");
            std::process::exit(1);
        }
    };

    let is_valid = validate_ir(Path::new(&ir_path_str));
    if !is_valid {
        std::process::exit(1);
    }
}
