//! Pass: Normalize Terms
//! Standardizes naming conventions, technical terminology, and casing across prompt rules.

use std::collections::HashMap;
use crate::types::PromptIR;

pub fn run_pass(mut ir: PromptIR, synonyms: Option<&HashMap<String, String>>) -> PromptIR {
    if let Some(synonym_map) = synonyms {
        for rule in &mut ir.atomic_rules {
            for (k, v) in synonym_map {
                rule.description = rule.description.replace(k, v);
            }
        }
    }
    ir
}
