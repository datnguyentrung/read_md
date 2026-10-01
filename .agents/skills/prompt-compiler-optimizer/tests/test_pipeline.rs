use std::collections::HashMap;
use prompt_compiler_optimizer::passes::{normalize_terms, semantic_dedup};
use prompt_compiler_optimizer::types::{AtomicRule, PromptIR};

fn create_sample_ir() -> PromptIR {
    PromptIR {
        version: "1.0.0".to_string(),
        metadata: HashMap::new(),
        sections: vec![],
        atomic_rules: vec![
            AtomicRule {
                id: "1".to_string(),
                section_id: None,
                rule_type: "behavior".to_string(),
                description: "Always answer in Vietnamese".to_string(),
                priority: "HIGH".to_string(),
                conditions: vec![],
                actions: vec![],
                tags: vec![],
            },
            AtomicRule {
                id: "2".to_string(),
                section_id: None,
                rule_type: "behavior".to_string(),
                description: "always answer in vietnamese".to_string(),
                priority: "HIGH".to_string(),
                conditions: vec![],
                actions: vec![],
                tags: vec![],
            },
            AtomicRule {
                id: "3".to_string(),
                section_id: None,
                rule_type: "behavior".to_string(),
                description: "Be polite".to_string(),
                priority: "LOW".to_string(),
                conditions: vec![],
                actions: vec![],
                tags: vec![],
            },
        ],
    }
}

#[test]
fn test_semantic_dedup() {
    let sample = create_sample_ir();
    let res = semantic_dedup::run_pass(sample);
    assert_eq!(res.atomic_rules.len(), 2);
}

#[test]
fn test_normalize_terms() {
    let mut sample = create_sample_ir();
    sample.atomic_rules = vec![AtomicRule {
        id: "1".to_string(),
        section_id: None,
        rule_type: "format".to_string(),
        description: "Output in JSON format".to_string(),
        priority: "MEDIUM".to_string(),
        conditions: vec![],
        actions: vec![],
        tags: vec![],
    }];

    let mut synonyms = HashMap::new();
    synonyms.insert("JSON format".to_string(), "valid JSON object".to_string());

    let res = normalize_terms::run_pass(sample, Some(&synonyms));
    assert_eq!(res.atomic_rules[0].description, "Output in valid JSON object");
}
