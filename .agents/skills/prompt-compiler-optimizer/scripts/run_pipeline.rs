//! Run Pipeline
//! Main entrypoint to run the multi-pass prompt compiler and optimizer pipeline in Rust.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use prompt_compiler_optimizer::passes::{normalize_terms, reorder_structure, semantic_dedup};
use prompt_compiler_optimizer::types::{AtomicRule, PromptIR, Section};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input_file: Option<String> = None;
    let mut output_dir = String::from("./artifacts");
    let mut profile = String::from("gpt-4o");

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" | "-i" => {
                if i + 1 < args.len() {
                    input_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--output-dir" | "-o" => {
                if i + 1 < args.len() {
                    output_dir = args[i + 1].clone();
                    i += 1;
                }
            }
            "--profile" | "-p" => {
                if i + 1 < args.len() {
                    profile = args[i + 1].clone();
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let input_path_str = match input_file {
        Some(f) => f,
        None => {
            eprintln!("Usage: run_pipeline --input <prompt-file> [--output-dir <dir>] [--profile <model>]");
            std::process::exit(1);
        }
    };

    let input_path = Path::new(&input_path_str);
    if !input_path.exists() {
        eprintln!("Error: Input file not found: {}", input_path_str);
        std::process::exit(1);
    }

    let raw_text = fs::read_to_string(input_path).expect("Failed to read input file");
    println!("[*] Compiling prompt from {} for profile: {}", input_path_str, profile);

    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("prompt")
        .to_string();

    let mut metadata = HashMap::new();
    metadata.insert("title".to_string(), serde_json::Value::String(stem));
    metadata.insert("target_model".to_string(), serde_json::Value::String(profile));

    let mut ir = PromptIR {
        version: "1.0.0".to_string(),
        metadata,
        sections: vec![Section {
            id: "s1".to_string(),
            title: "System Prompt".to_string(),
            order: 1,
            content: raw_text.clone(),
        }],
        atomic_rules: vec![AtomicRule {
            id: "r1".to_string(),
            section_id: Some("s1".to_string()),
            rule_type: "behavior".to_string(),
            description: if raw_text.len() > 200 {
                raw_text[..200].trim().to_string()
            } else {
                raw_text.trim().to_string()
            },
            priority: "HIGH".to_string(),
            conditions: vec![],
            actions: vec![],
            tags: vec![],
        }],
    };

    println!("[*] Running optimization passes...");
    ir = normalize_terms::run_pass(ir, None);
    ir = semantic_dedup::run_pass(ir);
    ir = reorder_structure::run_pass(ir);

    let out_dir_path = Path::new(&output_dir);
    fs::create_dir_all(out_dir_path).expect("Failed to create output directory");
    let ir_out_path = out_dir_path.join("prompt_ir.json");

    let serialized = serde_json::to_string_pretty(&ir).expect("Failed to serialize IR to JSON");
    fs::write(&ir_out_path, serialized).expect("Failed to write IR output");

    println!("[+] Compiled IR written to: {}", ir_out_path.display());
}
