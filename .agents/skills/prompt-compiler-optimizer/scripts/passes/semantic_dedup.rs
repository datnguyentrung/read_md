//! Pass 2: Semantic Deduplication
//! Eliminates redundant atomic rules with duplicate or identical semantic intent while merging provenance.

use std::collections::HashSet;
use crate::types::{AtomicRule, ChangeLogEntry, PromptIR};

pub fn run_pass(mut ir: PromptIR) -> (PromptIR, Vec<ChangeLogEntry>) {
    let mut seen = HashSet::new();
    let mut deduped_rules: Vec<AtomicRule> = Vec::new();
    let mut changes = Vec::new();

    for rule in ir.atomic_rules {
        let normalized = rule.semantics.trim().to_lowercase();
        if !seen.contains(&normalized) {
            seen.insert(normalized);
            deduped_rules.push(rule);
        } else {
            // Find existing rule and merge provenance (INV-01)
            if let Some(existing) = deduped_rules.iter_mut().find(|r| r.semantics.trim().to_lowercase() == normalized) {
                existing.source_spans.extend(rule.source_spans.clone());
                changes.push(ChangeLogEntry {
                    change_id: format!("CH-DEDUP-{}", rule.id),
                    pass: "semantic_dedup".to_string(),
                    action: "merge".to_string(),
                    source_rule_ids: vec![existing.id.clone(), rule.id.clone()],
                    target_rule_ids: vec![existing.id.clone()],
                    reason: "Identical semantic content detected and merged".to_string(),
                    risk: "low".to_string(),
                    before_hash: format!("{:x}", rule.semantics.len()),
                    after_hash: format!("{:x}", existing.semantics.len()),
                    provenance_preserved: true,
                    verification_refs: vec![],
                });
            }
        }
    }

    ir.atomic_rules = deduped_rules;
    (ir, changes)
}
