//! Pass 1: Normalize Terms
//! Standardizes naming conventions, technical terminology, and casing across prompt rules.

use std::collections::HashMap;
use crate::types::{ChangeLogEntry, PromptIR};

pub fn run_pass(mut ir: PromptIR, synonyms: Option<&HashMap<String, String>>) -> (PromptIR, Vec<ChangeLogEntry>) {
    let mut changes = Vec::new();
    if let Some(synonym_map) = synonyms {
        for rule in &mut ir.atomic_rules {
            let orig_semantics = rule.semantics.clone();
            let mut updated = orig_semantics.clone();
            for (k, v) in synonym_map {
                updated = updated.replace(k, v);
            }
            if updated != orig_semantics {
                rule.semantics = updated.clone();
                changes.push(ChangeLogEntry {
                    change_id: format!("CH-NORM-{}", rule.id),
                    pass: "normalize_terms".to_string(),
                    action: "normalize".to_string(),
                    source_rule_ids: vec![rule.id.clone()],
                    target_rule_ids: vec![rule.id.clone()],
                    reason: "Canonicalize terminology".to_string(),
                    risk: "low".to_string(),
                    before_hash: format!("{:x}", orig_semantics.len()),
                    after_hash: format!("{:x}", updated.len()),
                    provenance_preserved: true,
                    verification_refs: vec![],
                });
            }
        }
    }
    (ir, changes)
}
