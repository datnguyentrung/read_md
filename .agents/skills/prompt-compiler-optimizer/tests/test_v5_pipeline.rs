use prompt_compiler_optimizer::pipeline::{
    calculate_metrics, capability_registry, format_cleanup, parse_prompt, plan_optimization,
    protected_line_ranges, render_markdown, resolve_goal, resolve_profile, run_pipeline,
    sha256_hex, GoalResolution, PipelineConfig,
};
use prompt_compiler_optimizer::types::{
    AssuranceLevel, ImplementationStatus, ManualOverrideAudit, ProfileResolutionStatus,
    ReleaseDecision,
};
use std::process::Command;

fn sample_prompt() -> &'static str {
    "# System\n\nAlways answer from supplied data.\n\n\n## Example\n\n```json\n{\n  \"type\": \"sample\"\n}\n```\n\n## Output\n\nOutput valid JSON.  \n"
}

#[test]
fn sha256_matches_standard_vector() {
    assert_eq!(
        sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn parser_creates_multiple_rules_and_example_nodes() {
    let profile = resolve_profile(None, None, sample_prompt());
    let ir = parse_prompt(sample_prompt(), &profile);
    assert!(ir.atomic_rules.len() > 1);
    let example = ir
        .atomic_rules
        .iter()
        .find(|rule| rule.rule_type == "example" && rule.semantics.contains("\"type\": \"sample\""))
        .expect("payload example rule");
    assert!(example.source_spans[0].line_range.is_some());
}

#[test]
fn formatter_collapses_external_whitespace_and_preserves_example_bytes() {
    let profile = resolve_profile(None, None, sample_prompt());
    let ir = parse_prompt(sample_prompt(), &profile);
    let protected = protected_line_ranges(&ir);
    let (formatted, changed) = format_cleanup(sample_prompt(), &protected);
    assert!(changed);
    assert!(!formatted.contains("\n\n\n"));
    assert!(formatted.contains("```json\n{\n  \"type\": \"sample\"\n}\n```"));
    assert!(!formatted.contains("JSON.  \n"));
}

#[test]
fn profile_resolution_is_evidence_driven() {
    let explicit = resolve_profile(Some("custom-model"), None, "prompt");
    assert_eq!(explicit.status, ProfileResolutionStatus::Explicit);
    let runtime = serde_json::json!({"model":"runtime-model","capabilities":{"json":"SUPPORTED"}});
    let detected = resolve_profile(None, Some(&runtime), "prompt");
    assert_eq!(detected.status, ProfileResolutionStatus::Detected);
    let unknown = resolve_profile(None, None, "prompt");
    assert_eq!(unknown.status, ProfileResolutionStatus::Unknown);
    assert_eq!(unknown.profile, "GENERIC_UNKNOWN");
}

#[test]
fn goal_inference_uses_budget_then_safe_fallback() {
    let profile = resolve_profile(None, None, sample_prompt());
    let ir = parse_prompt(sample_prompt(), &profile);
    let metrics = calculate_metrics(sample_prompt());
    let constrained = PipelineConfig {
        token_budget: Some(1),
        ..PipelineConfig::default()
    };
    assert_eq!(
        resolve_goal(&constrained, &ir, &metrics).inferred_goal,
        "TOKEN_EFFICIENCY"
    );
    assert_eq!(
        resolve_goal(&PipelineConfig::default(), &ir, &metrics).inferred_goal,
        "BALANCED_PRESERVATION"
    );
}

#[test]
fn unsupported_requested_tier_is_blocked_without_downgrade() {
    let config = PipelineConfig {
        tier: Some("MAJOR".to_string()),
        ..PipelineConfig::default()
    };
    let goal = GoalResolution {
        requested_goal: None,
        inferred_goal: "BALANCED_PRESERVATION".to_string(),
        source: "TEST".to_string(),
        confidence: "HIGH".to_string(),
        evidence: vec![],
    };
    let plan = plan_optimization(&config, &goal, true);
    assert_eq!(plan.selected_tier, "MAJOR");
    assert_eq!(plan.execution_status, "PARTIAL");
    assert_eq!(plan.selected_passes, vec!["format_cleanup"]);
    assert!(plan
        .missing_capabilities
        .contains(&"GENERALIZATION".to_string()));
}

#[test]
fn registry_never_marks_placeholder_passes_implemented() {
    let registry = capability_registry();
    for name in [
        "generalization",
        "decision_tree",
        "modularization",
        "externalization",
    ] {
        let capability = registry.iter().find(|item| item.name == name).unwrap();
        assert_eq!(
            capability.implementation_status,
            ImplementationStatus::NotImplemented
        );
    }
}

#[test]
fn static_success_does_not_release_by_default() {
    let report = run_pipeline(sample_prompt(), &PipelineConfig::default());
    assert_eq!(
        report.verification.assurance_level,
        AssuranceLevel::VerifiedStatic
    );
    assert_eq!(
        report.verification.release_decision,
        ReleaseDecision::BlockedInsufficientAssurance
    );
    assert_eq!(
        format!("{:?}", report.verification.layers[1].status),
        "NotExecuted"
    );
}

#[test]
fn differential_evidence_can_satisfy_default_assurance() {
    let config = PipelineConfig {
        differential_evidence: Some(
            serde_json::json!({"passed":true,"test_count":12,"critical_regressions":0}),
        ),
        ..PipelineConfig::default()
    };
    let report = run_pipeline(sample_prompt(), &config);
    assert_eq!(
        report.verification.assurance_level,
        AssuranceLevel::VerifiedDifferential
    );
    assert_eq!(
        report.verification.release_decision,
        ReleaseDecision::Released
    );
}

#[test]
fn manual_override_keeps_static_assurance_and_records_audit() {
    let config = PipelineConfig {
        manual_override: Some(ManualOverrideAudit {
            approver: "risk-owner".to_string(),
            reason: "controlled trial".to_string(),
            accepted_risks: "L1 not executed".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
        }),
        ..PipelineConfig::default()
    };
    let report = run_pipeline(sample_prompt(), &config);
    assert_eq!(
        report.verification.assurance_level,
        AssuranceLevel::VerifiedStatic
    );
    assert_eq!(
        report.verification.release_decision,
        ReleaseDecision::OverrideReleased
    );
    assert_eq!(
        report.verification.manual_override.unwrap().approver,
        "risk-owner"
    );
}

#[test]
fn markdown_report_has_required_order_and_safe_fence() {
    let prompt = "Rule\n\n```\ninside\n```\n";
    let config = PipelineConfig {
        min_assurance: AssuranceLevel::VerifiedStatic,
        ..PipelineConfig::default()
    };
    let report = run_pipeline(prompt, &config);
    let markdown = render_markdown(&report);
    let metrics = markdown.find("## Metrics Table").unwrap();
    let changes = markdown.find("## Transformation ChangeLog").unwrap();
    let candidate = markdown.find("## Compiled Prompt").unwrap();
    let verification = markdown
        .find("## Verification / Assurance / Release")
        .unwrap();
    assert!(metrics < changes && changes < candidate && candidate < verification);
    assert!(markdown.contains("````markdown"));
    assert!(markdown.contains("Rule\n\n```\ninside\n```"));
}

#[test]
fn cli_defaults_to_stdout_without_creating_artifacts() {
    let temp = std::env::temp_dir().join(format!("prompt-compiler-cli-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&temp);
    std::fs::create_dir_all(&temp).unwrap();
    let input = temp.join("prompt.md");
    std::fs::write(&input, sample_prompt()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_run_pipeline"))
        .current_dir(&temp)
        .args(["--input", input.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stdout).contains("## Metrics Table"));
    assert!(!temp.join("artifacts").exists());
    std::fs::remove_dir_all(&temp).unwrap();
}

#[test]
fn cli_rejects_incomplete_manual_override() {
    let temp =
        std::env::temp_dir().join(format!("prompt-compiler-override-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&temp);
    std::fs::create_dir_all(&temp).unwrap();
    let input = temp.join("prompt.md");
    std::fs::write(&input, sample_prompt()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_run_pipeline"))
        .args([
            "--input",
            input.to_str().unwrap(),
            "--override-release",
            "--approved-by",
            "owner",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("requires --approved-by, --override-reason, and --accepted-risk"));
    std::fs::remove_dir_all(&temp).unwrap();
}

#[test]
fn matching_static_evidence_is_reused() {
    let first = run_pipeline(sample_prompt(), &PipelineConfig::default());
    let config = PipelineConfig {
        static_evidence: Some(serde_json::to_value(&first).unwrap()),
        ..PipelineConfig::default()
    };
    let second = run_pipeline(sample_prompt(), &config);
    assert!(second.verification.layers[0]
        .reason
        .contains("reused matching static evidence fingerprint"));
}

#[test]
fn invalid_differential_evidence_fails_closed() {
    let config = PipelineConfig {
        differential_evidence: Some(
            serde_json::json!({"passed":true,"test_count":0,"critical_regressions":0}),
        ),
        ..PipelineConfig::default()
    };
    let report = run_pipeline(sample_prompt(), &config);
    assert_eq!(
        report.verification.release_decision,
        ReleaseDecision::Failed
    );
}

#[test]
fn override_does_not_bypass_unsupported_capability() {
    let config = PipelineConfig {
        tier: Some("MAJOR".to_string()),
        manual_override: Some(ManualOverrideAudit {
            approver: "owner".to_string(),
            reason: "request".to_string(),
            accepted_risks: "known".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
        }),
        ..PipelineConfig::default()
    };
    let report = run_pipeline(sample_prompt(), &config);
    assert_eq!(
        report.verification.release_decision,
        ReleaseDecision::BlockedUnsupported
    );
}

#[test]
fn output_dir_is_explicit_and_writes_audit_bundle() {
    let temp =
        std::env::temp_dir().join(format!("prompt-compiler-artifact-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&temp);
    std::fs::create_dir_all(&temp).unwrap();
    let input = temp.join("prompt.md");
    let output_dir = temp.join("audit");
    std::fs::write(&input, sample_prompt()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_run_pipeline"))
        .args([
            "--input",
            input.to_str().unwrap(),
            "--output-dir",
            output_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    for file in [
        "candidate.prompt.md",
        "report.md",
        "report.json",
        "capabilities.json",
    ] {
        assert!(output_dir.join(file).exists(), "missing {}", file);
    }
    std::fs::remove_dir_all(&temp).unwrap();
}
