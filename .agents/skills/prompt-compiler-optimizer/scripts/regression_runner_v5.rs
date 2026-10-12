use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct DifferentialEvidence {
    passed: bool,
    #[serde(default)]
    test_count: usize,
    #[serde(default)]
    critical_regressions: usize,
    #[serde(default)]
    runner: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let evidence_path = args
        .windows(2)
        .find(|pair| pair[0] == "--evidence")
        .map(|pair| PathBuf::from(&pair[1]));
    let Some(path) = evidence_path else {
        println!("{{\"layer\":\"L1_DIFFERENTIAL\",\"status\":\"NOT_EXECUTED\",\"reason\":\"no validated runner evidence supplied\"}}");
        std::process::exit(2);
    };
    let result = fs::read_to_string(&path)
        .map_err(|e| format!("cannot read {}: {}", path.display(), e))
        .and_then(|content| {
            serde_json::from_str::<DifferentialEvidence>(&content)
                .map_err(|e| format!("invalid evidence: {}", e))
        });
    match result {
        Ok(evidence)
            if evidence.passed && evidence.critical_regressions == 0 && evidence.test_count > 0 =>
        {
            println!("{}", serde_json::to_string_pretty(&evidence).unwrap());
        }
        Ok(evidence) => {
            println!("{}", serde_json::to_string_pretty(&evidence).unwrap());
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}
