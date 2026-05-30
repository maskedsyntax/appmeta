use crate::project::constants::field_limits;
use crate::project::truth_file::{GeneratedField, ProjectTruthFile, ValidationWarning};

pub fn validate_project(project: &ProjectTruthFile) -> Vec<ValidationWarning> {
    let mut warnings = Vec::new();

    validate_completeness(project, &mut warnings);
    validate_generated_fields(project, &mut warnings);
    validate_consistency(project, &mut warnings);
    validate_content(project, &mut warnings);

    warnings
}

fn validate_completeness(project: &ProjectTruthFile, warnings: &mut Vec<ValidationWarning>) {
    if project.app_identity.app_name.is_empty() {
        warnings.push(ValidationWarning {
            severity: "error".into(),
            field: "app_name".into(),
            message: "App name is required.".into(),
        });
    }
    if project.privacy.privacy_policy_url.is_empty() {
        warnings.push(ValidationWarning {
            severity: "warning".into(),
            field: "privacy_policy_url".into(),
            message: "Privacy policy URL is missing.".into(),
        });
    }
    if project.review.requires_login == Some(true)
        && project.review.demo_account_available != Some(true)
        && project.review.demo_username.is_empty()
    {
        warnings.push(ValidationWarning {
            severity: "error".into(),
            field: "demo_account".into(),
            message: "Login required but no demo account provided for App Review.".into(),
        });
    }
    if project.monetization.has_iap == Some(true) {
        let has_iap_field = project
            .generated_fields
            .iter()
            .any(|f| f.field.starts_with("iap_") && !f.value.is_empty());
        if !has_iap_field && project.monetization.iap_type.is_empty() {
            warnings.push(ValidationWarning {
                severity: "warning".into(),
                field: "iap".into(),
                message: "IAP detected but IAP metadata or type is missing.".into(),
            });
        }
    }
}

fn validate_generated_fields(project: &ProjectTruthFile, warnings: &mut Vec<ValidationWarning>) {
    for field in &project.generated_fields {
        if let Some(max) = field.max_characters {
            if field.character_count > max {
                warnings.push(ValidationWarning {
                    severity: "error".into(),
                    field: field.field.clone(),
                    message: format!(
                        "{} exceeds limit: {} / {} characters",
                        field.field, field.character_count, max
                    ),
                });
            }
        }
        for w in &field.warnings {
            warnings.push(ValidationWarning {
                severity: "warning".into(),
                field: field.field.clone(),
                message: w.clone(),
            });
        }
        for m in &field.missing_info {
            warnings.push(ValidationWarning {
                severity: "info".into(),
                field: field.field.clone(),
                message: m.clone(),
            });
        }
    }

    let required_fields = ["subtitle", "description", "keywords"];
    for req in required_fields {
        let has = project
            .generated_fields
            .iter()
            .any(|f| f.field == req && !f.value.is_empty());
        if !has {
            warnings.push(ValidationWarning {
                severity: "warning".into(),
                field: req.into(),
                message: format!("{req} has not been generated yet."),
            });
        }
    }
}

fn validate_consistency(project: &ProjectTruthFile, warnings: &mut Vec<ValidationWarning>) {
    let has_analytics_dep = project.source_facts.iter().any(|f| {
        f.fact.to_lowercase().contains("firebase_analytics")
            || f.fact.to_lowercase().contains("mixpanel")
            || f.fact.to_lowercase().contains("amplitude")
    });
    if has_analytics_dep && project.privacy.uses_analytics == Some(false) {
        warnings.push(ValidationWarning {
            severity: "error".into(),
            field: "privacy.uses_analytics".into(),
            message: "Analytics SDK detected but privacy says no analytics.".into(),
        });
    }

    let has_backend = project.source_facts.iter().any(|f| {
        f.fact.to_lowercase().contains("supabase")
            || f.fact.to_lowercase().contains("firebase_core")
    });
    if has_backend && project.privacy.stores_data_locally == Some(true) {
        warnings.push(ValidationWarning {
            severity: "warning".into(),
            field: "privacy.stores_data_locally".into(),
            message: "Backend SDK detected but app claims local-only storage.".into(),
        });
    }

    for perm in &project.permissions {
        if perm.purpose.is_empty() && !perm.verified {
            warnings.push(ValidationWarning {
                severity: "warning".into(),
                field: format!("permission:{}", perm.permission),
                message: format!(
                    "{} permission detected but purpose not documented.",
                    perm.permission
                ),
            });
        }
    }
}

fn validate_content(project: &ProjectTruthFile, warnings: &mut Vec<ValidationWarning>) {
    let health_keywords = ["cure", "treat", "diagnose", "therapy", "medical"];
    for field in &project.generated_fields {
        let lower = field.value.to_lowercase();
        for kw in health_keywords {
            if lower.contains(kw) {
                warnings.push(ValidationWarning {
                    severity: "warning".into(),
                    field: field.field.clone(),
                    message: format!(
                        "Possible health/medical claim ('{kw}') — verify wording is safe."
                    ),
                });
            }
        }
        if field.field == "description" && field.value.len() < 100 {
            warnings.push(ValidationWarning {
                severity: "warning".into(),
                field: "description".into(),
                message: "Description may be too vague (under 100 characters).".into(),
            });
        }
        if field.field == "keywords" && field.value.contains(", ") {
            warnings.push(ValidationWarning {
                severity: "info".into(),
                field: "keywords".into(),
                message: "Keywords contain spaces after commas — Apple prefers no spaces.".into(),
            });
        }
        if field.field == "subtitle"
            && !project.app_identity.app_name.is_empty()
            && lower.contains(&project.app_identity.app_name.to_lowercase())
        {
            warnings.push(ValidationWarning {
                severity: "info".into(),
                field: "subtitle".into(),
                message: "Subtitle repeats app name — consider distinct wording.".into(),
            });
        }
    }
}

#[allow(dead_code)]
pub fn apply_field_limits(field_name: &str, value: &str) -> GeneratedField {
    let max = field_limits().get(field_name).copied();
    GeneratedField {
        field: field_name.to_string(),
        value: value.to_string(),
        alternatives: Vec::new(),
        character_count: value.chars().count(),
        max_characters: max,
        confidence: "high".to_string(),
        source_facts_used: Vec::new(),
        warnings: Vec::new(),
        missing_info: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::truth_file::ProjectTruthFile;

    #[test]
    fn detects_missing_demo_account() {
        let mut p = ProjectTruthFile::new_empty("Test".into(), "/tmp".into());
        p.review.requires_login = Some(true);
        let warnings = validate_project(&p);
        assert!(warnings.iter().any(|w| w.field == "demo_account"));
    }

    #[test]
    fn char_limit_subtitle() {
        let field = apply_field_limits("subtitle", &"a".repeat(35));
        assert_eq!(field.max_characters, Some(30));
        assert!(field.character_count > 30);
    }
}
