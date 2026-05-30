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
