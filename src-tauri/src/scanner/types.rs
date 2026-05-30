use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectScanResult {
    pub path: String,
    pub framework: Option<String>,
    pub platforms: Vec<String>,
    pub app_name: Option<String>,
    pub bundle_id: Option<String>,
    pub version: Option<String>,
    pub build_number: Option<String>,
    pub min_os_version: Option<String>,
    pub dependencies: Vec<DetectedDependency>,
    pub permissions: Vec<DetectedPermission>,
    pub files_scanned: Vec<String>,
    pub detected_features: Vec<DetectedFeature>,
    pub risk_flags: Vec<RiskFlag>,
    pub questions: Vec<ConfirmationQuestion>,
    pub document_summaries: Vec<DocumentSummary>,
    pub detected_urls: Vec<DetectedUrl>,
    pub confidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedDependency {
    pub name: String,
    pub version: Option<String>,
    pub source_file: String,
    pub category: Option<String>,
    pub risk: Option<String>,
    pub requires_confirmation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedPermission {
    pub permission: String,
    pub platform: String,
    pub detected_from: String,
    pub plist_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedFeature {
    pub name: String,
    pub source_file: String,
    pub confidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFlag {
    pub flag: String,
    pub reason: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmationQuestion {
    pub id: String,
    pub question: String,
    pub reason: String,
    pub options: Vec<String>,
    pub related_fact_ids: Vec<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSummary {
    pub file_name: String,
    pub first_paragraph: String,
    pub line_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedUrl {
    pub url: String,
    pub kind: String,
    pub source_file: String,
    pub confidence: String,
}
