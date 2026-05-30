use crate::scanner::DetectedFeature;
use crate::error::AppResult;
use std::path::Path;
use walkdir::WalkDir;

const MAX_FILES: usize = 500;
const SKIP_DIRS: &[&str] = &[".dart_tool", "build", ".git", "node_modules", ".pub-cache"];

pub fn detect_features(root: &Path) -> AppResult<Vec<DetectedFeature>> {
    let lib_dir = root.join("lib");
    if !lib_dir.is_dir() {
        return Ok(Vec::new());
    }

    let suffixes = ["Screen", "Page", "View", "Widget"];
    let mut features = Vec::new();
    let mut count = 0;

    for entry in WalkDir::new(&lib_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        if count >= MAX_FILES {
            break;
        }
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("dart") {
            continue;
        }
        if SKIP_DIRS.iter().any(|d| path.to_string_lossy().contains(d)) {
            continue;
        }
        count += 1;

        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        for suffix in suffixes {
            let stem_lower = file_stem.to_lowercase();
            let suffix_lower = suffix.to_lowercase();
            if stem_lower.ends_with(&suffix_lower) {
                let name = file_stem
                    .strip_suffix(suffix)
                    .or_else(|| {
                        file_stem
                            .char_indices()
                            .nth(file_stem.len().saturating_sub(suffix.len()))
                            .map(|(i, _)| &file_stem[..i])
                    })
                    .unwrap_or(file_stem);
                let name_lower = name.to_lowercase();
                let feature_name = if name.is_empty() {
                    file_stem.to_string()
                } else {
                    to_title_case(&name_lower.replace('_', " "))
                };
                let rel = path
                    .strip_prefix(root)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string();
                features.push(DetectedFeature {
                    name: feature_name,
                    source_file: rel,
                    confidence: "medium".to_string(),
                });
                break;
            }
        }

        // Also detect paywall/onboarding by filename
        let lower = file_name.to_lowercase();
        for keyword in ["paywall", "onboarding", "settings", "analytics"] {
            if lower.contains(keyword) {
                let rel = path
                    .strip_prefix(root)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string();
                let name = to_title_case(keyword);
                if !features.iter().any(|f| f.name == name) {
                    features.push(DetectedFeature {
                        name,
                        source_file: rel,
                        confidence: "high".to_string(),
                    });
                }
            }
        }
    }

    features.sort_by(|a, b| a.name.cmp(&b.name));
    features.dedup_by(|a, b| a.name == b.name);
    Ok(features)
}

fn to_title_case(s: &str) -> String {
    let mut result = String::new();
    for (i, part) in s.split('_').enumerate() {
        if i > 0 {
            result.push(' ');
        }
        if let Some(first) = part.chars().next() {
            result.extend(first.to_uppercase());
            result.push_str(&part[1..]);
        }
    }
    if result.is_empty() {
        s.to_string()
    } else {
        result
    }
}
