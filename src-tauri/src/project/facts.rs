use crate::project::truth_file::*;
use crate::scanner::ProjectScanResult;
use uuid::Uuid;

pub fn create_project_from_scan(scan: ProjectScanResult) -> ProjectTruthFile {
    let now = chrono::Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let app_name = scan
        .app_name
        .clone()
        .unwrap_or_else(|| "Untitled App".to_string());

    let mut project = ProjectTruthFile {
        schema_version: "1.0".to_string(),
        project: ProjectMeta {
            id: id.clone(),
            name: app_name.clone(),
            path: scan.path.clone(),
            created_at: now.clone(),
            updated_at: now,
        },
        app_identity: AppIdentity {
            app_name: app_name.clone(),
            bundle_id: scan.bundle_id.clone().unwrap_or_default(),
            platforms: scan.platforms.clone(),
            version: scan.version.clone().unwrap_or_default(),
            ..Default::default()
        },
        summary: Summary::default(),
        features: Vec::new(),
        privacy: Privacy::default(),
        permissions: Vec::new(),
        monetization: Monetization::default(),
        review: Review::default(),
        ai_usage: AiUsage::default(),
        claims_to_avoid: vec![
            "Guaranteed App Store approval".into(),
            "Medical or therapeutic guarantees".into(),
            "Financial outcome guarantees".into(),
        ],
        source_facts: Vec::new(),
        generated_fields: Vec::new(),
        question_answers: std::collections::HashMap::new(),
        scan_questions: scan.questions.clone(),
    };

    if let Some(framework) = &scan.framework {
        project.source_facts.push(SourceFact {
            id: Uuid::new_v4().to_string(),
            fact: format!("Framework: {framework}"),
            source_type: "scanner".into(),
            source_file: scan.path.clone(),
            confidence: scan.confidence.clone(),
            verified: true,
            status: "verified".into(),
        });
    }

    if let Some(bundle_id) = &scan.bundle_id {
        project.source_facts.push(SourceFact {
            id: Uuid::new_v4().to_string(),
            fact: format!("Bundle ID: {bundle_id}"),
            source_type: "scanner".into(),
            source_file: "ios/Runner/Info.plist".into(),
            confidence: "high".into(),
            verified: true,
            status: "verified".into(),
        });
    }

    for dep in &scan.dependencies {
        project.source_facts.push(SourceFact {
            id: format!("dependency:{}", dep.name),
            fact: format!("Dependency: {} {}", dep.name, dep.version.as_deref().unwrap_or("")),
            source_type: "scanner".into(),
            source_file: dep.source_file.clone(),
            confidence: "high".into(),
            verified: false,
            status: if dep.requires_confirmation {
                "needs_confirmation".into()
            } else {
                "unknown".into()
            },
        });
    }

    for feature in &scan.detected_features {
        project.features.push(Feature {
            name: feature.name.clone(),
            description: String::new(),
            source: feature.source_file.clone(),
            verified: false,
            mention_in_marketing: true,
        });
        project.source_facts.push(SourceFact {
            id: Uuid::new_v4().to_string(),
            fact: format!("Detected feature: {}", feature.name),
            source_type: "scanner".into(),
            source_file: feature.source_file.clone(),
            confidence: feature.confidence.clone(),
            verified: false,
            status: "needs_confirmation".into(),
        });
    }

    for perm in &scan.permissions {
        project.permissions.push(PermissionEntry {
            permission: perm.permission.clone(),
            platform: perm.platform.clone(),
            detected_from: perm.detected_from.clone(),
            purpose: String::new(),
            verified: false,
        });
        project.source_facts.push(SourceFact {
            id: Uuid::new_v4().to_string(),
            fact: format!("Permission: {} ({})", perm.permission, perm.platform),
            source_type: "scanner".into(),
            source_file: perm.detected_from.clone(),
            confidence: "high".into(),
            verified: false,
            status: "needs_confirmation".into(),
        });
    }

    for flag in &scan.risk_flags {
        project.source_facts.push(SourceFact {
            id: Uuid::new_v4().to_string(),
            fact: format!("Risk: {} — {}", flag.flag, flag.reason),
            source_type: "scanner".into(),
            source_file: flag.source.clone(),
            confidence: "medium".into(),
            verified: false,
            status: "needs_confirmation".into(),
        });

        if flag.flag.contains("iap") {
            project.monetization.has_iap = Some(true);
        }
        if flag.flag.contains("analytics") || flag.flag.contains("data_collection") {
            // signal only, user must confirm
        }
    }

    for doc in &scan.document_summaries {
        if !doc.first_paragraph.is_empty() {
            project.source_facts.push(SourceFact {
                id: Uuid::new_v4().to_string(),
                fact: format!("From {}: {}", doc.file_name, doc.first_paragraph),
                source_type: "imported_doc".into(),
                source_file: doc.file_name.clone(),
                confidence: "medium".into(),
                verified: false,
                status: "needs_confirmation".into(),
            });
            if project.summary.short_summary.is_empty() && doc.file_name == "README.md" {
                project.summary.short_summary = doc.first_paragraph.clone();
            }
        }
    }

    if let Some(privacy_url) = scan
        .detected_urls
        .iter()
        .find(|u| u.kind == "privacy_policy")
    {
        if project.privacy.privacy_policy_url.is_empty() {
            project.privacy.privacy_policy_url = privacy_url.url.clone();
        }
        project.source_facts.push(SourceFact {
            id: Uuid::new_v4().to_string(),
            fact: format!(
                "Privacy policy URL detected: {} (confirm this is correct for App Store Connect)",
                privacy_url.url
            ),
            source_type: "scanner".into(),
            source_file: privacy_url.source_file.clone(),
            confidence: privacy_url.confidence.clone(),
            verified: false,
            status: "needs_confirmation".into(),
        });
    }

    project
}

pub fn update_fact_status(project: &mut ProjectTruthFile, fact_id: &str, status: &str) {
    if let Some(fact) = project.source_facts.iter_mut().find(|f| f.id == fact_id) {
        fact.status = status.to_string();
        fact.verified = status == "verified";
    }
    project.touch();
}

pub fn answer_question(
    project: &mut ProjectTruthFile,
    question_id: &str,
    answer: &str,
) {
    project
        .question_answers
        .insert(question_id.to_string(), answer.to_string());

    let fact = SourceFact {
        id: Uuid::new_v4().to_string(),
        fact: format!("Confirmed: {answer}"),
        source_type: "manual".into(),
        source_file: question_id.to_string(),
        confidence: "high".into(),
        verified: true,
        status: "verified".into(),
    };
    project.source_facts.push(fact);

    // Apply common answer side effects
    let lower = answer.to_lowercase();
    if question_id.contains("iap") || lower.contains("subscription") || lower.contains("unlock") {
        project.monetization.has_iap = Some(!lower.contains("none"));
        project.monetization.iap_type = answer.to_string();
    }
    if lower == "yes" || lower == "no" {
        if question_id.contains("analytics") {
            project.privacy.uses_analytics = Some(lower == "yes");
        }
        if question_id.contains("auth") || question_id.contains("account") {
            project.privacy.has_account = Some(lower == "yes");
            project.review.requires_login = Some(lower == "yes");
        }
    }

    project.touch();
}
