# AppMeta — App Store Connect Submission Assistant

## 1. Overview

AppMeta is a privacy-first desktop app that helps indie developers generate accurate App Store Connect submission content using their own AI API key.

The app connects to a local project folder, scans relevant files, extracts project facts, asks the developer to confirm uncertain details, and then generates App Store Connect-ready metadata such as app description, subtitle, keywords, promotional text, review notes, privacy explanations, IAP descriptions, and release notes.

The main goal is not to let AI randomly guess metadata. The main goal is to build a verified project profile first, then use AI only to transform that verified context into polished App Store Connect fields.

---

## 2. Core Product Principle

AI is not the source of truth.

The project profile is the source of truth.

The app should follow this flow:

```txt
Connect Project Folder
→ Scan project locally
→ Extract possible facts
→ Ask confirmation questions
→ Build Project Truth File
→ Generate App Store Connect fields
→ Validate character limits and missing info
→ Export/copy submission pack
```

The AI must only use verified or explicitly confirmed project facts.

If information is missing, the AI should ask for it instead of inventing an answer.

---

## 3. Target Users

### Primary Users

- Indie iOS/macOS developers
- Flutter developers shipping to App Store
- Swift/SwiftUI developers
- Small app studios
- Developers submitting their first app
- Developers who repeatedly struggle with App Store Connect metadata

### Secondary Users

- Freelancers preparing submission assets for clients
- Agencies managing multiple app submissions
- Developers localizing their App Store listing

---

## 4. Platforms

### MVP Platform

- macOS desktop app

### Later Platforms

- Windows desktop
- Linux desktop

Tauri makes cross-platform desktop possible, but the MVP should focus on macOS because the product is App Store submission-focused and many iOS/macOS developers are on Mac.

---

## 5. Tech Stack

### Desktop Framework

- Tauri

### Backend/Core

- Rust

### Frontend

- SvelteKit or Svelte
- TypeScript

### Local Storage

Use one of:

- SQLite via Rust
- Tauri store plugin
- JSON files in app data directory for MVP

Recommended MVP approach:

```txt
Local app config: JSON
Project profiles: JSON
Saved submission packs: JSON
```

Move to SQLite only if project management becomes more complex.

### AI Providers

BYOK-first.

Initial providers:

- OpenAI
- Anthropic
- Google Gemini
- OpenRouter

Later providers:

- Groq
- Mistral
- Ollama/local models

### Secret Storage

Use OS-native secure storage where possible.

Suggested:

- macOS Keychain
- Windows Credential Manager
- Linux Secret Service

For MVP, if secure storage is too much, store API keys locally with strong warning and add secure storage before public release.

---

## 6. High-Level App Sections

### 6.1 Welcome / Dashboard

Purpose:

- Create new submission project
- Open existing submission project
- Configure AI provider
- View recent projects

Actions:

- Connect Project Folder
- Paste App Details
- Import README/spec.md
- Open Saved Project

---

### 6.2 Project Scanner

Purpose:

Scan a local project folder and detect useful App Store submission context.

The scanner should run locally. Source files should not be uploaded to any server by default.

Scanner output should be structured, not prose.

Example:

```json
{
  "framework": "Flutter",
  "platforms": ["iOS", "Android", "macOS"],
  "app_name": "Patterns",
  "bundle_id": "dev.aftaab.patterns",
  "version": "1.1.4",
  "dependencies": ["drift", "sqlite3_flutter_libs", "in_app_purchase"],
  "permissions": [],
  "detected_features": ["journal", "analytics", "settings", "paywall"],
  "risk_flags": ["iap_detected"],
  "confidence": "medium"
}
```

---

### 6.3 Detected Facts Review

Purpose:

Show extracted facts and let the developer confirm/edit them.

Facts should have statuses:

- Verified
- Needs Confirmation
- Rejected
- Unknown

Example:

```txt
Detected: App uses in-app purchases
Source: pubspec.yaml dependency "in_app_purchase"
Status: Needs Confirmation

Question:
What type of in-app purchase does the app use?
- Tip jar
- One-time unlock
- Subscription
- Consumable credits
- Other
```

---

### 6.4 Project Truth File

Purpose:

Store confirmed facts about the app.

This file is the main context used for AI generation.

Example file name:

```txt
appmeta.project.json
```

It can be stored inside the app data directory by default. Later, optionally allow saving it inside the project repo.

---

### 6.5 Field Generator

Purpose:

Generate App Store Connect fields from verified project facts.

Sections:

