//! Artifact Manager
//! Handles archiving, versioning, and reporting for prompt compilation runs in Rust.

use std::env;
use std::fs;
use std::path::Path;

fn archive_run(run_id: &str, artifact_dir: &Path) {
    let target = artifact_dir.join(format!("run_{}", run_id));
    fs::create_dir_all(&target).expect("Failed to create archive directory");
    println!("[+] Run archived at: {}", target.display());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut run_id_opt: Option<String> = None;
    let mut artifact_dir_opt = String::from("./artifacts");

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--run-id" && i + 1 < args.len() {
            run_id_opt = Some(args[i + 1].clone());
            i += 1;
        } else if args[i] == "--artifact-dir" && i + 1 < args.len() {
            artifact_dir_opt = args[i + 1].clone();
            i += 1;
        }
        i += 1;
    }

    match run_id_opt {
        Some(run_id) => {
            archive_run(&run_id, Path::new(&artifact_dir_opt));
        }
        None => {
            eprintln!("Usage: artifact_manager --run-id <id> [--artifact-dir <dir>]");
            std::process::exit(1);
        }
    }
}
