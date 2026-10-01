//! Pass 3: Reorder Structure
//! Reorders sections and rules to optimize attention weighting and follow model profile best practices.

use crate::types::{ChangeLogEntry, PromptIR};

pub fn run_pass(mut ir: PromptIR) -> (PromptIR, Vec<ChangeLogEntry>) {
    // Sort rules by priority (0=Critical, 1000=Lowest)
    ir.atomic_rules.sort_by_key(|r| r.priority);
    let changes = vec![ChangeLogEntry {
        change_id: "CH-REORDER-01".to_string(),
        pass: "reorder_structure".to_string(),
        action: "reorder".to_string(),
        source_rule_ids: ir.atomic_rules.iter().map(|r| r.id.clone()).collect(),
        target_rule_ids: ir.atomic_rules.iter().map(|r| r.id.clone()).collect(),
        reason: "Reorder rules by priority score".to_string(),
        risk: "low".to_string(),
        before_hash: "unsorted".to_string(),
        after_hash: "sorted_priority".to_string(),
        provenance_preserved: true,
        verification_refs: vec![],
    }];
    (ir, changes)
}