- App Information
- Product Page
- Version Information
- App Review Information
- Privacy
- In-App Purchases
- Release Notes
- Localization
- Export

Each generated field should include:

- Generated value
- Character count
- Copy button
- Regenerate button
- Warnings
- Source facts used
- Confidence level

---

### 6.6 Validation

Purpose:

Warn the user about missing, vague, risky, or invalid fields.

Validation examples:

- Subtitle too long
- Keywords exceed 100 characters
- Description is too vague
- Review notes missing for login-based app
- IAP detected but review instructions missing
- Privacy policy URL missing
- Local-first claim not confirmed
- Medical/health claim may need safer wording
- App uses analytics dependency but privacy answer says no data collected

---

### 6.7 Export

Export formats:

- Markdown
- JSON
- Plain text
- App Store Connect checklist

MVP export:

```txt
submission-pack.md
submission-pack.json
```

Later:

- PDF
- CSV
- Direct App Store Connect API integration

---

## 7. MVP Scope

The MVP should focus on one strong use case:

> Connect a Flutter/iOS project folder and generate App Store Connect-ready metadata from confirmed facts.

### MVP Must-Have Features

1. Local project folder picker
2. Flutter project detection
3. iOS `Info.plist` scanner
4. `pubspec.yaml` dependency scanner
5. README/spec/changelog importer
6. Detected facts review screen
7. Manual project questionnaire
8. Project Truth File
9. BYOK AI settings
10. Product page metadata generator
11. App Review notes generator
12. Privacy questionnaire/helper
13. IAP metadata helper
14. Character count validation
15. Markdown/JSON export

### MVP Nice-to-Have Features

1. Swift native project detection
2. Android manifest scanning
3. Multiple AI providers
4. Localization generator
5. Screenshot caption generator
6. Local secure key storage
7. Saved templates

### Not in MVP

- Direct App Store Connect API submission
- Full privacy label automation
- Competitor keyword scraping
- App Store ranking database
- Screenshot image generation
- Team collaboration
- Cloud sync
- VS Code extension
- CLI companion

---

## 8. Project Scanner Design

### 8.1 Supported Project Types in MVP

#### Flutter

Detect files:

```txt
pubspec.yaml
lib/
ios/Runner/Info.plist
android/app/src/main/AndroidManifest.xml
macos/Runner/Info.plist
README.md
CHANGELOG.md
```

#### Basic iOS/macOS Native Detection

Detect files:

```txt
*.xcodeproj
*.xcworkspace
Info.plist
Package.swift
Package.resolved
```

Native Swift support can be partial in MVP.

---

### 8.2 Files to Scan

#### General

```txt
README.md
CHANGELOG.md
spec.md
privacy.md
terms.md
package.json
.env.example
```

#### Flutter

```txt
pubspec.yaml
lib/
ios/Runner/Info.plist
ios/Podfile.lock
android/app/src/main/AndroidManifest.xml
android/app/build.gradle
macos/Runner/Info.plist
```

#### Swift/iOS

```txt
Info.plist
Package.swift
Package.resolved
*.entitlements
project.pbxproj
```

---

### 8.3 What to Extract

#### App Identity

- App name
- Bundle ID
- Version
- Build number
- Supported platforms
- Framework
- Minimum OS version

#### Features

Extract from:

- README
- spec files
- route names
- screen/page/view file names
- feature folders
- changelog

Examples:

```txt
JournalScreen
AnalyticsView
PaywallPage
SettingsScreen
EntryEditor
OnboardingFlow
```

#### Privacy Signals

Detect:

- Account/auth dependencies
- Analytics SDKs
- Crash reporting SDKs
- Ads SDKs
- Tracking SDKs
- Cloud/backend SDKs
- Local database usage
- Permissions

#### Monetization Signals

Detect:

- StoreKit
- RevenueCat
- in_app_purchase
- paywall screens
- product IDs
- subscription references
- tip jar references

#### Permissions

Detect from iOS/macOS `Info.plist`:

- Camera
- Microphone
- Location
- Photo Library
- Contacts
- Calendar
- Bluetooth
- HealthKit
- User Tracking

Detect from Android manifest:

- Internet
- Camera
- Microphone
- Location
- Contacts
- Storage
- Notifications

---

## 9. Dependency Risk Mapping

The scanner should map dependencies to possible App Store/privacy implications.

Example mapping:

