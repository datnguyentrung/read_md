use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Source Span for 100% Provenance tracking (INV-01)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceSpan {
    pub section: String,
    #[serde(default)]
    pub line_range: Option<String>,
    pub text: String,
}

/// Atomic Rule definition matching V4.1 Schema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AtomicRule {
    pub id: String,
    #[serde(rename = "type")]
    pub rule_type: String, // behavior, constraint, decision, knowledge, example, output
    pub semantics: String,
    #[serde(default)]
    pub actor: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub object: Option<String>,
    #[serde(default)]
    pub conditions: Vec<String>,
    #[serde(default)]
    pub exceptions: Vec<String>,
    pub scope: Vec<String>,
    pub priority: i32, // 0 = Highest, 1000 = Lowest
    pub source_spans: Vec<SourceSpan>,
    #[serde(default)]
    pub confidence: Option<f64>,
    #[serde(default = "default_rule_status")]
    pub status: String, // active, externalized, pruned, needs_review
}

fn default_rule_status() -> String {
    "active".to_string()
}

/// Semantic Unit segmented from prompt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticUnit {
    pub id: String,
    pub text: String,
    pub start: usize,
    pub end: usize,
    #[serde(default)]
    pub section_path: Vec<String>,
    #[serde(default)]
    pub role_hint: Option<String>,
}

/// Structural Section of Prompt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Section {
    pub id: String,
    pub title: String,
    pub order: i32,
    #[serde(default)]
    pub role_hint: Option<String>,
    pub content: String,
}

/// Hard Invariant definition (INV-03 / INV-04)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Invariant {
    pub id: String,
    pub description: String,
    pub rule_ref: String,
    #[serde(default = "default_true")]
    pub required: bool,
}

fn default_true() -> bool {
    true
}

/// Root Prompt Intermediate Representation (PromptIR)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptIR {
    pub version: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub sections: Vec<Section>,
    #[serde(default)]
    pub semantic_units: Vec<SemanticUnit>,
    pub atomic_rules: Vec<AtomicRule>,
    #[serde(default)]
    pub invariants: Vec<Invariant>,
}

/// Semantic Graph Nodes & Edges
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub category: String,
    #[serde(default)]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub relation: String, // same_as, subsumes, conflicts_with, depends_on, refines, triggers, example_of, exception_to
    #[serde(default)]
    pub weight: Option<f64>,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// Pass Configuration & Plan
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassSpec {
    pub name: String,
    pub enabled: bool,
    pub risk: String, // low, medium, high, operational_high
    #[serde(default)]
    pub order: Option<i32>,
    #[serde(default)]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub rationale: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OptimizationPlan {
    #[serde(default = "default_plan_status")]
    pub status: String,
    pub decision: String, // KEEP, LIGHT, STRUCTURAL, MAJOR, EXTERNALIZE, MODULARIZE
    pub passes: Vec<PassSpec>,
    pub hard_invariants: Vec<String>,
    #[serde(default)]
    pub token_target: Option<HashMap<String, serde_json::Value>>,
}

fn default_plan_status() -> String {
    "PLANNED".to_string()
}

/// ChangeLog entry for individual optimization actions (Appendix D)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChangeLogEntry {
    pub change_id: String,
    pub pass: String,
    pub action: String, // merge, generalize, prune, reorder, externalize, extract_tree
    pub source_rule_ids: Vec<String>,
    pub target_rule_ids: Vec<String>,
    pub reason: String,
    pub risk: String,
    pub before_hash: String,
    pub after_hash: String,
    pub provenance_preserved: bool,
    #[serde(default)]
    pub verification_refs: Vec<String>,
}

/// Verification Report (Appendix E)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationReport {
    pub timestamp: String,
    pub status: String, // RELEASED, NEEDS_REVIEW, UNVERIFIED, ROLLED_BACK
    pub assurance_level: String, // VERIFIED_STATIC, VERIFIED_OFFLINE, VERIFIED_CONTRACT, VERIFIED_MOCK, VERIFIED_E2E
    pub verification_scope: String,
    #[serde(default)]
    pub verification_debt: Vec<String>,
    pub static_gate: String, // PASS, FAIL, NEEDS_REVIEW
    pub behavior_gate: String, // PASS, FAIL, SKIPPED, NEEDS_REVIEW
    pub metrics_delta: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub invariants: Vec<serde_json::Value>,
    #[serde(default)]
    pub test_results: Vec<serde_json::Value>,
}

/// Full Release Bundle (Appendix E)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReleaseBundle {
    pub status: String,
    pub decision: String,
    pub assurance_level: String,
    pub verification_scope: String,
    #[serde(default)]
    pub verification_debt: Vec<String>,
    pub source: HashMap<String, String>,
    pub optimizer: HashMap<String, String>,
    pub analysis: HashMap<String, String>,
    pub candidate: HashMap<String, String>,
    pub metrics: HashMap<String, serde_json::Value>,
    pub verification: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub rollback: HashMap<String, String>,
}
