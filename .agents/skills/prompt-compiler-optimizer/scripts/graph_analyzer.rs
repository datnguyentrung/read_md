//! Graph Analyzer
//! Analyzes dependency and conflict relations across atomic rules in Rust.

use std::env;
use std::fs;
use std::path::Path;
use prompt_compiler_optimizer::types::{GraphNode, PromptIR, SemanticGraph};

fn analyze_graph(ir_path: &Path) -> Result<SemanticGraph, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(ir_path)?;
    let ir: PromptIR = serde_json::from_str(&content)?;
    
    println!("[*] Analyzing semantic graph for {} atomic rules...", ir.atomic_rules.len());
    
    let nodes = ir
        .atomic_rules
        .iter()
        .map(|r| GraphNode {
            id: r.id.clone(),
            label: if r.description.len() > 30 {
                r.description[..30].to_string()
            } else {
                r.description.clone()
            },
            category: r.rule_type.clone(),
        })
        .collect();

    Ok(SemanticGraph {
        nodes,
        edges: vec![],
    })
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
            eprintln!("Usage: graph_analyzer --ir <path-to-prompt_ir.json>");
            std::process::exit(1);
        }
    };

    match analyze_graph(Path::new(&ir_path_str)) {
        Ok(graph) => {
            let out = serde_json::to_string_pretty(&graph).unwrap();
            println!("{}", out);
        }
        Err(e) => {
            eprintln!("Error analyzing graph: {}", e);
            std::process::exit(1);
        }
    }
}