```json
{
  "firebase_analytics": {
    "category": "analytics",
    "risk": "data_collection_possible",
    "question": "Do you send analytics events from the app?"
  },
  "firebase_crashlytics": {
    "category": "crash_reporting",
    "risk": "diagnostics_data_possible",
    "question": "Do you collect crash reports or diagnostics?"
  },
  "sentry_flutter": {
    "category": "crash_reporting",
    "risk": "diagnostics_data_possible",
    "question": "Do you use Sentry in production?"
  },
  "google_mobile_ads": {
    "category": "ads",
    "risk": "tracking_possible",
    "question": "Does the app show ads or use ad tracking?"
  },
  "in_app_purchase": {
    "category": "iap",
    "risk": "iap_detected",
    "question": "What type of in-app purchases does the app offer?"
  },
  "revenuecat": {
    "category": "iap",
    "risk": "subscription_or_iap_detected",
    "question": "Are you using RevenueCat for subscriptions, one-time unlocks, or tips?"
  },
  "drift": {
    "category": "local_database",
    "risk": "local_storage_detected",
    "question": "Is user data stored only on-device?"
  },
  "sqflite": {
    "category": "local_database",
    "risk": "local_storage_detected",
    "question": "Is user data stored only on-device?"
  },
  "supabase": {
    "category": "backend",
    "risk": "server_data_possible",
    "question": "What user data is sent to Supabase?"
  }
}
```

Dependencies should not directly become final facts. They should become detected signals that the user confirms.

---

## 10. Project Truth File Schema

Suggested MVP schema:

```json
{
  "schema_version": "1.0",
  "project": {
    "id": "uuid",
    "name": "",
    "path": "",
    "created_at": "",
    "updated_at": ""
  },
  "app_identity": {
    "app_name": "",
    "bundle_id": "",
    "sku": "",
    "platforms": [],
    "primary_category": "",
    "secondary_category": "",
    "version": "",
    "copyright": ""
  },
  "summary": {
    "short_summary": "",
    "long_summary": "",
    "target_audience": [],
    "primary_use_cases": [],
    "not_for": []
  },
  "features": [
    {
      "name": "",
      "description": "",
      "source": "",
      "verified": false,
      "mention_in_marketing": true
    }
  ],
  "privacy": {
    "has_account": null,
    "collects_personal_data": null,
    "stores_data_locally": null,
    "uses_cloud_sync": null,
    "uses_analytics": null,
    "uses_crash_reporting": null,
    "uses_ads": null,
    "uses_tracking": null,
    "uses_third_party_sdks": [],
    "privacy_policy_url": "",
    "notes": ""
  },
  "permissions": [
    {
      "permission": "",
      "platform": "",
      "detected_from": "",
      "purpose": "",
      "verified": false
    }
  ],
  "monetization": {
    "is_free": null,
    "is_paid_upfront": null,
    "has_iap": null,
    "iap_type": "",
    "subscriptions": false,
    "product_ids": [],
    "pricing_notes": ""
  },
  "review": {
    "requires_login": null,
    "demo_account_available": null,
    "demo_username": "",
    "demo_password": "",
    "review_notes": "",
    "testing_steps": [],
    "special_instructions": ""
  },
  "ai_usage": {
    "uses_ai": null,
    "ai_features": [],
    "user_generated_content": null,
    "moderation_notes": ""
  },
  "claims_to_avoid": [],
  "source_facts": [
    {
      "id": "",
      "fact": "",
      "source_type": "manual|scanner|imported_doc|ai_extracted",
      "source_file": "",
      "confidence": "low|medium|high",
      "verified": false
    }
  ]
}
```

---

## 11. AI Generation Rules

Every AI generation request must include:

1. Project Truth File
2. Target field name
3. Field constraints
4. Tone preference
5. Things not to claim
6. Missing information behavior

### System Prompt Template

```txt
You are generating App Store Connect submission content.

Use ONLY the provided Project Truth File.
Do not invent features, integrations, privacy practices, medical claims, pricing, awards, platform support, or App Store policies.

If required information is missing, return:
MISSING_INFO: <question to ask the user>

If a claim is uncertain, return:
NEEDS_CONFIRMATION: <claim and why>

Write clearly and professionally.
Avoid exaggerated claims.
Avoid guaranteed outcomes.
Avoid claiming medical, therapeutic, financial, or legal benefits unless explicitly confirmed and safe.

Project Truth File:
{{PROJECT_TRUTH_FILE}}

Field:
{{FIELD_NAME}}

Field Constraints:
{{FIELD_CONSTRAINTS}}

Tone:
{{TONE}}
```

### Output Format

AI should return structured JSON:

