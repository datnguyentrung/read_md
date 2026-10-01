//! Prompt Metrics
//! Calculates token length estimates, line counts, and word counts in Rust.

use std::env;
use std::fs;
use std::path::Path;

struct Metrics {
    lines: usize,
    words: usize,
    estimated_tokens: usize,
}

fn calculate_metrics(text: &str) -> Metrics {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let estimated_tokens = (words as f64 * 1.3) as usize;

    Metrics {
        lines,
        words,
        estimated_tokens,
    }
}

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
            eprintln!("Usage: prompt_metrics --file <prompt-file>");
            std::process::exit(1);
        }
    };

    match fs::read_to_string(Path::new(&file_str)) {
        Ok(content) => {
            let m = calculate_metrics(&content);
            println!("[Metrics for {}]", file_str);
            println!("  lines: {}", m.lines);
            println!("  words: {}", m.words);
            println!("  estimated_tokens: {}", m.estimated_tokens);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}
