export interface ProjectMeta {
  id: string;
  name: string;
  path: string;
  created_at: string;
  updated_at: string;
}

export interface AppIdentity {
  app_name: string;
  bundle_id: string;
  sku: string;
  platforms: string[];
  primary_category: string;
  secondary_category: string;
  version: string;
  copyright: string;
}

export interface Summary {
  short_summary: string;
  long_summary: string;
  target_audience: string[];
  primary_use_cases: string[];
  not_for: string[];
}

export interface Feature {
  name: string;
  description: string;
  source: string;
  verified: boolean;
  mention_in_marketing: boolean;
}

export interface Privacy {
  has_account: boolean | null;
  collects_personal_data: boolean | null;
  stores_data_locally: boolean | null;
  uses_cloud_sync: boolean | null;
  uses_analytics: boolean | null;
  uses_crash_reporting: boolean | null;
  uses_ads: boolean | null;
  uses_tracking: boolean | null;
  uses_third_party_sdks: string[];
  privacy_policy_url: string;
  notes: string;
}

export interface PermissionEntry {
  permission: string;
  platform: string;
  detected_from: string;
  purpose: string;
  verified: boolean;
}

export interface Monetization {
  is_free: boolean | null;
  is_paid_upfront: boolean | null;
  has_iap: boolean | null;
  iap_type: string;
  subscriptions: boolean;
  product_ids: string[];
  pricing_notes: string;
}

export interface Review {
  requires_login: boolean | null;
  demo_account_available: boolean | null;
  demo_username: string;
  demo_password: string;
  review_notes: string;
  testing_steps: string[];
  special_instructions: string;
}

export interface AiUsage {
  uses_ai: boolean | null;
  ai_features: string[];
  user_generated_content: boolean | null;
  moderation_notes: string;
}

export interface SourceFact {
  id: string;
  fact: string;
  source_type: string;
  source_file: string;
  confidence: string;
  verified: boolean;
  status: string;
}

export interface GeneratedField {
  field: string;
  value: string;
  alternatives: string[];
  character_count: number;
  max_characters: number | null;
  confidence: string;
  source_facts_used: string[];
  warnings: string[];
  missing_info: string[];
}

export interface ProjectTruthFile {
  schema_version: string;
  project: ProjectMeta;
  app_identity: AppIdentity;
  summary: Summary;
  features: Feature[];
  privacy: Privacy;
  permissions: PermissionEntry[];
  monetization: Monetization;
  review: Review;
  ai_usage: AiUsage;
  claims_to_avoid: string[];
  source_facts: SourceFact[];
  generated_fields: GeneratedField[];
  question_answers: Record<string, string>;
}

export interface ProjectSummary {
  id: string;
  name: string;
  path: string;
  updated_at: string;
}

export interface ValidationWarning {
  severity: string;
  field: string;
  message: string;
}

export interface AppSettings {
  provider: string;
  api_key: string;
  model: string;
  base_url: string;
  temperature: number;
  max_tokens: number;
  tone: string;
  context_preview_acknowledged: boolean;
}

export interface ProjectScanResult {
  path: string;
  framework: string | null;
  platforms: string[];
  app_name: string | null;
  bundle_id: string | null;
  version: string | null;
  build_number: string | null;
  min_os_version: string | null;
  dependencies: DetectedDependency[];
  permissions: DetectedPermission[];
  files_scanned: string[];
  detected_features: DetectedFeature[];
  risk_flags: RiskFlag[];
  questions: ConfirmationQuestion[];
  document_summaries: DocumentSummary[];
  confidence: string;
}

export interface DetectedDependency {
  name: string;
  version: string | null;
  source_file: string;
  category: string | null;
  risk: string | null;
  requires_confirmation: boolean;
}

export interface DetectedPermission {
  permission: string;
  platform: string;
  detected_from: string;
  plist_key: string | null;
}

export interface DetectedFeature {
  name: string;
  source_file: string;
  confidence: string;
}

export interface RiskFlag {
  flag: string;
  reason: string;
  source: string;
}

export interface ConfirmationQuestion {
  id: string;
  question: string;
  reason: string;
  options: string[];
  related_fact_ids: string[];
  required: boolean;
}

export interface DocumentSummary {
  file_name: string;
  first_paragraph: string;
  line_count: number;
}

export const FIELD_LIMITS: Record<string, number> = {
  app_name: 30,
  subtitle: 30,
  promotional_text: 170,
  keywords: 100,
};

export const PRODUCT_PAGE_FIELDS = [
  "subtitle",
  "promotional_text",
  "description",
  "keywords",
  "support_url_guidance",
  "marketing_url_guidance",
];

export const REVIEW_FIELDS = ["review_notes", "whats_new"];

export const PRIVACY_FIELDS = ["privacy_summary", "privacy_policy_outline"];

export const IAP_FIELDS = ["iap_display_name", "iap_description", "iap_review_notes"];

export const NAV_ITEMS = [
  { href: "/", label: "Dashboard" },
  { href: "/scan", label: "Project Scan" },
  { href: "/facts", label: "Facts" },
  { href: "/product-page", label: "Product Page" },
  { href: "/review", label: "Review Info" },
  { href: "/privacy", label: "Privacy" },
  { href: "/iap", label: "IAP" },
  { href: "/export", label: "Export" },
  { href: "/settings", label: "Settings" },
];
