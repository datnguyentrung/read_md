use crate::types::{
    AssuranceLevel, AtomicRule, CapabilityState, ChangeLogEntry, ImplementationStatus, Invariant,
    ManualOverrideAudit, PassCapability, PassExecutionRecord, PassExecutionStatus,
    ProfileResolution, ProfileResolutionStatus, PromptIR, ReleaseDecision, Section, SemanticUnit,
    SourceSpan, VerificationLayerResult, VerificationStatus,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const OPTIMIZER_VERSION: &str = "5.0.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PipelineConfig {
    pub profile: Option<String>,
    pub goal: Option<String>,
    pub tier: Option<String>,
    pub token_budget: Option<usize>,
    pub latency_budget_ms: Option<u64>,
    pub min_assurance: AssuranceLevel,
    pub runtime_capabilities: Option<serde_json::Value>,
    pub differential_evidence: Option<serde_json::Value>,
    pub static_evidence: Option<serde_json::Value>,
    pub manual_override: Option<ManualOverrideAudit>,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            profile: None,
            goal: None,
            tier: None,
            token_budget: None,
            latency_budget_ms: None,
            min_assurance: AssuranceLevel::VerifiedDifferential,
            runtime_capabilities: None,
            differential_evidence: None,
            static_evidence: None,
            manual_override: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptMetrics {
    pub characters: usize,
    pub lines: usize,
    pub words: usize,
    pub estimated_tokens: usize,
    pub token_estimate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GoalResolution {
    pub requested_goal: Option<String>,
    pub inferred_goal: String,
    pub source: String,
    pub confidence: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanningResult {
    pub requested_tier: Option<String>,
    pub selected_tier: String,
    pub execution_status: String,
    pub selected_passes: Vec<String>,
    pub missing_capabilities: Vec<String>,
    pub expected_benefits: Vec<String>,
    pub semantic_risks: Vec<String>,
    pub keep_reason: Option<String>,
    pub rationale: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationOutcome {
    pub verification_status: String,
    pub assurance_level: AssuranceLevel,
    pub release_decision: ReleaseDecision,
    pub layers: Vec<VerificationLayerResult>,
    pub evaluation_debt: Vec<String>,
    pub invariant_coverage: f64,
    pub example_preservation_rate: f64,
    pub output_contract_preserved: bool,
    pub critical_regressions: Option<usize>,
    pub manual_override: Option<ManualOverrideAudit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PipelineReport {
    pub schema_version: String,
    pub timestamp: String,
    pub source_hash: String,
    pub candidate_hash: String,
    pub verification_fingerprint: String,
    pub profile_resolution: ProfileResolution,
    pub goal_resolution: GoalResolution,
    pub planning: PlanningResult,
    pub pass_executions: Vec<PassExecutionRecord>,
    pub changes: Vec<ChangeLogEntry>,
    pub metrics_before: PromptMetrics,
    pub metrics_after: PromptMetrics,
    pub candidate: String,
    pub verification: VerificationOutcome,
}

pub fn capability_registry() -> Vec<PassCapability> {
    vec![
        capability(
            "format_cleanup",
            "FORMAT_CLEANUP",
            ImplementationStatus::Implemented,
            vec!["source prompt is valid UTF-8"],
            vec![],
            "raw prompt + protected example spans",
            "formatted prompt with identical protected spans",
            vec![
                "no trailing whitespace",
                "at most one blank line outside protected spans",
            ],
            vec!["L0 structural validation", "example byte preservation"],
        ),
        capability(
            "normalize_terms",
            "TERM_NORMALIZATION",
            ImplementationStatus::Partial,
            vec!["canonical dictionary supplied"],
            vec!["rule-aware source editor"],
            "PromptIR + canonical dictionary",
            "PromptIR and source-preserving edits",
            vec!["rule boundaries unchanged"],
            vec!["L0 provenance", "L1 differential"],
        ),
        capability(
            "semantic_dedup",
            "SEMANTIC_DEDUP",
            ImplementationStatus::Partial,
            vec!["equivalent scope, conditions, and exceptions"],
            vec!["semantic relation evidence", "rule-aware source editor"],
            "PromptIR + relation graph",
            "deduplicated PromptIR and source-preserving edits",
            vec!["all source spans merged", "examples untouched"],
            vec!["L0 invariants", "L1 differential"],
        ),
        capability(
            "reorder_structure",
            "STRUCTURAL_REORDER",
            ImplementationStatus::Partial,
            vec!["explicit priorities and dependency-safe order"],
            vec!["dependency graph", "rule-aware renderer"],
            "PromptIR + dependency graph",
            "reordered PromptIR and prompt",
            vec!["dependencies and exceptions retain precedence"],
            vec!["L0 graph validation", "L1 differential"],
        ),
        capability(
            "preserve_and_structure_examples",
            "EXAMPLE_PRESERVATION",
            ImplementationStatus::Partial,
            vec!["examples parsed into distinct nodes"],
            vec!["complete semantic parser"],
            "PromptIR with example nodes",
            "PromptIR with byte-identical active examples",
            vec!["100% example hashes retained"],
            vec!["L0 example hash comparison"],
        ),
        capability(
            "decision_tree",
            "DECISION_TREE",
            ImplementationStatus::NotImplemented,
            vec!["conditions and exceptions parsed", "priority is explicit"],
            vec!["semantic parser", "differential runner"],
            "decision rules",
            "ordered decision structure",
            vec!["branch boundaries preserved"],
            vec!["boundary tests", "L1 differential"],
        ),
        capability(
            "modularization",
            "MODULARIZATION",
            ImplementationStatus::NotImplemented,
            vec!["module boundaries proven"],
            vec!["runtime module loader"],
            "PromptIR + runtime loader contract",
            "core prompt + module manifest",
            vec!["required modules load deterministically"],
            vec!["loader compatibility", "L3 mock replay"],
        ),
        capability(
            "generalization",
            "GENERALIZATION",
            ImplementationStatus::NotImplemented,
            vec!["scope, conditions, and exceptions equivalent"],
            vec!["semantic relation evidence", "differential runner"],
            "related rules",
            "canonical generalized rules",
            vec!["decision boundaries unchanged"],
            vec!["boundary tests", "L1 differential"],
        ),
        capability(
            "externalization",
            "EXTERNALIZATION",
            ImplementationStatus::NotImplemented,
            vec!["content is externalizable"],
            vec!["storage", "retrieval", "runtime integrity check"],
            "PromptIR + runtime retrieval contract",
            "prompt references + external content manifest",
            vec!["content is retrievable and integrity checked"],
            vec!["runtime compatibility", "L4 E2E"],
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn capability(
    name: &str,
    capability: &str,
    implementation_status: ImplementationStatus,
    preconditions: Vec<&str>,
    dependencies: Vec<&str>,
    input_contract: &str,
    output_contract: &str,
    postconditions: Vec<&str>,
    verification_requirements: Vec<&str>,
) -> PassCapability {
    PassCapability {
        name: name.to_string(),
        capability: capability.to_string(),
        implementation_status,
        preconditions: strings(preconditions),
        dependencies: strings(dependencies),
        input_contract: input_contract.to_string(),
        output_contract: output_contract.to_string(),
        postconditions: strings(postconditions),
        verification_requirements: strings(verification_requirements),
    }
}

fn strings(values: Vec<&str>) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

pub fn parse_prompt(raw: &str, profile: &ProfileResolution) -> PromptIR {
    let normalized = decode_serialized_prompt(raw);
    let source_hash = sha256_hex(raw.as_bytes());
    let lines: Vec<&str> = normalized.lines().collect();
    let mut line_char_offsets = Vec::with_capacity(lines.len());
    let mut char_offset = 0usize;
    for line in &lines {
        line_char_offsets.push(char_offset);
        char_offset += line.chars().count() + 1;
    }
    let blocks = logical_blocks(&lines);
    let mut sections = Vec::new();
    let mut units = Vec::new();
    let mut rules = Vec::new();
    let mut invariants = Vec::new();

    for (index, (start, end, text)) in blocks.iter().enumerate() {
        let id = format!("R-{:04}", index + 1);
        let role = classify_role(text);
        let rule_type = classify_rule_type(text);
        let conditions = extract_matching_lines(text, &["khi ", "if ", "nếu ", "trường hợp"]);
        let exceptions = extract_matching_lines(text, &["ngoại lệ", "except", "trừ khi", "riêng "]);
        let priority = classify_priority(text);
        let section_id = format!("S-{:04}", index + 1);
        let span = SourceSpan {
            section: section_id.clone(),
            line_range: Some(format!("{}-{}", start + 1, end + 1)),
            text: text.clone(),
        };

        sections.push(Section {
            id: section_id.clone(),
            title: block_title(text, index + 1),
            order: index as i32 + 1,
            role_hint: Some(role.clone()),
            content: text.clone(),
        });
        units.push(SemanticUnit {
            id: format!("U-{:04}", index + 1),
            text: text.clone(),
            start: line_char_offsets.get(*start).copied().unwrap_or(0),
            end: line_char_offsets.get(*start).copied().unwrap_or(0) + text.chars().count(),
            section_path: vec![section_id],
            role_hint: Some(role),
        });
        rules.push(AtomicRule {
            id: id.clone(),
            rule_type: rule_type.clone(),
            semantics: text.trim().to_string(),
            actor: Some("Assistant".to_string()),
            action: Some("follow_instruction".to_string()),
            object: None,
            conditions,
            exceptions,
            scope: vec!["global".to_string()],
            priority,
            source_spans: vec![span],
            confidence: Some(0.85),
            status: "active".to_string(),
        });
        if matches!(rule_type.as_str(), "constraint" | "output" | "example") {
            invariants.push(Invariant {
                id: format!("INV-SOURCE-{:04}", index + 1),
                description: format!("Preserve {} rule {}", rule_type, id),
                rule_ref: id,
                required: true,
            });
        }
    }

    let mut metadata = HashMap::new();
    metadata.insert("source_hash".to_string(), serde_json::json!(source_hash));
    metadata.insert(
        "target_model".to_string(),
        serde_json::json!(profile.profile),
    );
    metadata.insert(
        "profile_resolution".to_string(),
        serde_json::json!(format!("{:?}", profile.status).to_uppercase()),
    );

    PromptIR {
        version: OPTIMIZER_VERSION.to_string(),
        metadata,
        sections,
        semantic_units: units,
        atomic_rules: rules,
        invariants,
    }
}

fn logical_blocks(lines: &[&str]) -> Vec<(usize, usize, String)> {
    let mut result = Vec::new();
    let mut start = 0usize;
    let mut current = Vec::new();
    let mut in_fence = false;
    let mut in_a2ui = false;

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~~") {
            in_fence = !in_fence;
        }
        if trimmed.contains("<a2ui-json>") {
            in_a2ui = true;
        }
        if current.is_empty() && !trimmed.is_empty() {
            start = idx;
        }
        if !current.is_empty() || !trimmed.is_empty() {
            current.push(*line);
        }
        if trimmed.contains("</a2ui-json>") {
            in_a2ui = false;
        }

        let boundary = trimmed.is_empty() && !in_fence && !in_a2ui;
        if boundary && current.iter().any(|l| !l.trim().is_empty()) {
            while current.last().map(|l| l.trim().is_empty()).unwrap_or(false) {
                current.pop();
            }
            let end = start + current.len().saturating_sub(1);
            result.push((start, end, current.join("\n")));
            current.clear();
        }
    }
    if current.iter().any(|l| !l.trim().is_empty()) {
        while current.last().map(|l| l.trim().is_empty()).unwrap_or(false) {
            current.pop();
        }
        let end = start + current.len().saturating_sub(1);
        result.push((start, end, current.join("\n")));
    }
    result
}

fn classify_role(text: &str) -> String {
    let lower = text.to_lowercase();
    if is_example(text) {
        "examples"
    } else if lower.contains("output") || lower.contains("schema") || lower.contains("format") {
        "output"
    } else if lower.contains("mục tiêu") || lower.contains("objective") {
        "objective"
    } else if lower.contains("quy tắc") || lower.contains("rule") || lower.contains("cấm") {
        "rules"
    } else {
        "unknown"
    }
    .to_string()
}

fn classify_rule_type(text: &str) -> String {
    let lower = text.to_lowercase();
    if is_example(text) {
        "example"
    } else if lower.contains("schema")
        || lower.contains("output")
        || lower.contains("đầu ra")
        || lower.contains("format")
    {
        "output"
    } else if lower.contains("trường hợp")
        || lower.contains("nếu ")
        || lower.contains(" khi ")
        || lower.starts_with("khi ")
    {
        "decision"
    } else if lower.contains("cấm")
        || lower.contains("bắt buộc")
        || lower.contains("tuyệt đối")
        || lower.contains("must")
    {
        "constraint"
    } else {
        "behavior"
    }
    .to_string()
}

fn is_example(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.contains("ví dụ")
        || lower.contains("example")
        || lower.contains("few-shot")
        || text.contains("<a2ui-json>")
        || text.trim_start().starts_with("```")
}

fn classify_priority(text: &str) -> i32 {
    let lower = text.to_lowercase();
    if lower.contains("critical") || lower.contains("tối thượng") || lower.contains("highest")
    {
        0
    } else if lower.contains("bắt buộc") || lower.contains("tuyệt đối") || lower.contains("cấm")
    {
        100
    } else if is_example(text) {
        800
    } else {
        500
    }
}

fn extract_matching_lines(text: &str, needles: &[&str]) -> Vec<String> {
    text.lines()
        .filter_map(|line| {
            let lower = line.trim().to_lowercase();
            if needles.iter().any(|needle| lower.contains(needle)) {
                Some(line.trim().to_string())
            } else {
                None
            }
        })
        .collect()
}

fn block_title(text: &str, index: usize) -> String {
    let first = text
        .lines()
        .find(|l| !l.trim().is_empty())
        .unwrap_or("Section")
        .trim();
    let cleaned = first.trim_start_matches('#').trim();
    if cleaned.len() <= 80 {
        cleaned.to_string()
    } else {
        format!("Section {}", index)
    }
}

pub fn decode_serialized_prompt(raw: &str) -> String {
    let literal_newlines = raw.matches("\\n").count();
    let actual_newlines = raw.lines().count().saturating_sub(1);
    if literal_newlines < 3 || literal_newlines < actual_newlines / 2 {
        return raw.replace("\r\n", "\n").replace('\r', "\n");
    }

    let mut joined = String::new();
    let mut continuing = false;
    for line in raw.lines() {
        let mut part = line.to_string();
        if continuing {
            part = part.trim_start().trim_start_matches('\\').to_string();
        }
        if part.ends_with('\\') {
            part.pop();
            joined.push_str(&part);
            continuing = true;
        } else {
            joined.push_str(&part);
            joined.push('\n');
            continuing = false;
        }
    }
    joined
        .replace("\\\\n", "\n")
        .replace("\\n", "\n")
        .replace("\\\"", "\"")
}

pub fn protected_line_ranges(ir: &PromptIR) -> HashSet<usize> {
    let mut protected = HashSet::new();
    for rule in &ir.atomic_rules {
        if rule.rule_type != "example" {
            continue;
        }
        for span in &rule.source_spans {
            if let Some(range) = &span.line_range {
                let mut parts = range.split('-').filter_map(|p| p.parse::<usize>().ok());
                if let (Some(start), Some(end)) = (parts.next(), parts.next()) {
                    for line in start..=end {
                        protected.insert(line);
                    }
                }
            }
        }
    }
    protected
}

pub fn format_cleanup(text: &str, protected: &HashSet<usize>) -> (String, bool) {
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut output = Vec::new();
    let mut previous_blank = false;
    for (idx, line) in normalized.lines().enumerate() {
        let line_number = idx + 1;
        if protected.contains(&line_number) {
            output.push(line.to_string());
            previous_blank = line.trim().is_empty();
            continue;
        }
        let cleaned = line.trim_end().to_string();
        let blank = cleaned.trim().is_empty();
        if blank && previous_blank {
            continue;
        }
        output.push(cleaned);
        previous_blank = blank;
    }
    while output.last().map(|l| l.trim().is_empty()).unwrap_or(false) {
        output.pop();
    }
    let candidate = format!("{}\n", output.join("\n"));
    let changed = candidate != text;
    (candidate, changed)
}

pub fn resolve_profile(
    explicit: Option<&str>,
    runtime: Option<&serde_json::Value>,
    raw: &str,
) -> ProfileResolution {
    if let Some(profile) = explicit {
        return ProfileResolution {
            status: ProfileResolutionStatus::Explicit,
            profile: profile.to_string(),
            evidence: vec!["--profile supplied by user".to_string()],
            capabilities: capabilities_from_runtime(runtime),
        };
    }
    if let Some(value) = runtime {
        if let Some(model) = value.get("model").and_then(|v| v.as_str()) {
            return ProfileResolution {
                status: ProfileResolutionStatus::Detected,
                profile: model.to_string(),
                evidence: vec!["runtime capability metadata:model".to_string()],
                capabilities: capabilities_from_runtime(runtime),
            };
        }
        if value.get("capabilities").is_some() {
            return ProfileResolution {
                status: ProfileResolutionStatus::Partial,
                profile: "GENERIC_PARTIAL".to_string(),
                evidence: vec!["runtime capabilities available without model identity".to_string()],
                capabilities: capabilities_from_runtime(runtime),
            };
        }
    }
    let lower = raw.to_lowercase();
    for family in ["claude", "gemini", "gpt"] {
        if lower.contains(&format!("target_model: {}", family))
            || lower.contains(&format!("target model: {}", family))
        {
            return ProfileResolution {
                status: ProfileResolutionStatus::Detected,
                profile: family.to_uppercase(),
                evidence: vec!["trusted prompt metadata marker".to_string()],
                capabilities: HashMap::new(),
            };
        }
    }
    ProfileResolution {
        status: ProfileResolutionStatus::Unknown,
        profile: "GENERIC_UNKNOWN".to_string(),
        evidence: vec!["no explicit profile or trusted runtime metadata".to_string()],
        capabilities: HashMap::new(),
    }
}

fn capabilities_from_runtime(
    runtime: Option<&serde_json::Value>,
) -> HashMap<String, CapabilityState> {
    let mut result = HashMap::new();
    if let Some(map) = runtime
        .and_then(|v| v.get("capabilities"))
        .and_then(|v| v.as_object())
    {
        for (key, value) in map {
            let state = match value.as_str().unwrap_or("UNKNOWN").to_uppercase().as_str() {
                "SUPPORTED" => CapabilityState::Supported,
                "UNSUPPORTED" => CapabilityState::Unsupported,
                _ => CapabilityState::Unknown,
            };
            result.insert(key.clone(), state);
        }
    }
    result
}

pub fn resolve_goal(
    config: &PipelineConfig,
    ir: &PromptIR,
    metrics: &PromptMetrics,
) -> GoalResolution {
    if let Some(goal) = &config.goal {
        return GoalResolution {
            requested_goal: Some(goal.clone()),
            inferred_goal: goal.to_uppercase(),
            source: "EXPLICIT".to_string(),
            confidence: "HIGH".to_string(),
            evidence: vec!["--goal supplied by user".to_string()],
        };
    }
    if let Some(limit) = config.token_budget {
        if metrics.estimated_tokens > limit {
            return GoalResolution {
                requested_goal: None,
                inferred_goal: "TOKEN_EFFICIENCY".to_string(),
                source: "INFERRED_RUNTIME_CONSTRAINT".to_string(),
                confidence: "HIGH".to_string(),
                evidence: vec![format!(
                    "estimated tokens {} exceed budget {}",
                    metrics.estimated_tokens, limit
                )],
            };
        }
    }
    if config.latency_budget_ms.is_some() && metrics.estimated_tokens > 4_000 {
        return GoalResolution {
            requested_goal: None,
            inferred_goal: "LATENCY_REDUCTION".to_string(),
            source: "INFERRED_RUNTIME_CONSTRAINT".to_string(),
            confidence: "MEDIUM".to_string(),
            evidence: vec!["latency budget supplied and prompt is large".to_string()],
        };
    }
    let duplicate_rules = duplicate_rule_count(ir);
    if duplicate_rules > 0 {
        return GoalResolution {
            requested_goal: None,
            inferred_goal: "STRUCTURAL_SIMPLIFICATION".to_string(),
            source: "INFERRED_IR_EVIDENCE".to_string(),
            confidence: "MEDIUM".to_string(),
            evidence: vec![format!(
                "{} exact normalized duplicate rules detected",
                duplicate_rules
            )],
        };
    }
    GoalResolution {
        requested_goal: None,
        inferred_goal: "BALANCED_PRESERVATION".to_string(),
        source: "SAFE_FALLBACK".to_string(),
        confidence: "LOW".to_string(),
        evidence: vec!["insufficient evidence for a narrower optimization objective".to_string()],
    }
}

fn duplicate_rule_count(ir: &PromptIR) -> usize {
    let mut seen = HashSet::new();
    let mut duplicates = 0;
    for rule in &ir.atomic_rules {
        let normalized = normalize_space(&rule.semantics).to_lowercase();
        if !seen.insert(normalized) {
            duplicates += 1;
        }
    }
    duplicates
}

fn normalize_space(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn plan_optimization(
    config: &PipelineConfig,
    goal: &GoalResolution,
    formatting_needed: bool,
) -> PlanningResult {
    let registry = capability_registry();
    let requested = config.tier.as_ref().map(|v| v.to_uppercase());
    let supported_tiers = [
        "KEEP",
        "LIGHT",
        "STRUCTURAL",
        "MAJOR",
        "EXTERNALIZE",
        "MODULARIZE",
    ];
    if let Some(tier) = requested.clone() {
        if !supported_tiers.contains(&tier.as_str()) {
            return blocked_plan(
                requested,
                tier.clone(),
                vec![format!("unknown tier {}", tier)],
            );
        }
        let required = tier_passes(&tier);
        let available = required
            .iter()
            .filter_map(|name| {
                registry.iter().find(|c| c.name == *name).and_then(|cap| {
                    if cap.implementation_status == ImplementationStatus::Implemented {
                        Some((*name).to_string())
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();
        let missing = required
            .iter()
            .filter_map(|name| {
                registry.iter().find(|c| c.name == *name).and_then(|cap| {
                    if cap.implementation_status == ImplementationStatus::Implemented {
                        None
                    } else {
                        Some(cap.capability.clone())
                    }
                })
            })
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            return PlanningResult {
                requested_tier: requested,
                selected_tier: tier.clone(),
                execution_status: if available.is_empty() {
                    "BLOCKED"
                } else {
                    "PARTIAL"
                }
                .to_string(),
                selected_passes: available,
                missing_capabilities: missing,
                expected_benefits: vec![
                    "apply independently valid supported passes without claiming tier completion"
                        .to_string(),
                ],
                semantic_risks: tier_risks(&tier),
                keep_reason: None,
                rationale: vec![
                    "requested tier is only partially executable; no silent downgrade or upgrade"
                        .to_string(),
                ],
            };
        }
        return PlanningResult {
            requested_tier: requested,
            selected_tier: tier.clone(),
            execution_status: "PLANNED".to_string(),
            selected_passes: required.into_iter().map(str::to_string).collect(),
            missing_capabilities: vec![],
            expected_benefits: vec![format!("execute validated {} tier", tier)],
            semantic_risks: tier_risks(&tier),
            keep_reason: None,
            rationale: vec!["requested tier passed capability validation".to_string()],
        };
    }

    if formatting_needed {
        return PlanningResult {
            requested_tier: None,
            selected_tier: "LIGHT".to_string(),
            execution_status: "PLANNED".to_string(),
            selected_passes: vec!["format_cleanup".to_string()],
            missing_capabilities: vec![],
            expected_benefits: vec![
                "remove redundant blank lines and trailing whitespace".to_string()
            ],
            semantic_risks: vec![
                "low: protected examples and payloads remain byte-identical".to_string()
            ],
            keep_reason: None,
            rationale: vec![format!(
                "goal {} permits a proven low-risk formatting pass",
                goal.inferred_goal
            )],
        };
    }
    PlanningResult {
        requested_tier: None,
        selected_tier: "KEEP".to_string(),
        execution_status: "COMPLETED".to_string(),
        selected_passes: vec![],
        missing_capabilities: vec![],
        expected_benefits: vec!["avoid unnecessary semantic risk".to_string()],
        semantic_risks: vec![],
        keep_reason: Some("no supported transformation has a verified benefit".to_string()),
        rationale: vec!["KEEP is preferred over unproven optimization".to_string()],
    }
}

fn blocked_plan(
    requested: Option<String>,
    selected: String,
    missing: Vec<String>,
) -> PlanningResult {
    PlanningResult {
        requested_tier: requested,
        selected_tier: selected,
        execution_status: "BLOCKED".to_string(),
        selected_passes: vec![],
        missing_capabilities: missing,
        expected_benefits: vec![],
        semantic_risks: vec![
            "requested transformation cannot be validated with current capabilities".to_string(),
        ],
        keep_reason: None,
        rationale: vec!["requested tier is not silently downgraded".to_string()],
    }
}

fn tier_passes(tier: &str) -> Vec<&'static str> {
    match tier {
        "KEEP" => vec![],
        "LIGHT" => vec![
            "normalize_terms",
            "semantic_dedup",
            "reorder_structure",
            "preserve_and_structure_examples",
            "format_cleanup",
        ],
        "STRUCTURAL" => vec![
            "reorder_structure",
            "decision_tree",
            "preserve_and_structure_examples",
            "format_cleanup",
        ],
        "MAJOR" => vec![
            "generalization",
            "semantic_dedup",
            "reorder_structure",
            "preserve_and_structure_examples",
            "format_cleanup",
        ],
        "EXTERNALIZE" => vec![
            "externalization",
            "reorder_structure",
            "preserve_and_structure_examples",
            "format_cleanup",
        ],
        "MODULARIZE" => vec![
            "modularization",
            "reorder_structure",
            "preserve_and_structure_examples",
            "format_cleanup",
        ],
        _ => vec![],
    }
}

fn tier_risks(tier: &str) -> Vec<String> {
    match tier {
        "KEEP" => vec![],
        "LIGHT" => vec!["low lexical or formatting drift".to_string()],
        "STRUCTURAL" => vec!["branch precedence may change".to_string()],
        _ => vec!["high semantic or runtime compatibility risk".to_string()],
    }
}

pub fn execute_plan(
    source: &str,
    ir: &PromptIR,
    plan: &mut PlanningResult,
) -> (String, Vec<PassExecutionRecord>, Vec<ChangeLogEntry>) {
    if plan.execution_status == "BLOCKED" {
        return (source.to_string(), vec![], vec![]);
    }
    let mut candidate = source.to_string();
    let mut records = Vec::new();
    let mut changes = Vec::new();
    for pass in &plan.selected_passes {
        if pass == "format_cleanup" {
            let protected = protected_line_ranges(ir);
            let before = candidate.clone();
            let (formatted, changed) = format_cleanup(&candidate, &protected);
            candidate = formatted;
            if changed {
                records.push(PassExecutionRecord {
                    name: pass.clone(),
                    status: PassExecutionStatus::Applied,
                    changed: true,
                    reason: "normalized whitespace outside protected example spans".to_string(),
                    validation: vec![
                        "protected spans retained".to_string(),
                        "blank-line invariant satisfied".to_string(),
                    ],
                });
                changes.push(ChangeLogEntry {
                    change_id: "CH-FORMAT-001".to_string(), pass: pass.clone(), action: "format_cleanup".to_string(),
                    source_rule_ids: vec![], target_rule_ids: vec![],
                    reason: "Normalize newlines, trailing whitespace, and repeated blank lines outside examples".to_string(),
                    risk: "low".to_string(), before_hash: sha256_hex(before.as_bytes()), after_hash: sha256_hex(candidate.as_bytes()),
                    provenance_preserved: true, verification_refs: vec!["L0-format".to_string(), "INV-09".to_string()],
                });
            } else {
                records.push(PassExecutionRecord {
                    name: pass.clone(),
                    status: PassExecutionStatus::SkippedNotApplicable,
                    changed: false,
                    reason: "source already satisfies formatting postconditions".to_string(),
                    validation: vec!["formatting postconditions checked".to_string()],
                });
            }
        } else {
            records.push(PassExecutionRecord {
                name: pass.clone(),
                status: PassExecutionStatus::Unsupported,
                changed: false,
                reason: "capability is not fully implemented and source-preserving".to_string(),
                validation: vec![],
            });
            plan.execution_status = "PARTIAL".to_string();
            if let Some(cap) = capability_registry().into_iter().find(|c| c.name == *pass) {
                if !plan.missing_capabilities.contains(&cap.capability) {
                    plan.missing_capabilities.push(cap.capability);
                }
            }
        }
    }
    if plan.execution_status == "PLANNED" {
        plan.execution_status = if records
            .iter()
            .any(|r| r.status == PassExecutionStatus::Applied)
        {
            "APPLIED"
        } else {
            "COMPLETED"
        }
        .to_string();
    }
    (candidate, records, changes)
}

pub fn calculate_metrics(text: &str) -> PromptMetrics {
    let words = text.split_whitespace().count();
    PromptMetrics {
        characters: text.chars().count(),
        lines: text.lines().count(),
        words,
        estimated_tokens: (words as f64 * 1.3).ceil() as usize,
        token_estimate: true,
    }
}

pub fn verify(
    source_ir: &PromptIR,
    source: &str,
    candidate: &str,
    config: &PipelineConfig,
    plan: &PlanningResult,
) -> VerificationOutcome {
    let mut debt = Vec::new();
    let examples = source_ir
        .atomic_rules
        .iter()
        .filter(|r| r.rule_type == "example")
        .collect::<Vec<_>>();
    let preserved = examples
        .iter()
        .filter(|rule| {
            rule.source_spans
                .iter()
                .all(|span| candidate.contains(&span.text))
        })
        .count();
    let example_rate = if examples.is_empty() {
        1.0
    } else {
        preserved as f64 / examples.len() as f64
    };
    let output_rules = source_ir
        .atomic_rules
        .iter()
        .filter(|r| r.rule_type == "output")
        .collect::<Vec<_>>();
    let output_preserved = output_rules.iter().all(|rule| {
        contract_markers(&rule.semantics)
            .iter()
            .all(|marker| candidate.contains(marker))
    });
    let source_nonempty = !source.trim().is_empty() && !candidate.trim().is_empty();
    let computed_static_pass = source_nonempty && example_rate == 1.0 && output_preserved;
    let fingerprint = static_fingerprint(source, candidate, config);
    let cached_static = config
        .static_evidence
        .as_ref()
        .map(|evidence| {
            evidence
                .get("verification_fingerprint")
                .and_then(|v| v.as_str())
                == Some(fingerprint.as_str())
                && evidence
                    .pointer("/verification/layers/0/status")
                    .and_then(|v| v.as_str())
                    == Some("PASS")
                && evidence.get("schema_version").and_then(|v| v.as_str())
                    == Some(OPTIMIZER_VERSION)
        })
        .unwrap_or(false);
    let static_pass = cached_static || computed_static_pass;
    let l0 = VerificationLayerResult {
        layer: "L0_STATIC".to_string(),
        status: if static_pass {
            VerificationStatus::Pass
        } else {
            VerificationStatus::Fail
        },
        scope: "UTF-8 structure, required examples, output-contract markers, provenance hashes"
            .to_string(),
        reason: if cached_static {
            "reused matching static evidence fingerprint"
        } else if static_pass {
            "all implemented static checks passed"
        } else {
            "one or more static checks failed"
        }
        .to_string(),
        evidence: vec![
            format!("verification_fingerprint={}", fingerprint),
            format!("example_preservation_rate={:.4}", example_rate),
            format!("output_contract_preserved={}", output_preserved),
        ],
    };

    let differential_passed = config.differential_evidence.as_ref().map(|value| {
        value.get("passed").and_then(|v| v.as_bool()) == Some(true)
            && value
                .get("test_count")
                .and_then(|v| v.as_u64())
                .unwrap_or(0)
                > 0
            && value.get("critical_regressions").and_then(|v| v.as_u64()) == Some(0)
            && value
                .get("source_hash")
                .and_then(|v| v.as_str())
                .map(|v| v == sha256_hex(source.as_bytes()))
                .unwrap_or(true)
            && value
                .get("candidate_hash")
                .and_then(|v| v.as_str())
                .map(|v| v == sha256_hex(candidate.as_bytes()))
                .unwrap_or(true)
    });
    let l1_status = match differential_passed {
        Some(true) => VerificationStatus::Pass,
        Some(false) => VerificationStatus::Fail,
        None => VerificationStatus::NotExecuted,
    };
    if differential_passed.is_none() {
        debt.push(
            "L1 differential verification NOT_EXECUTED: no validated evidence supplied".to_string(),
        );
    }
    let l1 = VerificationLayerResult {
        layer: "L1_DIFFERENTIAL".to_string(),
        status: l1_status.clone(),
        scope: "baseline/candidate behavioral comparison".to_string(),
        reason: match differential_passed {
            Some(true) => "supplied differential evidence passed",
            Some(false) => "supplied differential evidence failed",
            None => "no model/differential runner evidence",
        }
        .to_string(),
        evidence: config
            .differential_evidence
            .as_ref()
            .map(|v| vec![sha256_hex(v.to_string().as_bytes())])
            .unwrap_or_default(),
    };
    for layer in ["L2_CONTRACT", "L3_MOCK", "L4_E2E"] {
        debt.push(format!(
            "{} NOT_EXECUTED: no compatible runner evidence supplied",
            layer
        ));
    }
    let mut layers = vec![l0, l1];
    for layer in ["L2_CONTRACT", "L3_MOCK", "L4_E2E"] {
        layers.push(VerificationLayerResult {
            layer: layer.to_string(),
            status: VerificationStatus::NotExecuted,
            scope: "not executed".to_string(),
            reason: "dependency/evidence unavailable".to_string(),
            evidence: vec![],
        });
    }

    let assurance = if !static_pass {
        AssuranceLevel::Unverified
    } else if differential_passed == Some(true) {
        AssuranceLevel::VerifiedDifferential
    } else {
        AssuranceLevel::VerifiedStatic
    };
    let critical_regressions = differential_passed.map(|passed| if passed { 0 } else { 1 });
    let minimum_met = assurance_rank(&assurance) >= assurance_rank(&config.min_assurance);
    let unsupported = plan.execution_status == "BLOCKED" || !plan.missing_capabilities.is_empty();
    let failed = !static_pass || differential_passed == Some(false);
    let mut release = if failed {
        ReleaseDecision::Failed
    } else if unsupported {
        ReleaseDecision::BlockedUnsupported
    } else if !minimum_met {
        ReleaseDecision::BlockedInsufficientAssurance
    } else {
        ReleaseDecision::Released
    };
    if config.manual_override.is_some()
        && matches!(
            release,
            ReleaseDecision::BlockedInsufficientAssurance | ReleaseDecision::NeedsReview
        )
    {
        release = ReleaseDecision::OverrideReleased;
    }
    VerificationOutcome {
        verification_status: if failed {
            "FAILED"
        } else if differential_passed == Some(true) {
            "VERIFIED"
        } else if static_pass {
            "PARTIALLY_VERIFIED"
        } else {
            "UNVERIFIED"
        }
        .to_string(),
        assurance_level: assurance,
        release_decision: release,
        layers,
        evaluation_debt: debt,
        invariant_coverage: if static_pass { 1.0 } else { 0.0 },
        example_preservation_rate: example_rate,
        output_contract_preserved: output_preserved,
        critical_regressions,
        manual_override: config.manual_override.clone(),
    }
}

fn contract_markers(text: &str) -> Vec<String> {
    let candidates = [
        "<a2ui-json>",
        "</a2ui-json>",
        "JSON",
        "XML",
        "Markdown",
        "schema",
        "type",
        "action",
    ];
    candidates
        .iter()
        .filter(|m| text.contains(**m))
        .map(|m| m.to_string())
        .collect()
}

fn assurance_rank(level: &AssuranceLevel) -> usize {
    match level {
        AssuranceLevel::Unverified => 0,
        AssuranceLevel::VerifiedStatic => 1,
        AssuranceLevel::VerifiedDifferential => 2,
        AssuranceLevel::VerifiedContract => 3,
        AssuranceLevel::VerifiedMock => 4,
        AssuranceLevel::VerifiedE2e => 5,
    }
}

pub fn run_pipeline(raw: &str, config: &PipelineConfig) -> PipelineReport {
    let decoded = decode_serialized_prompt(raw);
    let profile = resolve_profile(
        config.profile.as_deref(),
        config.runtime_capabilities.as_ref(),
        &decoded,
    );
    let ir = parse_prompt(&decoded, &profile);
    let before = calculate_metrics(&decoded);
    let protected = protected_line_ranges(&ir);
    let (preview, formatting_needed) = format_cleanup(&decoded, &protected);
    let goal = resolve_goal(config, &ir, &before);
    let mut planning = plan_optimization(config, &goal, formatting_needed && preview != decoded);
    let (candidate, pass_executions, changes) = execute_plan(&decoded, &ir, &mut planning);
    let after = calculate_metrics(&candidate);
    let verification = verify(&ir, &decoded, &candidate, config, &planning);
    let verification_fingerprint = static_fingerprint(&decoded, &candidate, config);
    PipelineReport {
        schema_version: OPTIMIZER_VERSION.to_string(),
        timestamp: utc_timestamp(),
        source_hash: sha256_hex(decoded.as_bytes()),
        candidate_hash: sha256_hex(candidate.as_bytes()),
        verification_fingerprint,
        profile_resolution: profile,
        goal_resolution: goal,
        planning,
        pass_executions,
        changes,
        metrics_before: before,
        metrics_after: after,
        candidate,
        verification,
    }
}

pub fn render_markdown(report: &PipelineReport) -> String {
    let reduction = if report.metrics_before.estimated_tokens == 0 {
        0.0
    } else {
        (1.0 - report.metrics_after.estimated_tokens as f64
            / report.metrics_before.estimated_tokens as f64)
            * 100.0
    };
    let mut out = String::new();
    out.push_str("## Metrics Table\n\n");
    out.push_str("| Metric | Before | After | Delta |\n| :--- | ---: | ---: | ---: |\n");
    out.push_str(&format!(
        "| Characters | {} | {} | {} |\n",
        report.metrics_before.characters,
        report.metrics_after.characters,
        signed_delta(
            report.metrics_before.characters,
            report.metrics_after.characters
        )
    ));
    out.push_str(&format!(
        "| Estimated tokens | {} | {} | {:.2}% |\n",
        report.metrics_before.estimated_tokens, report.metrics_after.estimated_tokens, reduction
    ));
    out.push_str(&format!(
        "| Examples preserved | - | {:.2}% | - |\n\n",
        report.verification.example_preservation_rate * 100.0
    ));

    out.push_str("## Transformation ChangeLog\n\n");
    out.push_str("| Pass | Status | Changed | Reason |\n| :--- | :--- | :---: | :--- |\n");
    if report.pass_executions.is_empty() {
        out.push_str("| none | SKIPPED_NOT_APPLICABLE | no | KEEP or blocked before execution |\n");
    }
    for record in &report.pass_executions {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            escape_table(&record.name),
            enum_label(&record.status),
            if record.changed { "yes" } else { "no" },
            escape_table(&record.reason)
        ));
    }

    out.push_str("\n## Compiled Prompt\n\n");
    let fence = safe_fence(&report.candidate);
    out.push_str(&format!(
        "{}markdown\n{}{}\n\n",
        fence, report.candidate, fence
    ));

    out.push_str("## Verification / Assurance / Release\n\n");
    out.push_str(&format!(
        "- Verification Status: `{}`\n",
        report.verification.verification_status
    ));
    out.push_str(&format!(
        "- Assurance Level: `{}`\n",
        enum_label(&report.verification.assurance_level)
    ));
    out.push_str(&format!(
        "- Release Decision: `{}`\n",
        enum_label(&report.verification.release_decision)
    ));
    out.push_str(&format!(
        "- Selected Tier: `{}`; Execution Status: `{}`\n",
        report.planning.selected_tier, report.planning.execution_status
    ));
    out.push_str(&format!(
        "- Profile: `{}` (`{}`)\n",
        report.profile_resolution.profile,
        enum_label(&report.profile_resolution.status)
    ));
    out.push_str("\n| Layer | Status | Reason |\n| :--- | :--- | :--- |\n");
    for layer in &report.verification.layers {
        out.push_str(&format!(
            "| {} | {} | {} |\n",
            layer.layer,
            enum_label(&layer.status),
            escape_table(&layer.reason)
        ));
    }
    if !report.verification.evaluation_debt.is_empty() {
        out.push_str("\n### Evaluation Debt\n\n");
        for item in &report.verification.evaluation_debt {
            out.push_str(&format!("- {}\n", item));
        }
    }
    out
}

fn signed_delta(before: usize, after: usize) -> String {
    if after >= before {
        format!("+{}", after - before)
    } else {
        format!("-{}", before - after)
    }
}

fn escape_table(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

fn enum_label<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value)
        .unwrap_or_else(|_| "\"UNKNOWN\"".to_string())
        .trim_matches('"')
        .to_string()
}

pub fn static_fingerprint(source: &str, candidate: &str, config: &PipelineConfig) -> String {
    let policy = serde_json::json!({
        "optimizer_version": OPTIMIZER_VERSION,
        "profile": config.profile,
        "goal": config.goal,
        "tier": config.tier,
        "token_budget": config.token_budget,
        "latency_budget_ms": config.latency_budget_ms
    });
    sha256_hex(
        format!(
            "{}|{}|{}",
            sha256_hex(source.as_bytes()),
            sha256_hex(candidate.as_bytes()),
            policy
        )
        .as_bytes(),
    )
}

fn safe_fence(text: &str) -> String {
    let mut width = 3usize;
    let mut run = 0usize;
    for ch in text.chars() {
        if ch == '`' {
            run += 1;
            width = width.max(run + 1);
        } else {
            run = 0;
        }
    }
    "`".repeat(width)
}

pub fn write_artifacts(
    output_dir: &Path,
    report: &PipelineReport,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(output_dir)?;
    fs::write(output_dir.join("candidate.prompt.md"), &report.candidate)?;
    fs::write(output_dir.join("report.md"), render_markdown(report))?;
    fs::write(
        output_dir.join("report.json"),
        serde_json::to_string_pretty(report)?,
    )?;
    fs::write(
        output_dir.join("capabilities.json"),
        serde_json::to_string_pretty(&capability_registry())?,
    )?;
    Ok(())
}

pub fn utc_timestamp() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let days = seconds.div_euclid(86_400);
    let rem = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = rem / 3_600;
    let minute = (rem % 3_600) / 60;
    let second = rem % 60;
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hour, minute, second
    )
}

fn civil_from_days(days_since_epoch: i64) -> (i64, i64, i64) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    year += if month <= 2 { 1 } else { 0 };
    (year, month, day)
}

pub fn sha256_hex(input: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut data = input.to_vec();
    let bit_len = (data.len() as u64) * 8;
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bit_len.to_be_bytes());
    let mut h = [
        0x6a09e667u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    for chunk in data.chunks(64) {
        let mut w = [0u32; 64];
        for (i, word) in chunk.chunks(4).enumerate() {
            w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    h.iter().map(|v| format!("{:08x}", v)).collect()
}
