use crate::scanner::DocumentSummary;
use crate::error::AppResult;
use std::path::Path;

const DOC_FILES: &[&str] = &["README.md", "CHANGELOG.md", "spec.md", "privacy.md"];

pub fn scan_documents(root: &Path) -> AppResult<Vec<DocumentSummary>> {
    let mut summaries = Vec::new();
    for name in DOC_FILES {
        let path = root.join(name);
        if path.exists() {
            if let Ok(summary) = read_document_summary(&path, name) {
                summaries.push(summary);
            }
        }
    }
    Ok(summaries)
}

pub fn extract_markdown_title(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(title) = trimmed.strip_prefix("# ") {
            let title = title.trim();
            if !title.is_empty() {
                return Some(title.to_string());
            }
        }
    }
    None
}

pub fn extract_summary_excerpt(content: &str, max_chars: usize) -> String {
    let mut parts = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with("```")
            || trimmed.starts_with(">")
            || trimmed.starts_with("---")
        {
            continue;
        }
        parts.push(trimmed);
        let joined = parts.join(" ");
        if joined.chars().count() >= max_chars {
            return truncate_chars(&joined, max_chars);
        }
    }
    parts.join(" ")
}

fn truncate_chars(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    s.chars().take(max.saturating_sub(3)).collect::<String>() + "..."
}

fn read_document_summary(path: &Path, file_name: &str) -> AppResult<DocumentSummary> {
    let content = std::fs::read_to_string(path)?;
    let line_count = content.lines().count();
    let first_paragraph = content
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .take(3)
        .collect::<Vec<_>>()
        .join(" ");
    let first_paragraph = if first_paragraph.len() > 500 {
        format!("{}...", &first_paragraph[..500])
    } else {
        first_paragraph
    };

    Ok(DocumentSummary {
        file_name: file_name.to_string(),
        first_paragraph,
        line_count,
    })
}
