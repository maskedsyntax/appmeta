# AppMeta TODO

Implementation checklist for the AppMeta MVP. See [spec.md](spec.md) for full product details.

## Phase 1 — Skeleton

- [x] Scaffold Tauri + SvelteKit app
- [x] Sidebar navigation (Dashboard, Scan, Facts, Product Page, Review, Privacy, IAP, Export, Settings)
- [x] JSON settings persistence (`settings.json`)
- [x] JSON project persistence (`projects/{id}.json`)
- [x] Tauri commands: `pick_project_folder`, `load/save_settings`, `load/save/list_projects`
- [x] App error types (`error.rs`)
- [x] Warnings panel in layout

## Phase 2 — Scanner

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

## MVP Complete When

- [ ] Manual E2E: scan real Flutter project → confirm facts → generate → export
- [x] `npm run tauri dev` launches without errors
- [x] `npm run tauri build` produces macOS app

## Post-MVP

- [ ] Anthropic provider
- [ ] Gemini provider
- [ ] OpenRouter provider (partial — settings only)
- [ ] macOS Keychain for API keys
- [ ] Swift native project scanner
- [ ] Android manifest scanning
- [ ] Localization generator
- [ ] Screenshot caption generator
- [ ] Saved templates
- [ ] App Store Connect API integration
- [ ] Windows/Linux builds
