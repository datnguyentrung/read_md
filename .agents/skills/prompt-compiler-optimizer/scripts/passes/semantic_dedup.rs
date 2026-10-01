//! Pass: Semantic Deduplication
//! Eliminates redundant atomic rules with duplicate or identical semantic intent.

use std::collections::HashSet;
use crate::types::{AtomicRule, PromptIR};

pub fn run_pass(mut ir: PromptIR) -> PromptIR {
    let mut seen = HashSet::new();
    let mut deduped_rules: Vec<AtomicRule> = Vec::new();

    for rule in ir.atomic_rules {
        let normalized = rule.description.trim().to_lowercase();
        if !seen.contains(&normalized) {
            seen.insert(normalized);
            deduped_rules.push(rule);
        }
    }

    ir.atomic_rules = deduped_rules;
    ir
}
