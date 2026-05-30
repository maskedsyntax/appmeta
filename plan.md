# AppMeta — Multi-Stack Scanner Plan

Roadmap to support all target mobile project types. See [spec.md](spec.md) for product details and [TODO.md](TODO.md) for the implementation checklist.

## Goal

Expand the local project scanner from Flutter-only to full support for:

| Project type | Detection signal | Target status |
|--------------|------------------|---------------|
| **Flutter** | `pubspec.yaml` | Fully supported (done) |
| **Swift (SPM)** | `Package.swift` | Full scan parity with Flutter |
| **iOS Native** | `*.xcodeproj` / `*.xcworkspace` | Full scan parity with Flutter |
| **React Native** | `package.json` + `react-native` dep | Full scan parity with Flutter |
| **Expo** | `app.json` / `app.config.*` + Expo deps | Full scan parity with Flutter |

Each stack should produce the same `ProjectScanResult` shape: app identity, dependencies, permissions, features, risk flags, and confirmation questions.

---

## Architecture

### Scanner router (`scanner/mod.rs`)

Replace the current Flutter-first branch with a detector chain:

```txt
scan_project(path)
  → detect_project_type(path)   // returns enum: Flutter | SwiftSpm | IosNative | ReactNative | Expo
  → dispatch to stack module
  → merge shared passes (documents, dependency risk, permissions)
  → ProjectScanResult
```

Add `scanner/detect.rs` with ordered heuristics:

1. `pubspec.yaml` → Flutter
2. `app.json` / `app.config.js` / `app.config.ts` with Expo deps → Expo
3. `package.json` with `react-native` → React Native
4. `Package.swift` (without dominant RN/Flutter signals) → Swift SPM
5. `*.xcodeproj` / `*.xcworkspace` → iOS Native
6. Otherwise → `InvalidProject` with a helpful message listing supported types

### New modules

```txt
scanner/
  detect.rs          # project type detection
  flutter.rs         # existing
  ios.rs             # extend: generic Info.plist discovery
  swift_spm.rs       # Package.swift + Package.resolved
  ios_native.rs      # pbxproj, entitlements, plist discovery
  react_native.rs    # package.json, metro config, RN ios/android paths
  expo.rs            # app.json / app.config, eas.json
  android.rs         # AndroidManifest.xml (shared by Flutter, RN, Expo)
  features.rs        # extend: lib/, src/, app/, screens/ conventions per stack
  dependencies.rs    # extend: npm + SPM + CocoaPods risk maps
```

### Shared extraction targets (all stacks)

Every stack scanner must populate:

- App name, bundle ID, version, build number
- Framework label and platforms (`iOS`, `Android`, `macOS` where applicable)
- Minimum OS version
- Dependencies (with stack-appropriate source)
- iOS permission usage keys from Info.plist
- Android dangerous permissions from manifest (where Android folder exists)
- Detected features / screens from source tree
- Document summaries (README, CHANGELOG, spec)
- Dependency risk flags + confirmation questions

---

## Per-Stack Implementation

### Flutter (maintain)

Already implemented. Minor follow-ups:

- [ ] Parse `android/app/src/main/AndroidManifest.xml` via shared `android.rs`
- [ ] Parse `ios/Podfile.lock` for native iOS dependency signals

### Swift (Package.swift)

**Detect:** root or nested `Package.swift`

**Scan:**

- Parse `Package.swift` for package name, platforms, dependencies
- Parse `Package.resolved` for pinned versions
- Discover `Info.plist` under `Sources/`, `.build/`, or linked Xcode target
- Parse `*.entitlements` for capabilities
- Feature detection from `Sources/**/*.swift` (View/ViewController naming)

**Files:**

```txt
Package.swift
Package.resolved
Info.plist
*.entitlements
Sources/
README.md
```

### iOS Native (.xcodeproj)

**Detect:** `*.xcodeproj` or `*.xcworkspace` at root or under `ios/`

**Scan:**

- Parse `project.pbxproj` for `PRODUCT_BUNDLE_IDENTIFIER`, `MARKETING_VERSION`, `CURRENT_PROJECT_VERSION`, `IPHONEOS_DEPLOYMENT_TARGET`, target names
- Discover Info.plist path from pbxproj (`INFOPLIST_FILE`)
- Parse entitlements referenced in build settings
- Parse `Podfile.lock` / `Package.resolved` if present
- Feature detection from `*.swift` / `*.m` / `*.storyboard` filenames

**Files:**

```txt
*.xcodeproj/project.pbxproj
*.xcworkspace
Info.plist
*.entitlements
Podfile.lock
Package.resolved
```

### React Native

**Detect:** `package.json` with `react-native` in dependencies (not Expo-managed unless Expo signals win)

