use std::collections::HashMap;

pub fn field_limits() -> HashMap<&'static str, usize> {
    HashMap::from([
        ("app_name", 30),
        ("subtitle", 30),
        ("promotional_text", 170),
        ("keywords", 100),
    ])
}

pub fn field_constraints(field: &str) -> &'static str {
    match field {
        "subtitle" => "Maximum 30 characters. Do not repeat the app name. Clear value proposition.",
        "promotional_text" => {
            "Maximum 170 characters. Can be updated without a new app version. Highlight current feature."
        }
        "description" => {
            "Full app description. Be specific about verified features only. No exaggerated claims."
        }
        "keywords" => {
            "Maximum 100 characters total. Comma-separated, no spaces after commas. No repeat of app name."
        }
        "whats_new" => "Release notes for this version. Mention verified changes only.",
        "review_notes" => {
            "Instructions for App Review team. Include demo account if login required. Explain IAP testing."
        }
        "privacy_summary" => {
            "Plain-language privacy summary based on confirmed answers only. Do not auto-answer Apple privacy labels."
        }
        "privacy_policy_outline" => "Outline for privacy policy based on confirmed data practices.",
        "iap_display_name" => "Display name for in-app purchase product.",
        "iap_description" => "Description of what the IAP unlocks or provides.",
        "iap_review_notes" => "Instructions for reviewer to test IAP or subscriptions.",
        "support_url_guidance" => "Guidance for support URL content.",
        "marketing_url_guidance" => "Guidance for marketing URL content.",
        _ => "Generate App Store Connect ready content from verified facts only.",
    }
}

#[allow(dead_code)]
pub fn all_generatable_fields() -> Vec<&'static str> {
    vec![
        "subtitle",
        "promotional_text",
        "description",
        "keywords",
        "whats_new",
        "review_notes",
        "privacy_summary",
        "privacy_policy_outline",
        "iap_display_name",
        "iap_description",
        "iap_review_notes",
        "support_url_guidance",
        "marketing_url_guidance",
    ]
}
