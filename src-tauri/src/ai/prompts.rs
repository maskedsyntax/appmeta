use crate::project::constants::{field_constraints, field_limits};
use crate::project::truth_file::{GeneratedField, ProjectTruthFile};
use crate::storage::AppSettings;

pub fn build_system_prompt() -> String {
    r#"You generate App Store Connect submission copy.

Rules:
- Use ONLY the project context provided. Do not invent features, pricing, awards, or policies.
- The "value" field MUST contain the actual text to paste into App Store Connect. Never leave it empty.
- If facts are incomplete, still write your best draft from what is available (app name, summary, README facts, detected features).
- Put caveats in "warnings" and follow-up questions in "missing_info" — but always fill "value" with usable draft text.
- Avoid exaggerated claims, medical/financial guarantees, and unverified platform support.

Respond with ONLY a raw JSON object (no markdown fences, no commentary):
{
  "field": "<field_name>",
  "value": "<the actual App Store text>",
  "alternatives": ["<optional alt>"],
  "confidence": "high|medium|low",
  "source_facts_used": ["<fact>"],
  "warnings": ["<caveat>"],
  "missing_info": ["<question if something is unclear>"]
}"#
        .to_string()
}

pub fn build_compact_context(project: &ProjectTruthFile) -> String {
    let app_name = if project.app_identity.app_name.is_empty() {
        project.project.name.clone()
    } else {
        project.app_identity.app_name.clone()
    };

    let mut out = String::new();
    out.push_str(&format!("App name: {app_name}\n"));
    if !project.app_identity.bundle_id.is_empty() {
        out.push_str(&format!("Bundle ID: {}\n", project.app_identity.bundle_id));
    }
    if !project.app_identity.version.is_empty() {
        out.push_str(&format!("Version: {}\n", project.app_identity.version));
    }
    if !project.app_identity.platforms.is_empty() {
        out.push_str(&format!(
            "Platforms: {}\n",
            project.app_identity.platforms.join(", ")
        ));
    }
    if !project.app_identity.primary_category.is_empty() {
        out.push_str(&format!(
            "Category: {}\n",
            project.app_identity.primary_category
        ));
    }
    if !project.summary.short_summary.is_empty() {
        out.push_str(&format!("\nShort summary:\n{}\n", project.summary.short_summary));
    }
    if !project.summary.long_summary.is_empty() {
        out.push_str(&format!("\nLong summary:\n{}\n", project.summary.long_summary));
    }
    if !project.features.is_empty() {
        out.push_str("\nFeatures:\n");
        for f in &project.features {
            let tag = if f.verified { "verified" } else { "detected" };
            out.push_str(&format!("- {} ({tag}): {}\n", f.name, f.description));
        }
    }
    if !project.source_facts.is_empty() {
        out.push_str("\nSource facts:\n");
        for fact in project.source_facts.iter().take(30) {
            out.push_str(&format!("- [{}] {}\n", fact.confidence, fact.fact));
        }
    }
    if !project.question_answers.is_empty() {
        out.push_str("\nConfirmed answers:\n");
        for (_, answer) in &project.question_answers {
            out.push_str(&format!("- {answer}\n"));
        }
    }
    if !project.claims_to_avoid.is_empty() {
        out.push_str(&format!(
            "\nDo not claim: {}\n",
            project.claims_to_avoid.join(", ")
        ));
    }
    out
}

pub fn build_user_prompt(
    project: &ProjectTruthFile,
    field_name: &str,
    settings: &AppSettings,
) -> String {
    let context = build_compact_context(project);
    let constraints = field_constraints(field_name);
    let max_chars = field_limits()
        .get(field_name)
        .map(|m| m.to_string())
        .unwrap_or_else(|| "none".to_string());

    format!(
        r#"Project context:
{context}

Generate field: {field_name}

Constraints: {constraints}
Character limit: {max_chars}
Tone: {}

Remember: "value" must contain the final text — never empty."#,
        settings.tone
    )
}

/// Pull a JSON object out of model output (handles fences, preamble, trailing text).
pub fn extract_json_text(content: &str) -> Option<String> {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return None;
    }

    if trimmed.starts_with('{') {
        if let Some(end) = find_json_object_end(trimmed) {
            return Some(trimmed[..=end].to_string());
        }
    }

    if let Some(start) = trimmed.find("```") {
        let after_fence = &trimmed[start + 3..];
        let after_fence = after_fence
            .strip_prefix("json")
            .unwrap_or(after_fence)
            .trim_start();
        if let Some(end) = after_fence.find("```") {
            let inner = after_fence[..end].trim();
            if inner.starts_with('{') {
                return Some(inner.to_string());
            }
        }
    }

    if let Some(start) = trimmed.find('{') {
        let slice = &trimmed[start..];
        if let Some(end) = find_json_object_end(slice) {
            return Some(slice[..=end].to_string());
        }
    }

    None
}

