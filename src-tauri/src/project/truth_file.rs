use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTruthFile {
    pub schema_version: String,
    pub project: ProjectMeta,
    pub app_identity: AppIdentity,
    pub summary: Summary,
    pub features: Vec<Feature>,
    pub privacy: Privacy,
    pub permissions: Vec<PermissionEntry>,
    pub monetization: Monetization,
    pub review: Review,
    pub ai_usage: AiUsage,
    pub claims_to_avoid: Vec<String>,
    pub source_facts: Vec<SourceFact>,
    #[serde(default)]
    pub generated_fields: Vec<GeneratedField>,
    #[serde(default)]
    pub question_answers: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub scan_questions: Vec<crate::scanner::ConfirmationQuestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub id: String,
    pub name: String,
    pub path: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppIdentity {
    pub app_name: String,
    pub bundle_id: String,
    pub sku: String,
    pub platforms: Vec<String>,
    pub primary_category: String,
    pub secondary_category: String,
    pub version: String,
    pub copyright: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Summary {
    pub short_summary: String,
    pub long_summary: String,
    pub target_audience: Vec<String>,
    pub primary_use_cases: Vec<String>,
    pub not_for: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub source: String,
    pub verified: bool,
    pub mention_in_marketing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Privacy {
    pub has_account: Option<bool>,
    pub collects_personal_data: Option<bool>,
    pub stores_data_locally: Option<bool>,
    pub uses_cloud_sync: Option<bool>,
    pub uses_analytics: Option<bool>,
    pub uses_crash_reporting: Option<bool>,
    pub uses_ads: Option<bool>,
    pub uses_tracking: Option<bool>,
    pub uses_third_party_sdks: Vec<String>,
    pub privacy_policy_url: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionEntry {
    pub permission: String,
    pub platform: String,
    pub detected_from: String,
    pub purpose: String,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Monetization {
    pub is_free: Option<bool>,
    pub is_paid_upfront: Option<bool>,
    pub has_iap: Option<bool>,
    pub iap_type: String,
    pub subscriptions: bool,
    pub product_ids: Vec<String>,
    pub pricing_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Review {
    pub requires_login: Option<bool>,
    pub demo_account_available: Option<bool>,
    pub demo_username: String,
    pub demo_password: String,
    pub review_notes: String,
    pub testing_steps: Vec<String>,
    pub special_instructions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AiUsage {
    pub uses_ai: Option<bool>,
    pub ai_features: Vec<String>,
    pub user_generated_content: Option<bool>,
    pub moderation_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFact {
    pub id: String,
    pub fact: String,
    pub source_type: String,
    pub source_file: String,
    pub confidence: String,
    pub verified: bool,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedField {
    pub field: String,
    pub value: String,
    pub alternatives: Vec<String>,
    pub character_count: usize,
    pub max_characters: Option<usize>,
    pub confidence: String,
    pub source_facts_used: Vec<String>,
    pub warnings: Vec<String>,
    pub missing_info: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub severity: String,
    pub field: String,
    pub message: String,
}

impl ProjectTruthFile {
    #[allow(dead_code)]
    pub fn new_empty(name: String, path: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        let id = uuid::Uuid::new_v4().to_string();
        Self {
            schema_version: "1.0".to_string(),
            project: ProjectMeta {
                id,
                name: name.clone(),
                path,
                created_at: now.clone(),
                updated_at: now,
            },
            app_identity: AppIdentity {
                app_name: name,
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
            scan_questions: Vec::new(),
        }
    }

    pub fn touch(&mut self) {
        self.project.updated_at = chrono::Utc::now().to_rfc3339();
    }
}
