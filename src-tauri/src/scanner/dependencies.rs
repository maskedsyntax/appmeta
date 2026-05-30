use crate::scanner::{ConfirmationQuestion, DetectedDependency, RiskFlag};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
struct RiskEntry {
    category: String,
    risk: String,
    question: String,
}

pub struct DependencyAnalysis {
    pub risk_flags: Vec<RiskFlag>,
    pub questions: Vec<ConfirmationQuestion>,
}

pub fn analyze_dependencies(deps: &[DetectedDependency]) -> DependencyAnalysis {
    let risk_map: std::collections::HashMap<String, RiskEntry> =
        serde_json::from_str(include_str!("../../data/dependency_risks.json"))
            .unwrap_or_default();

    let mut risk_flags = Vec::new();
    let mut questions = Vec::new();
    let mut seen_questions = HashSet::new();

    for dep in deps {
        let normalized = dep.name.replace('-', "_");
        if let Some(entry) = risk_map.get(&normalized).or_else(|| risk_map.get(&dep.name)) {
            risk_flags.push(RiskFlag {
                flag: entry.risk.clone(),
                reason: format!("Dependency '{}' detected", dep.name),
                source: dep.source_file.clone(),
            });

            let q_id = format!("dep_{}", normalized);
            if seen_questions.insert(q_id.clone()) {
                let options = match entry.category.as_str() {
                    "iap" => vec![
                        "Tip jar".into(),
                        "One-time unlock".into(),
                        "Subscription".into(),
                        "Consumable credits".into(),
                        "Other".into(),
                        "None".into(),
                    ],
                    "analytics" | "crash_reporting" | "ads" | "auth" => {
                        vec!["Yes".into(), "No".into(), "Not in production".into()]
                    }
                    _ => vec!["Yes".into(), "No".into(), "Unsure".into()],
                };

                questions.push(ConfirmationQuestion {
                    id: q_id,
                    question: entry.question.clone(),
                    reason: format!(
                        "Detected dependency '{}' in {}",
                        dep.name, dep.source_file
                    ),
                    options,
                    related_fact_ids: vec![format!("dependency:{}", dep.name)],
                    required: entry.category == "iap" || entry.category == "auth",
                });
            }
        }
    }

    DependencyAnalysis {
        risk_flags,
        questions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_iap_dependency() {
        let deps = vec![DetectedDependency {
            name: "in_app_purchase".into(),
            version: Some("^3.0.0".into()),
            source_file: "pubspec.yaml".into(),
            category: None,
            risk: None,
            requires_confirmation: false,
        }];
        let analysis = analyze_dependencies(&deps);
        assert!(!analysis.risk_flags.is_empty());
        assert!(!analysis.questions.is_empty());
    }
}
