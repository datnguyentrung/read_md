use prompt_compiler_optimizer::pipeline::{
    render_markdown, run_pipeline, utc_timestamp, write_artifacts, PipelineConfig,
};
use prompt_compiler_optimizer::types::{AssuranceLevel, ManualOverrideAudit, ReleaseDecision};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
struct CliArgs {
    input: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    profile: Option<String>,
    goal: Option<String>,
    tier: Option<String>,
    token_budget: Option<usize>,
    latency_budget_ms: Option<u64>,
    min_assurance: Option<String>,
    runtime_capabilities: Option<PathBuf>,
    differential_evidence: Option<PathBuf>,
    static_evidence: Option<PathBuf>,
    report_format: String,
    override_release: bool,
    approved_by: Option<String>,
    override_reason: Option<String>,
    accepted_risk: Option<String>,
}

fn usage() -> &'static str {
    "Usage: run_pipeline --input <prompt> [--profile <name>] [--goal <goal>] [--tier <tier>] \
     [--token-budget <n>] [--latency-budget-ms <n>] [--min-assurance <level>] \
     [--runtime-capabilities <json>] [--static-evidence <report.json>] [--differential-evidence <json>] \
     [--report-format <markdown|json>] [--output-dir <dir>] \
     [--override-release --approved-by <id> --override-reason <text> --accepted-risk <text>]"
}

fn parse_args() -> Result<CliArgs, String> {
    let args: Vec<String> = env::args().collect();
    let mut parsed = CliArgs {
        report_format: "markdown".to_string(),
        ..CliArgs::default()
    };
    let mut i = 1;
    while i < args.len() {
        let next = |index: usize, flag: &str| {
            args.get(index + 1)
                .cloned()
                .ok_or_else(|| format!("{} requires a value", flag))
        };
        match args[i].as_str() {
            "--input" | "-i" => {
                parsed.input = Some(PathBuf::from(next(i, &args[i])?));
                i += 1;
            }
            "--output-dir" | "-o" => {
                parsed.output_dir = Some(PathBuf::from(next(i, &args[i])?));
                i += 1;
            }
            "--profile" | "-p" => {
                parsed.profile = Some(next(i, &args[i])?);
                i += 1;
            }
            "--goal" => {
                parsed.goal = Some(next(i, &args[i])?);
                i += 1;
            }
            "--tier" => {
                parsed.tier = Some(next(i, &args[i])?);
                i += 1;
            }
            "--token-budget" => {
                parsed.token_budget = Some(
                    next(i, &args[i])?
                        .parse()
                        .map_err(|_| "--token-budget must be a positive integer")?,
                );
                i += 1;
            }
            "--latency-budget-ms" => {
                parsed.latency_budget_ms = Some(
                    next(i, &args[i])?
                        .parse()
                        .map_err(|_| "--latency-budget-ms must be a positive integer")?,
                );
                i += 1;
            }
            "--min-assurance" => {
                parsed.min_assurance = Some(next(i, &args[i])?);
                i += 1;
            }
            "--runtime-capabilities" => {
                parsed.runtime_capabilities = Some(PathBuf::from(next(i, &args[i])?));
                i += 1;
            }
            "--differential-evidence" => {
                parsed.differential_evidence = Some(PathBuf::from(next(i, &args[i])?));
                i += 1;
            }
            "--static-evidence" => {
                parsed.static_evidence = Some(PathBuf::from(next(i, &args[i])?));
                i += 1;
            }
            "--report-format" => {
                parsed.report_format = next(i, &args[i])?.to_lowercase();
                i += 1;
            }
            "--override-release" => parsed.override_release = true,
            "--approved-by" => {
                parsed.approved_by = Some(next(i, &args[i])?);
                i += 1;
            }
            "--override-reason" => {
                parsed.override_reason = Some(next(i, &args[i])?);
                i += 1;
            }
            "--accepted-risk" => {
                parsed.accepted_risk = Some(next(i, &args[i])?);
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage());
                std::process::exit(0);
            }
            unknown => return Err(format!("unknown argument: {}", unknown)),
        }
        i += 1;
    }
    if parsed.input.is_none() {
        return Err("--input is required".to_string());
    }
    if !matches!(parsed.report_format.as_str(), "markdown" | "json") {
        return Err("--report-format must be markdown or json".to_string());
    }
    if parsed.override_release
        && (parsed.approved_by.is_none()
            || parsed.override_reason.is_none()
            || parsed.accepted_risk.is_none())
    {
        return Err(
            "--override-release requires --approved-by, --override-reason, and --accepted-risk"
                .to_string(),
        );
    }
    if !parsed.override_release
        && (parsed.approved_by.is_some()
            || parsed.override_reason.is_some()
            || parsed.accepted_risk.is_some())
    {
        return Err("override audit fields require --override-release".to_string());
    }
    Ok(parsed)
}