```json
{
  "field": "subtitle",
  "value": "",
  "alternatives": [],
  "confidence": "high",
  "source_facts_used": [],
  "warnings": [],
  "missing_info": []
}
```

---

## 12. App Store Field Generators

### 12.1 App Information

Generate or assist with:

- App name
- Subtitle
- Category recommendation
- Content rights explanation
- Age rating notes
- Copyright

### 12.2 Product Page

Generate:

- Promotional text
- Description
- Keywords
- Screenshot caption ideas
- App preview script ideas
- Marketing URL guidance
- Support URL guidance

### 12.3 Version Information

Generate:

- What’s New
- Update description
- Release notes

### 12.4 App Review Information

Generate:

- Review notes
- Demo account instructions
- Testing flow
- IAP testing instructions
- Login explanation
- Permission explanation
- Sensitive feature explanation

### 12.5 Privacy

Generate:

- Privacy summary
- Privacy policy draft outline
- Data collection questionnaire
- Third-party SDK warnings
- Local-first explanation
- Tracking warning checklist

Important:

The app should not directly answer Apple privacy labels without user confirmation.

### 12.6 In-App Purchases

Generate:

- IAP display name
- IAP description
- Product ID suggestions
- Subscription group names
- Reviewer instructions
- Paywall copy

### 12.7 Localization

Later feature.

Generate localized versions of:

- App name
- Subtitle
- Promotional text
- Description
- Keywords
- What’s New

---

## 13. Validation Rules

### 13.1 Character Limits

Track character limits for fields where applicable.

Examples:

- Keywords: 100 characters
- Subtitle: 30 characters
- Promotional text: 170 characters
- App name: 30 characters

Keep the limits in a configurable constants file so they can be updated.

### 13.2 Content Validation

Warn if:

- Description is too vague
- Description claims unsupported features
- Keywords contain spaces after commas unnecessarily
- Subtitle repeats app name
- Health/medical claims are too strong
- AI claims are vague
- Privacy says no data collected while analytics SDK detected
- IAP detected but IAP metadata is missing
- Login detected but demo account is missing
- Camera/location/microphone permission detected but purpose is missing
- App claims local-first but backend SDK is detected
- App claims open-source but repository URL is missing

### 13.3 Completeness Validation

Required/recommended fields:

- App name
- Subtitle
- Description
- Keywords
- Category
- Support URL
- Privacy Policy URL
- Review contact info
- Review notes when needed
- Demo account when login is required
- IAP notes when IAP exists

---

## 14. UI Design

### 14.1 Layout

Use a clean developer-tool layout.

```txt
Sidebar
  Dashboard
  Project Scan
  Facts
  Product Page
  Review Info
  Privacy
  IAP
  Export
  Settings

Main Panel
  Current section content

Right Panel
  Warnings
  Source facts
  Character counts
  Missing info
```

### 14.2 Visual Style

- Minimal
- Developer-friendly
- Calm colors
- Good spacing
- Copy buttons everywhere
- Clear warning states
- No overwhelming dashboards

### 14.3 Important UI Components

- Folder picker
- Scan result cards
- Fact confirmation cards
- Editable JSON/project profile viewer
- Generated field cards
- Character counters
- Warning badges
- Copy buttons
- Export buttons
- Provider settings form

---

## 15. BYOK Settings

### 15.1 Provider Config

Fields:

- Provider
- API key
- Model
- Base URL for OpenRouter/custom providers
- Temperature
- Max tokens

### 15.2 Provider Interface

Create a common Rust trait/interface:

```rust
trait AiProvider {
    async fn generate(&self, request: AiRequest) -> Result<AiResponse, AiError>;
}
```

Provider implementations:

- OpenAIProvider
- AnthropicProvider
- GeminiProvider
- OpenRouterProvider

### 15.3 API Key Safety

Rules:

- Never log API keys
- Never include API keys in error reports
- Mask API keys in UI
- Store securely when possible
- Allow deleting keys
- Show what content will be sent before generation

---

## 16. Rust Module Structure

Suggested Tauri backend structure:

```txt
src-tauri/src/
  main.rs
  commands/
    mod.rs
    scan.rs
    project.rs
    ai.rs
    export.rs
    settings.rs
  scanner/
    mod.rs
    flutter.rs
    ios.rs
    android.rs
    dependencies.rs
    permissions.rs
    documents.rs
  ai/
    mod.rs
    providers/
      openai.rs
      anthropic.rs
      gemini.rs
      openrouter.rs
    prompts.rs
    types.rs
  project/
    mod.rs
    truth_file.rs
    validation.rs
    facts.rs
  storage/
    mod.rs
    config.rs
    projects.rs
    secrets.rs
  export/
    mod.rs
    markdown.rs
    json.rs
  error.rs
```