fn find_json_object_end(s: &str) -> Option<usize> {
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escape = false;

    for (i, ch) in s.char_indices() {
        if escape {
            escape = false;
            continue;
        }
        match ch {
            '\\' if in_string => escape = true,
            '"' => in_string = !in_string,
            '{' if !in_string => depth += 1,
            '}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn string_array(value: &serde_json::Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

fn string_field(value: &serde_json::Value, key: &str) -> String {
    value
        .get(key)
        .and_then(|v| match v {
            serde_json::Value::String(s) => Some(s.clone()),
            serde_json::Value::Number(n) => Some(n.to_string()),
            serde_json::Value::Bool(b) => Some(b.to_string()),
            _ => None,
        })
        .unwrap_or_default()
}

pub fn parse_ai_response(field_name: &str, content: &str) -> GeneratedField {
    let trimmed = content.trim();

    if trimmed.starts_with("MISSING_INFO:") {
        let question = trimmed.trim_start_matches("MISSING_INFO:").trim();
        return empty_field_with_missing(field_name, vec![question.to_string()]);
    }

    if trimmed.starts_with("NEEDS_CONFIRMATION:") {
        return GeneratedField {
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

    if let Some(json_str) = extract_json_text(trimmed) {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json_str) {
            let mut value = string_field(&parsed, "value");
            let alternatives = string_array(&parsed, "alternatives");
            let confidence = string_field(&parsed, "confidence");
            let confidence = if confidence.is_empty() {
                "medium".to_string()
            } else {
                confidence
            };
            let source_facts_used = string_array(&parsed, "source_facts_used");
            let warnings = string_array(&parsed, "warnings");
            let missing_info = string_array(&parsed, "missing_info");

            if value.is_empty() {
                if let Some(alt) = alternatives.first() {
                    value = alt.clone();
                }
            }

            if !value.is_empty() {
                return GeneratedField {
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

            if !missing_info.is_empty() {
                return empty_field_with_missing(field_name, missing_info);
            }
        }
    }

    // Plain text response — use directly if it looks like copy, not JSON debris
    if !trimmed.is_empty() && !trimmed.starts_with('{') {
        return GeneratedField {
            field: field_name.to_string(),
            value: trimmed.to_string(),
            alternatives: Vec::new(),
            character_count: trimmed.chars().count(),
            max_characters: field_limits().get(field_name).copied(),
            confidence: "medium".to_string(),
            source_facts_used: Vec::new(),
            warnings: Vec::new(),
            missing_info: Vec::new(),
        };
    }

    GeneratedField {
        field: field_name.to_string(),
        value: String::new(),
        alternatives: Vec::new(),
        character_count: 0,
        max_characters: field_limits().get(field_name).copied(),
        confidence: "low".to_string(),
        source_facts_used: Vec::new(),
        warnings: vec!["Could not parse AI response. Try again or switch to gpt-4o-mini.".to_string()],
        missing_info: Vec::new(),
    }
}

fn empty_field_with_missing(field_name: &str, missing_info: Vec<String>) -> GeneratedField {
    GeneratedField {
        field: field_name.to_string(),
        value: String::new(),
        alternatives: Vec::new(),
        character_count: 0,
        max_characters: field_limits().get(field_name).copied(),
        confidence: "low".to_string(),
        source_facts_used: Vec::new(),
        warnings: vec!["AI could not draft text without more confirmed facts.".to_string()],
        missing_info,
    }
}

pub fn ai_context_preview_items(project: &ProjectTruthFile) -> Vec<String> {
    vec![
        format!("App name: {}", project.app_identity.app_name),
        format!("Bundle ID: {}", project.app_identity.bundle_id),
        format!("Short summary: {}", project.summary.short_summary),
        format!(
            "Features: {} confirmed",
            project.features.iter().filter(|f| f.verified).count()
        ),
        format!(
            "Privacy answers: analytics={:?}, account={:?}",
            project.privacy.uses_analytics, project.privacy.has_account
        ),
        format!("Monetization: IAP={:?}", project.monetization.has_iap),
        format!("Review: login required={:?}", project.review.requires_login),
        "Source code will NOT be sent.".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_json_from_code_fence() {
        let input = r#"Here is the result:
```json
{"field":"subtitle","value":"Track your habits","confidence":"high","alternatives":[],"source_facts_used":[],"warnings":[],"missing_info":[]}
```"#;
        let parsed = parse_ai_response("subtitle", input);
        assert_eq!(parsed.value, "Track your habits");
    }

    #[test]
    fn uses_alternative_when_value_empty() {
        let input = r#"{"field":"subtitle","value":"","alternatives":["Daily habit tracker"],"confidence":"low","source_facts_used":[],"warnings":[],"missing_info":["What is the app name?"]}"#;
        let parsed = parse_ai_response("subtitle", input);
        assert_eq!(parsed.value, "Daily habit tracker");
    }

    #[test]
    fn plain_text_fallback() {
        let parsed = parse_ai_response("subtitle", "Simple daily habit tracking");
        assert_eq!(parsed.value, "Simple daily habit tracking");
    }

    #[test]
    fn finds_json_with_preamble() {
        let text = r#"Sure! {"field":"keywords","value":"habit,tracker,daily","confidence":"medium","alternatives":[],"source_facts_used":[],"warnings":[],"missing_info":[]}"#;
        let parsed = parse_ai_response("keywords", text);
        assert_eq!(parsed.value, "habit,tracker,daily");
    }
}
