mod flutter;
mod ios;
mod dependencies;
mod permissions;
mod documents;
mod features;

pub mod types;

use crate::error::{AppError, AppResult};
use std::path::Path;
pub use types::*;

pub fn scan_project(path: &str) -> AppResult<ProjectScanResult> {
    let root = Path::new(path);
    if !root.is_dir() {
        return Err(AppError::InvalidProject(format!(
            "Path is not a directory: {path}"
        )));
    }

    let mut files_scanned = Vec::new();
    let framework;
    let mut platforms = Vec::new();
    let mut app_name = None;
    let mut bundle_id = None;
    let mut version = None;
    let mut build_number = None;
    let mut min_os_version = None;
    let mut dependencies = Vec::new();
    let mut permissions = Vec::new();
    let mut detected_features = Vec::new();
    let mut risk_flags = Vec::new();
    let mut questions = Vec::new();
    let mut document_summaries = Vec::new();

    let pubspec_path = root.join("pubspec.yaml");
    if pubspec_path.exists() {
        framework = Some("Flutter".to_string());
        platforms.push("iOS".to_string());
        if root.join("android").exists() {
            platforms.push("Android".to_string());
        }
        if root.join("macos").exists() {
            platforms.push("macOS".to_string());
        }
        files_scanned.push("pubspec.yaml".to_string());

        let pubspec = flutter::parse_pubspec(&pubspec_path)?;
        app_name = pubspec.name.clone();
        version = pubspec.version.clone();
        if let Some(desc) = pubspec.description {
            document_summaries.push(DocumentSummary {
                file_name: "pubspec.yaml".to_string(),
                first_paragraph: desc,
                line_count: 1,
            });
        }
        dependencies.extend(pubspec.dependencies);
    } else if root.join("Package.swift").exists() {
        framework = Some("Swift".to_string());
        platforms.push("iOS".to_string());
        files_scanned.push("Package.swift".to_string());
    } else {
        let has_xcode = glob::glob(&format!("{}/**/*.xcodeproj", path))
            .ok()
            .and_then(|mut p| p.next())
            .transpose()
            .ok()
            .flatten()
            .is_some();
        if has_xcode {
            framework = Some("iOS Native".to_string());
            platforms.push("iOS".to_string());
        } else {
            return Err(AppError::InvalidProject(
                "Unsupported project type. MVP supports Flutter projects.".into(),
            ));
        }
    }

    let plist_paths = [
        root.join("ios/Runner/Info.plist"),
        root.join("macos/Runner/Info.plist"),
    ];
    for plist_path in &plist_paths {
        if plist_path.exists() {
            let rel = plist_path
                .strip_prefix(root)
                .unwrap_or(plist_path)
                .to_string_lossy()
                .to_string();
            files_scanned.push(rel.clone());
            let ios_info = ios::parse_info_plist(plist_path)?;
            if bundle_id.is_none() {
                bundle_id = ios_info.bundle_id;
            }
            if version.is_none() {
                version = ios_info.version;
            }
            if build_number.is_none() {
                build_number = ios_info.build_number;
            }
            if min_os_version.is_none() {
                min_os_version = ios_info.min_os_version;
            }
            permissions.extend(ios_info.permissions);
        }
    }

    let dep_analysis = dependencies::analyze_dependencies(&dependencies);
    risk_flags.extend(dep_analysis.risk_flags);
    questions.extend(dep_analysis.questions);

    if let Ok(docs) = documents::scan_documents(root) {
        for doc in docs {
            files_scanned.push(doc.file_name.clone());
            document_summaries.push(doc);
        }
    }

    if root.join("lib").exists() {
        detected_features = features::detect_features(root)?;
        for f in &detected_features {
            files_scanned.push(f.source_file.clone());
        }
    }

    let confidence = if app_name.is_some() && bundle_id.is_some() {
        "high"
    } else if app_name.is_some() || bundle_id.is_some() {
        "medium"
    } else {
        "low"
    };

    Ok(ProjectScanResult {
        path: path.to_string(),
        framework,
        platforms,
        app_name,
        bundle_id,
        version,
        build_number,
        min_os_version,
        dependencies,
        permissions,
        files_scanned,
        detected_features,
        risk_flags,
        questions,
        document_summaries,
        confidence: confidence.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn scan_flutter_fixture() {
        let dir = TempDir::new().unwrap();
        let root = dir.path();
        fs::write(
            root.join("pubspec.yaml"),
            r#"name: test_app
version: 1.0.0+1
description: A test app
dependencies:
  flutter:
    sdk: flutter
  in_app_purchase: ^3.0.0
  drift: ^2.0.0
"#,
        )
        .unwrap();
        fs::create_dir_all(root.join("ios/Runner")).unwrap();
        fs::write(
            root.join("ios/Runner/Info.plist"),
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>CFBundleIdentifier</key><string>com.example.test</string>
  <key>CFBundleShortVersionString</key><string>1.0.0</string>
  <key>CFBundleVersion</key><string>1</string>
  <key>NSCameraUsageDescription</key><string>Take photos</string>
</dict></plist>"#,
        )
        .unwrap();
        fs::create_dir_all(root.join("lib/screens")).unwrap();
        fs::write(root.join("lib/screens/HomeScreen.dart"), "").unwrap();

        let result = scan_project(root.to_str().unwrap()).unwrap();
        assert_eq!(result.framework.as_deref(), Some("Flutter"));
        assert_eq!(result.app_name.as_deref(), Some("test_app"));
        assert_eq!(result.bundle_id.as_deref(), Some("com.example.test"));
        assert!(!result.dependencies.is_empty());
        assert!(!result.questions.is_empty());
        assert!(!result.detected_features.is_empty());
    }
}
