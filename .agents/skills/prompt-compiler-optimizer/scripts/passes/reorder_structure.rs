//! Pass: Reorder Structure
//! Reorders sections and rules to optimize attention weighting and follow model profile best practices.

use crate::types::PromptIR;

fn priority_weight(priority: &str) -> i32 {
    match priority.to_uppercase().as_str() {
        "CRITICAL" => 0,
        "HIGH" => 1,
        "MEDIUM" => 2,
        "LOW" => 3,
        _ => 2,
    }
}

pub fn run_pass(mut ir: PromptIR) -> PromptIR {
    ir.atomic_rules.sort_by_key(|r| priority_weight(&r.priority));
    ir
}
