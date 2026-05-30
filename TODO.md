# AppMeta TODO

Implementation checklist for the AppMeta MVP. See [spec.md](spec.md) for full product details and [plan.md](plan.md) for the multi-stack scanner roadmap.

## Phase 1 — Skeleton

- [x] Scaffold Tauri + SvelteKit app
- [x] Sidebar navigation (Dashboard, Scan, Facts, Product Page, Review, Privacy, IAP, Export, Settings)
- [x] JSON settings persistence (`settings.json`)
- [x] JSON project persistence (`projects/{id}.json`)
- [x] Tauri commands: `pick_project_folder`, `load/save_settings`, `load/save/list_projects`
- [x] App error types (`error.rs`)
- [x] Warnings panel in layout

## Phase 2 — Scanner (Flutter)

- [x] `scanner/flutter.rs` — parse `pubspec.yaml`
- [x] `scanner/ios.rs` — parse `Info.plist`
- [x] `scanner/dependencies.rs` — risk mapping + confirmation questions
- [x] `scanner/permissions.rs` — iOS usage key mapping
- [x] `scanner/documents.rs` — README/CHANGELOG/spec import
- [x] `scanner/features.rs` — detect screens from `lib/`
- [x] `scan_project_cmd` Tauri command
- [x] Scan UI page with results table + raw JSON
- [x] Rust unit tests for scanner fixtures

## Phase 3 — Truth File

- [x] `project/truth_file.rs` — schema v1.0
- [x] `project/facts.rs` — `create_project_from_scan`
- [x] Facts review UI with status toggles
- [x] Confirmation question UI
- [x] Manual questionnaire (summary, privacy, review)
- [x] Save/load project truth file
- [x] JSON debug view on Facts page

## Phase 4 — BYOK AI

- [x] `ai/providers/openai.rs` — OpenAI chat completions
- [x] `ai/prompts.rs` — system prompt + truth file injection
- [x] `generate_field_cmd` with structured JSON parsing
- [x] Handle `MISSING_INFO` / invalid JSON responses
- [x] Settings page (provider, API key, model, temperature)
- [x] Test connection button
- [x] AI context preview modal before first generation

## Phase 5 — Product Page Generator

- [x] Generate: subtitle, promotional_text, description, keywords
- [x] Character limits in `project/constants.rs`
- [x] FieldCard component with char count, copy, regenerate
- [x] Store generated fields in project truth file

## Phase 6 — Review, Privacy, IAP + Validation

- [x] Review notes + what's new generators
- [x] Privacy questionnaire UI
- [x] Privacy summary + policy outline generators
- [x] IAP helper fields + manual IAP form
- [x] `validate_project_cmd` — completeness, consistency, content heuristics
- [x] Live warnings panel

## Phase 7 — Export

- [x] `export/markdown.rs` — submission-pack.md format
- [x] `export/json.rs` — structured JSON export
- [x] Export page with preview, copy all, save to folder
- [x] `save_submission_pack_cmd`

## Phase 8 — Hardening

- [x] Error banner in layout
- [x] Empty states on all pages
- [x] README with build/run instructions
- [x] CI: cargo test, clippy, npm check
- [x] Updated `.gitignore`

## Phase 9 — Multi-Stack Scanner

See [plan.md](plan.md) for architecture and file-level detail.

### 9A — Detection & routing

- [ ] `scanner/detect.rs` — project type enum + ordered heuristics
- [ ] Refactor `scanner/mod.rs` — dispatch to stack modules instead of Flutter-only gate
- [ ] Helpful `InvalidProject` error listing supported types
- [ ] Scan UI: framework badge + stack-specific files scanned

### 9B — Swift (Package.swift)

- [ ] `scanner/swift_spm.rs` — parse `Package.swift` (name, platforms, dependencies)
- [ ] Parse `Package.resolved` for pinned versions
- [ ] Discover Info.plist + `*.entitlements` for SPM layouts
- [ ] Feature detection from `Sources/**/*.swift`
- [ ] SPM dependency risk mapping in `dependencies.rs`
- [ ] Fixture: `tests/fixtures/swift-spm-minimal/` + unit test

### 9C — iOS Native (.xcodeproj)

- [ ] `scanner/ios_native.rs` — parse `project.pbxproj` (bundle ID, version, build, min OS, targets)
- [ ] Resolve Info.plist path from pbxproj `INFOPLIST_FILE`
- [ ] Parse `*.entitlements` and optional `Podfile.lock`
- [ ] Feature detection from Swift/ObjC source filenames
- [ ] Fixture: `tests/fixtures/ios-native-minimal/` + unit test

### 9D — React Native

- [ ] `scanner/react_native.rs` — parse `package.json` (name, version, dependencies)
- [ ] Discover iOS plist under `ios/<Target>/Info.plist`
- [ ] npm dependency risk mapping in `dependencies.rs` (Firebase, Sentry, ads, IAP, etc.)
- [ ] Feature detection from `src/`, `screens/`, `pages/` (`.tsx`, `.jsx`)
- [ ] Fixture: `tests/fixtures/react-native-minimal/` + unit test

### 9E — Expo

- [ ] `scanner/expo.rs` — parse `app.json` / static `app.config.*` fields
- [ ] Parse Expo SDK deps from `package.json`
- [ ] Reuse RN ios/android paths when prebuild output exists
- [ ] Optional: parse `eas.json` build profile metadata
- [ ] Feature detection from `app/` (Expo Router) and `src/`
- [ ] Fixture: `tests/fixtures/expo-minimal/` + unit test

### 9F — Shared Android & iOS polish

- [ ] `scanner/android.rs` — shared `AndroidManifest.xml` parser (permissions, package, version)
- [ ] Extend `scanner/ios.rs` — generic Info.plist discovery (not just `ios/Runner/`)
- [ ] Extend `scanner/features.rs` — stack-aware source roots
- [ ] Flutter: wire Android manifest into scan result
- [ ] Stack-aware validation warnings in `validate_project_cmd`

### 9G — Docs & E2E

- [ ] Update README workflow copy (mobile project, not Flutter-only)
- [ ] Manual E2E: Flutter project → confirm facts → generate → export
- [ ] Manual E2E: Swift SPM project
- [ ] Manual E2E: iOS Native (.xcodeproj) project
- [ ] Manual E2E: React Native project
- [ ] Manual E2E: Expo project

- [x] Persist scan confirmation questions on project truth file
- [x] Delete project from dashboard
- [x] Generate all fields (product page, review, privacy, IAP)
- [x] Rescan current project without losing edits
- [x] Reset AI context preview in settings

- [ ] Manual E2E per supported stack (Flutter, Swift SPM, iOS Native, React Native, Expo)
- [x] `npm run tauri dev` launches without errors
- [x] `npm run tauri build` produces macOS app

## Post-MVP

- [ ] Anthropic provider
- [ ] Gemini provider
- [ ] OpenRouter provider (partial — settings only)
- [ ] macOS Keychain for API keys
- [ ] Capacitor / Ionic project scanner
- [ ] Localization generator
- [ ] Screenshot caption generator
- [ ] Saved templates
- [ ] App Store Connect API integration
- [ ] Windows/Linux builds