fn parse_assurance(value: Option<&str>) -> Result<AssuranceLevel, String> {
    match value
        .unwrap_or("VERIFIED_DIFFERENTIAL")
        .to_uppercase()
        .as_str()
    {
        "UNVERIFIED" => Ok(AssuranceLevel::Unverified),
        "VERIFIED_STATIC" => Ok(AssuranceLevel::VerifiedStatic),
        "VERIFIED_DIFFERENTIAL" | "VERIFIED_OFFLINE" => Ok(AssuranceLevel::VerifiedDifferential),
        "VERIFIED_CONTRACT" => Ok(AssuranceLevel::VerifiedContract),
        "VERIFIED_MOCK" => Ok(AssuranceLevel::VerifiedMock),
        "VERIFIED_E2E" => Ok(AssuranceLevel::VerifiedE2e),
        other => Err(format!("unsupported assurance level: {}", other)),
    }
}

fn read_json(path: Option<&Path>) -> Result<Option<serde_json::Value>, String> {
    match path {
        Some(path) => {
            let content = fs::read_to_string(path)
                .map_err(|e| format!("cannot read {}: {}", path.display(), e))?;
            serde_json::from_str(&content)
                .map(Some)
                .map_err(|e| format!("invalid JSON in {}: {}", path.display(), e))
        }
        None => Ok(None),
    }
}

fn real_main() -> Result<i32, String> {
    let args = parse_args()?;
    let input = args.input.as_ref().expect("validated input");
    let source =
        fs::read_to_string(input).map_err(|e| format!("cannot read {}: {}", input.display(), e))?;
    let manual_override = if args.override_release {
        Some(ManualOverrideAudit {
            approver: args.approved_by.expect("validated approver"),
            reason: args.override_reason.expect("validated reason"),
            accepted_risks: args.accepted_risk.expect("validated accepted risk"),
            timestamp: utc_timestamp(),
        })
    } else {
        None
    };
    let config = PipelineConfig {
        profile: args.profile,
        goal: args.goal,
        tier: args.tier,
        token_budget: args.token_budget,
        latency_budget_ms: args.latency_budget_ms,
        min_assurance: parse_assurance(args.min_assurance.as_deref())?,
        runtime_capabilities: read_json(args.runtime_capabilities.as_deref())?,
        differential_evidence: read_json(args.differential_evidence.as_deref())?,
        manual_override,
        static_evidence: read_json(args.static_evidence.as_deref())?,
    };
    let report = run_pipeline(&source, &config);
    if args.report_format == "json" {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
        );
    } else {
        print!("{}", render_markdown(&report));
    }
    if let Some(output_dir) = args.output_dir {
        write_artifacts(&output_dir, &report)
            .map_err(|e| format!("cannot write artifacts: {}", e))?;
    }
    Ok(match report.verification.release_decision {
        ReleaseDecision::Released | ReleaseDecision::OverrideReleased => 0,
        _ => 2,
    })
}

fn main() {
    match real_main() {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            eprintln!("error: {}\n{}", error, usage());
            std::process::exit(1);
        }
    }
}
