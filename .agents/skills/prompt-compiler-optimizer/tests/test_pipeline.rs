use std::collections::HashMap;
use prompt_compiler_optimizer::passes::{normalize_terms, reorder_structure, semantic_dedup};
use prompt_compiler_optimizer::types::{AtomicRule, Invariant, PromptIR, SourceSpan};

fn create_sample_v4_ir() -> PromptIR {
    PromptIR {
        version: "4.1.0".to_string(),
        metadata: HashMap::new(),
        sections: vec![],
        semantic_units: vec![],
        atomic_rules: vec![
            AtomicRule {
                id: "R-1".to_string(),
                rule_type: "behavior".to_string(),
                semantics: "Always answer in Vietnamese".to_string(),
                actor: Some("Assistant".to_string()),
                action: Some("answer".to_string()),
                object: None,
                conditions: vec![],
                exceptions: vec![],
                scope: vec!["global".to_string()],
                priority: 100,
                source_spans: vec![SourceSpan {
                    section: "rules".to_string(),
                    line_range: Some("1-2".to_string()),
                    text: "Always answer in Vietnamese".to_string(),
                }],
                confidence: Some(1.0),
                status: "active".to_string(),
            },
            AtomicRule {
                id: "R-2".to_string(),
                rule_type: "behavior".to_string(),
                semantics: "always answer in vietnamese".to_string(),
                actor: Some("Assistant".to_string()),
                action: Some("answer".to_string()),
                object: None,
                conditions: vec![],
                exceptions: vec![],
                scope: vec!["global".to_string()],
                priority: 100,
                source_spans: vec![SourceSpan {
                    section: "rules".to_string(),
                    line_range: Some("5-6".to_string()),
                    text: "always answer in vietnamese".to_string(),
                }],
                confidence: Some(1.0),
                status: "active".to_string(),
            },
            AtomicRule {
                id: "R-3".to_string(),
                rule_type: "behavior".to_string(),
                semantics: "Be polite".to_string(),
                actor: Some("Assistant".to_string()),
                action: Some("behave".to_string()),
                object: None,
                conditions: vec![],
                exceptions: vec![],
                scope: vec!["global".to_string()],
                priority: 500,
                source_spans: vec![SourceSpan {
                    section: "rules".to_string(),
                    line_range: Some("10-11".to_string()),
                    text: "Be polite".to_string(),
                }],
                confidence: Some(1.0),
                status: "active".to_string(),
            },
        ],
        invariants: vec![Invariant {
            id: "INV-01".to_string(),
            description: "Vietnamese language constraint".to_string(),
            rule_ref: "R-1".to_string(),
            required: true,
        }],
    }
}

#[test]
fn test_semantic_dedup_and_provenance_merge() {
    let sample = create_sample_v4_ir();
    let (res, changes) = semantic_dedup::run_pass(sample);
    
    // 1 duplicate rule merged
    assert_eq!(res.atomic_rules.len(), 2);
    assert_eq!(changes.len(), 1);
    
    // Check provenance merged (INV-01)
    let merged_rule = res.atomic_rules.iter().find(|r| r.id == "R-1").unwrap();
    assert_eq!(merged_rule.source_spans.len(), 2);
}

#[test]
fn test_normalize_terms() {
    let mut sample = create_sample_v4_ir();
    sample.atomic_rules = vec![AtomicRule {
        id: "R-FMT".to_string(),
        rule_type: "output".to_string(),
        semantics: "Output in JSON format".to_string(),
        actor: None,
        action: None,
        object: None,
        conditions: vec![],
        exceptions: vec![],
        scope: vec!["output".to_string()],
        priority: 10,
        source_spans: vec![SourceSpan {
            section: "output".to_string(),
            line_range: None,
            text: "Output in JSON format".to_string(),
        }],
        confidence: Some(1.0),
        status: "active".to_string(),
    }];

    let mut synonyms = HashMap::new();
    synonyms.insert("JSON format".to_string(), "valid JSON object".to_string());

    let (res, changes) = normalize_terms::run_pass(sample, Some(&synonyms));
    assert_eq!(res.atomic_rules[0].semantics, "Output in valid JSON object");
    assert_eq!(changes.len(), 1);
}

#[test]
fn test_reorder_structure() {
    let sample = create_sample_v4_ir();
    let (res, _) = reorder_structure::run_pass(sample);
    assert!(res.atomic_rules[0].priority <= res.atomic_rules[1].priority);
}
