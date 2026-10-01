//! Pass 7A: Externalization
//! Extracts static datasets, large few-shot examples, or documentation into external references/tools.

use crate::types::{ChangeLogEntry, PromptIR};

pub fn run_pass(ir: PromptIR) -> (PromptIR, Vec<ChangeLogEntry>) {
    (ir, vec![])
}
