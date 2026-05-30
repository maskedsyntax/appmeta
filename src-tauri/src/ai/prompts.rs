use crate::project::constants::{field_constraints, field_limits};
use crate::project::truth_file::ProjectTruthFile;
use crate::storage::AppSettings;

pub fn build_system_prompt() -> String {
    r#"You are generating App Store Connect submission content.

Use ONLY the provided Project Truth File.
Do not invent features, integrations, privacy practices, medical claims, pricing, awards, platform support, or App Store policies.

If required information is missing, return JSON with missing_info array containing questions.
If a claim is uncertain, include warnings explaining why.

Write clearly and professionally.
Avoid exaggerated claims.
Avoid guaranteed outcomes.
Avoid claiming medical, therapeutic, financial, or legal benefits unless explicitly confirmed and safe.

Respond with ONLY valid JSON in this format:
{
  "field": "<field_name>",
  "value": "<generated text>",
  "alternatives": ["<alt1>", "<alt2>"],
  "confidence": "high|medium|low",
  "source_facts_used": ["<fact id or description>"],
  "warnings": ["<warning>"],
  "missing_info": ["<question if info missing>"]
}"#
        .to_string()
}

pub fn build_user_prompt(
    project: &ProjectTruthFile,
    field_name: &str,
    settings: &AppSettings,
) -> String {
    let truth_json =
        serde_json::to_string_pretty(project).unwrap_or_else(|_| "{}".to_string());
    let constraints = field_constraints(field_name);
    let max_chars = field_limits()
        .get(field_name)
        .map(|m| m.to_string())
        .unwrap_or_else(|| "none".to_string());

    format!(
        r#"Project Truth File:
{truth_json}

Field: {field_name}

Field Constraints:
{constraints}
Maximum characters: {max_chars}

Tone: {}

Things not to claim:
{}

If information is missing for this field, populate missing_info with specific questions instead of inventing content."#,
        settings.tone,
        project.claims_to_avoid.join(", ")
    )
}

pub fn parse_ai_response(field_name: &str, content: &str) -> crate::project::truth_file::GeneratedField {
    let trimmed = content.trim();

    if trimmed.starts_with("MISSING_INFO:") {
        return crate::project::truth_file::GeneratedField {
            field: field_name.to_string(),
            value: String::new(),
            alternatives: Vec::new(),
            character_count: 0,
            max_characters: field_limits().get(field_name).copied(),
            confidence: "low".to_string(),
            source_facts_used: Vec::new(),
            warnings: Vec::new(),
            missing_info: vec![trimmed.trim_start_matches("MISSING_INFO:").trim().to_string()],
        };
    }

    if trimmed.starts_with("NEEDS_CONFIRMATION:") {
        return crate::project::truth_file::GeneratedField {
            field: field_name.to_string(),
            value: String::new(),
            alternatives: Vec::new(),
            character_count: 0,
            max_characters: field_limits().get(field_name).copied(),
            confidence: "low".to_string(),
            source_facts_used: Vec::new(),
            warnings: vec![trimmed
                .trim_start_matches("NEEDS_CONFIRMATION:")
                .trim()
                .to_string()],
            missing_info: Vec::new(),
        };
    }

    // Try to extract JSON from markdown code block
    let json_str = if trimmed.starts_with("```") {
        trimmed
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
    } else {
        trimmed
    };

    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(json_str) {
        let value = parsed
            .get("value")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let alternatives: Vec<String> = parsed
            .get("alternatives")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let confidence = parsed
            .get("confidence")
            .and_then(|v| v.as_str())
            .unwrap_or("medium")
            .to_string();
        let source_facts_used: Vec<String> = parsed
            .get("source_facts_used")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let warnings: Vec<String> = parsed
            .get("warnings")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let missing_info: Vec<String> = parsed
            .get("missing_info")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        return crate::project::truth_file::GeneratedField {
            field: field_name.to_string(),
            character_count: value.chars().count(),
            max_characters: field_limits().get(field_name).copied(),
            value,
            alternatives,
            confidence,
            source_facts_used,
            warnings,
            missing_info,
        };
    }

    crate::project::truth_file::GeneratedField {
        field: field_name.to_string(),
        value: trimmed.to_string(),
        alternatives: Vec::new(),
        character_count: trimmed.chars().count(),
        max_characters: field_limits().get(field_name).copied(),
        confidence: "low".to_string(),
        source_facts_used: Vec::new(),
        warnings: vec!["AI response was not valid JSON — using raw text.".to_string()],
        missing_info: Vec::new(),
    }
}

pub fn ai_context_preview_items(project: &ProjectTruthFile) -> Vec<String> {
    vec![
        format!("App name: {}", project.app_identity.app_name),
        format!("Bundle ID: {}", project.app_identity.bundle_id),
        format!("Short summary: {}", project.summary.short_summary),
        format!("Features: {} confirmed", project.features.iter().filter(|f| f.verified).count()),
        format!("Privacy answers: analytics={:?}, account={:?}", project.privacy.uses_analytics, project.privacy.has_account),
        format!("Monetization: IAP={:?}", project.monetization.has_iap),
        format!("Review: login required={:?}", project.review.requires_login),
        "Source code will NOT be sent.".to_string(),
    ]
}