**Scan:**

- Parse `package.json` for name, version, dependencies, scripts
- Resolve iOS plist: `ios/<AppName>/Info.plist` (discover from `.xcodeproj` or folder scan)
- Parse `android/app/src/main/AndroidManifest.xml`
- Parse `ios/Podfile.lock` for native modules
- Feature detection from `src/`, `app/`, `screens/`, `pages/` (`.tsx`, `.jsx`, `.ts`, `.js`)
- Map npm deps to risk flags (Firebase, Sentry, AdMob, etc.)

**Files:**

```txt
package.json
package-lock.json / yarn.lock / pnpm-lock.yaml
ios/
android/
app.json            # bare RN may have this too
metro.config.js
```

### Expo

**Detect:** `expo` in `package.json` deps **or** standalone `app.json` / `app.config.js` / `app.config.ts`

**Scan:**

- Parse `app.json` / evaluated static fields from config for `name`, `slug`, `version`, `ios.bundleIdentifier`, `android.package`
- Parse `package.json` dependencies (Expo SDK modules)
- Prebuild / ios paths same as React Native when `ios/` exists
- Parse `eas.json` for build profiles (optional metadata)
- Feature detection from `app/` directory (Expo Router) or `src/`

**Files:**

```txt
app.json
app.config.js / app.config.ts
package.json
eas.json
ios/                # after prebuild
android/
```

---

## Shared Infrastructure

### Info.plist discovery (`ios.rs`)

Generalize beyond Flutter's fixed paths:

```txt
ios/Runner/Info.plist          # Flutter
ios/<Target>/Info.plist        # RN / Expo / native
<AppName>/Info.plist           # SPM / native root layouts
```

Walk `ios/` and root for `Info.plist`; prefer plist linked from pbxproj when available.

### Android manifest (`android.rs`)

Shared parser for:

```txt
android/app/src/main/AndroidManifest.xml   # Flutter, RN, Expo
```

Extract: `package`, `versionName`, `versionCode`, `uses-permission`, `uses-feature`.

### Dependency risk mapping (`dependencies.rs`)

Extend maps for:

- **pub** — existing Flutter map
- **npm** — `@react-native-firebase/*`, `@sentry/react-native`, `react-native-purchases`, etc.
- **SPM / CocoaPods** — StoreKit, Firebase, analytics pods

Same output: `risk_flags` + confirmation `questions`.

### Feature detection (`features.rs`)

Stack-aware source roots:

| Stack | Roots | Patterns |
|-------|-------|----------|
| Flutter | `lib/` | `*Screen.dart`, `*Page.dart` |
| Swift / iOS Native | `Sources/`, project dirs | `*View.swift`, `*ViewController.swift` |
| React Native / Expo | `src/`, `app/`, `screens/` | `*Screen.tsx`, `*Page.tsx`, Expo Router `app/**/index.tsx` |

---

## UI & Truth File

- **Scan page:** show detected framework badge and stack-specific files scanned
- **Facts page:** no schema change — `framework` and `platforms` already exist on truth file
- **Validation:** stack-aware warnings (e.g. missing bundle ID for RN prebuild projects)
- **Dashboard copy:** "Connect a mobile project folder" instead of Flutter-only wording

---

## Testing Strategy

Fixtures under `src-tauri/tests/fixtures/`:

```txt
fixtures/
  flutter-minimal/
  swift-spm-minimal/
  ios-native-minimal/
  react-native-minimal/
  expo-minimal/
```

Each fixture gets a unit test asserting:

- Correct `framework` and `platforms`
- Bundle ID and version extracted
- At least one dependency and permission (where applicable)
- Feature detection finds expected screen names

Integration test: `scan_project_cmd` round-trip for each fixture.

Manual E2E (see TODO.md): one real project per stack through scan → facts → generate → export.

---

## Phasing

### Phase A — Detection & routing

- `detect.rs` + refactor `mod.rs` router
- Remove hard error for non-Flutter; route to stack handlers

### Phase B — iOS Native + Swift SPM

- `ios_native.rs`, `swift_spm.rs`
- Generic plist + pbxproj parsing
- Highest value for App Store–native developers

### Phase C — React Native + Expo

- `react_native.rs`, `expo.rs`
- npm dependency risk map
- RN/Expo plist path discovery

### Phase D — Android + polish

- Shared `android.rs`
- Extend `dependencies.rs` and `features.rs`
- UI copy updates, fixture tests, E2E per stack

---

## Out of Scope (this plan)

- Capacitor, Ionic, Unity, Kotlin Multiplatform
- Deep Gradle / Xcode build graph analysis
- Automatic `app.config.ts` evaluation (static parse only for MVP of Expo config)
- App Store Connect API submission

These can be added later using the same router pattern.
