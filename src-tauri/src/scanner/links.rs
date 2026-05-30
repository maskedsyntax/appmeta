use crate::error::AppResult;
use crate::scanner::types::DetectedUrl;
use std::path::Path;

const DOC_FILES: &[&str] = &["privacy.md", "PRIVACY.md", "README.md", "spec.md", "terms.md"];

pub fn detect_urls(root: &Path) -> AppResult<Vec<DetectedUrl>> {
    let mut found = Vec::new();

    for name in DOC_FILES {
        let path = root.join(name);
        if !path.exists() {
            continue;
        }
        let content = std::fs::read_to_string(&path)?;
        let urls = extract_urls(&content);
        let file = name.to_string();
        let is_privacy_doc = name.to_lowercase().contains("privacy");

        for url in urls {
            let lower = url.to_lowercase();
            let is_privacy = is_privacy_doc
                || lower.contains("privacy")
                || lower.contains("/privacy")
                || lower.contains("privacy-policy");

            if is_privacy {
                found.push(DetectedUrl {
                    url: url.clone(),
                    kind: "privacy_policy".into(),
                    source_file: file.clone(),
                    confidence: if is_privacy_doc || lower.contains("privacy") {
                        "high".into()
                    } else {
                        "medium".into()
                    },
                });
            } else if lower.contains("support") || lower.contains("help") {
                found.push(DetectedUrl {
                    url,
                    kind: "support".into(),
                    source_file: file.clone(),
                    confidence: "medium".into(),
                });
            }
        }
    }

    // pubspec homepage / repository
    let pubspec = root.join("pubspec.yaml");
    if pubspec.exists() {
        if let Ok(content) = std::fs::read_to_string(&pubspec) {
            if let Ok(doc) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                for key in ["homepage", "repository", "issue_tracker"] {
                    if let Some(url) = doc.get(key).and_then(|v| v.as_str()) {
                        if url.starts_with("http") {
                            let lower = url.to_lowercase();
                            if lower.contains("privacy") {
                                found.push(DetectedUrl {
                                    url: url.to_string(),
                                    kind: "privacy_policy".into(),
                                    source_file: "pubspec.yaml".into(),
                                    confidence: "medium".into(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    dedupe_urls(&mut found);
    Ok(found)
}

pub fn best_privacy_policy_url(urls: &[DetectedUrl]) -> Option<&DetectedUrl> {
    urls.iter()
        .filter(|u| u.kind == "privacy_policy")
        .max_by_key(|u| match u.confidence.as_str() {
            "high" => 2,
            "medium" => 1,
            _ => 0,
        })
}

fn extract_urls(content: &str) -> Vec<String> {
    let mut urls = Vec::new();
    for token in content.split_whitespace() {
        let mut cleaned = token
            .trim_matches(|c: char| {
                matches!(
                    c,
                    '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '"' | '\'' | ',' | ';'
                )
            })
            .to_string();
        // markdown link target: [text](url)
        if let Some(idx) = cleaned.find("http") {
            cleaned = cleaned[idx..].to_string();
            cleaned = cleaned
                .trim_end_matches(|c: char| {
                    !c.is_ascii_alphanumeric() && c != '/' && c != '-' && c != '_' && c != '.'
                })
                .to_string();
        }
        if (cleaned.starts_with("http://") || cleaned.starts_with("https://"))
            && !urls.contains(&cleaned)
        {
            urls.push(cleaned);
        }
    }
    urls
}

fn dedupe_urls(urls: &mut Vec<DetectedUrl>) {
    let mut seen = std::collections::HashSet::new();
    urls.retain(|u| seen.insert(u.url.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_privacy_url_in_readme() {
        let text = "See our [Privacy Policy](https://example.com/privacy) for details.";
        let urls = extract_urls(text);
        assert!(urls.iter().any(|u| u.contains("example.com/privacy")));
    }

    #[test]
    fn privacy_md_url_is_privacy_kind() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("privacy.md"),
            "Policy: https://myapp.com/privacy-policy\n",
        )
        .unwrap();
        let urls = detect_urls(dir.path()).unwrap();
        assert_eq!(urls.len(), 1);
        assert_eq!(urls[0].kind, "privacy_policy");
    }
}