---

## 17. Frontend Structure

Suggested Svelte structure:

```txt
src/
  lib/
    api/
      tauri.ts
      ai.ts
      project.ts
    components/
      layout/
      scanner/
      facts/
      fields/
      settings/
      export/
    stores/
      projectStore.ts
      settingsStore.ts
      generationStore.ts
    types/
      project.ts
      scan.ts
      ai.ts
      fields.ts
  routes/
    +layout.svelte
    +page.svelte
    project/
    scan/
    facts/
    product-page/
    review-info/
    privacy/
    iap/
    export/
    settings/
```

---

## 18. Tauri Commands

Suggested commands:

```rust
#[tauri::command]
async fn pick_project_folder() -> Result<String, AppError>;

#[tauri::command]
async fn scan_project(path: String) -> Result<ProjectScanResult, AppError>;

#[tauri::command]
async fn create_project_from_scan(scan: ProjectScanResult) -> Result<ProjectTruthFile, AppError>;

#[tauri::command]
async fn save_project(project: ProjectTruthFile) -> Result<(), AppError>;

#[tauri::command]
async fn load_project(project_id: String) -> Result<ProjectTruthFile, AppError>;

#[tauri::command]
async fn generate_field(project: ProjectTruthFile, field: String) -> Result<GeneratedField, AppError>;

#[tauri::command]
async fn validate_project(project: ProjectTruthFile) -> Result<Vec<ValidationWarning>, AppError>;

#[tauri::command]
async fn export_markdown(project: ProjectTruthFile, fields: Vec<GeneratedField>) -> Result<String, AppError>;

#[tauri::command]
async fn save_settings(settings: AppSettings) -> Result<(), AppError>;

#[tauri::command]
async fn load_settings() -> Result<AppSettings, AppError>;
```

---

## 19. Data Types

### ProjectScanResult

```rust
struct ProjectScanResult {
    path: String,
    framework: Option<String>,
    platforms: Vec<String>,
    app_name: Option<String>,
    bundle_id: Option<String>,
    version: Option<String>,
    dependencies: Vec<DetectedDependency>,
    permissions: Vec<DetectedPermission>,
    files_scanned: Vec<String>,
    detected_features: Vec<DetectedFeature>,
    risk_flags: Vec<RiskFlag>,
    questions: Vec<ConfirmationQuestion>,
}
```

### DetectedDependency

```rust
struct DetectedDependency {
    name: String,
    version: Option<String>,
    source_file: String,
    category: Option<String>,
    risk: Option<String>,
    requires_confirmation: bool,
}
```

### ConfirmationQuestion

```rust
struct ConfirmationQuestion {
    id: String,
    question: String,
    reason: String,
    options: Vec<String>,
    related_fact_ids: Vec<String>,
    required: bool,
}
```

### GeneratedField

```rust
struct GeneratedField {
    field: String,
    value: String,
    alternatives: Vec<String>,
    character_count: usize,
    max_characters: Option<usize>,
    confidence: String,
    source_facts_used: Vec<String>,
    warnings: Vec<String>,
    missing_info: Vec<String>,
}
```

---

## 20. Export Format

### Markdown Export Example

```md
# App Store Submission Pack

## App Information

### App Name
Patterns - OCD Journaling

### Subtitle
Private OCD & ERP Tracker

## Product Page

### Promotional Text
Track OCD patterns, distress, and ERP progress privately on your device.

### Description
...

### Keywords
ocd,erp,journal,tracker,anxiety,therapy

## App Review Notes

...

## Privacy Summary

...

## In-App Purchases

...
```

### JSON Export Example

```json
{
  "app_information": {
    "name": "",
    "subtitle": ""
  },
  "product_page": {
    "promotional_text": "",
    "description": "",
    "keywords": ""
  },
  "review": {
    "notes": ""
  },
  "privacy": {
    "summary": ""
  }
}
```

---

## 21. Error Handling

The app should handle:

- Invalid project folder
- Permission denied while reading files
- Unsupported project type
- Missing AI API key
- AI provider request failure
- Invalid AI response JSON
- File parse failure
- Export write failure
- Network unavailable
- User cancels folder picker

Error messages should be human-readable.

Example:

```txt
Could not read ios/Runner/Info.plist. You can continue, but permission detection may be incomplete.
```

---

## 22. Security & Privacy

### Core Rules

- Do not upload source code by default.
- Scan project files locally.
- Send only confirmed structured facts to AI.
- Let the user preview AI context before sending.
- Never log API keys.
- Never log full project source code.
- Allow deleting all saved project data.
- Clearly show which provider receives the request.

### AI Context Preview

Before generating, show:

```txt
The following context will be sent to OpenAI:

- App name
- App summary
- Confirmed features
- Privacy answers
- Monetization info
- Review instructions

Source code will not be sent.
```

---

## 23. Monetization

### Free

- One saved project
- Manual app intake
- Basic product page generation
- Basic review notes
- Markdown export
- BYOK only

### Pro

- Unlimited projects
- Project folder scanner
- Privacy helper
- IAP helper
- Localization
- JSON export
- Advanced validation
- Saved templates
- Multiple AI providers

### Studio

- Client/project organization
- Team export format
- Multiple app profiles
- Custom templates
- Future App Store Connect API support

Recommended early pricing:

- Free
- Pro: $9 one-time or $12/year
- Studio: $29 one-time or $29/year

Since BYOK reduces your AI cost, one-time pricing can work well for indie developers.

---

## 24. Product Name

App name:

```txt
AppMeta
```

Other names considered during planning:

- StoreReady
- SubmitMate
- MetaPilot
- ShipKit
- ReviewReady
- StorePilot
- LaunchFields

---

## 25. Development Milestones

### Milestone 1 — App Skeleton

- Create Tauri + Svelte project
- Add sidebar layout
- Add settings page
- Add local project data model
- Add export placeholder

### Milestone 2 — Project Scanner MVP

- Folder picker
- Detect Flutter project
- Parse `pubspec.yaml`
- Parse iOS `Info.plist`
- Detect dependencies
- Detect basic screens/features from `lib/`
- Show scan result

### Milestone 3 — Fact Confirmation

- Create detected facts UI
- Add confirmation questions
- Allow editing facts
- Save Project Truth File

### Milestone 4 — BYOK AI

- Add OpenAI provider first
- Add provider settings
- Add prompt builder
- Generate one field: subtitle
- Validate structured JSON response

### Milestone 5 — Metadata Generator

- Generate subtitle
- Generate promotional text
- Generate description
- Generate keywords
- Add character counts
- Add copy buttons

### Milestone 6 — Review + Privacy + IAP

- Generate review notes
- Add privacy questionnaire
- Add IAP helper
- Add validation warnings

### Milestone 7 — Export

- Markdown export
- JSON export
- Save generated pack
- Copy all fields

### Milestone 8 — Polish

- Error states
- Empty states
- Better onboarding
- Secure API key storage
- App icon
- Landing page
- Build/release pipeline

---

## 26. First Build Checklist

Start with this order:

1. Create Tauri + Svelte app
2. Add sidebar navigation
3. Implement folder picker
4. Implement `scan_project`
5. Parse `pubspec.yaml`
6. Parse `ios/Runner/Info.plist`
7. Detect dependency categories
8. Show scan result in UI
9. Convert scan result to editable Project Truth File
10. Add OpenAI BYOK settings
11. Generate subtitle from confirmed facts
12. Add product page generation
13. Add markdown export

---

## 27. Important Product Warnings

The app should never say:

- Guaranteed App Store approval
- Guaranteed ASO ranking
- Guaranteed downloads
- Automatically correct privacy labels
- Medical/legal/financial compliance guaranteed

Use safer wording:

```txt
Helps prepare App Store Connect metadata based on your confirmed project details.
```

```txt
Highlights possible missing or risky information before submission.
```

```txt
Generates draft privacy and review text for developer review.
```

---

## 28. Long-Term Ideas

- CLI companion
- VS Code extension
- GitHub repo import
- App Store Connect API integration
- Screenshot caption generator
- App localization packs
- Metadata version history
- Client handoff exports
- App update changelog generator
- ASO keyword research integrations
- Play Store support
- Privacy policy hosted pages
- Submission readiness score

---

## 29. Final MVP Definition

The MVP is complete when a user can:

1. Open the desktop app
2. Connect a Flutter project folder
3. See detected app facts
4. Confirm/edit the facts
5. Enter their own AI API key
6. Generate App Store product page fields
7. Generate review notes
8. Generate privacy/IAP helper text
9. See character count warnings
10. Export a submission pack as Markdown/JSON

That is the first shippable version.
