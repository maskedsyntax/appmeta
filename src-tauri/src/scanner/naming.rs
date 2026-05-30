use std::path::Path;

/// Resolve a human-facing app name from scan signals (not the pubspec package id).
pub fn resolve_display_name(
    project_root: &Path,
    package_name: Option<&str>,
    ios_display_name: Option<&str>,
    ios_bundle_name: Option<&str>,
    bundle_id: Option<&str>,
    readme_title: Option<&str>,
) -> Option<String> {
    if let Some(name) = ios_display_name.filter(|s| is_good_display_name(s)) {
        return Some(name.to_string());
    }
    if let Some(name) = ios_bundle_name.filter(|s| is_good_display_name(s)) {
        return Some(name.to_string());
    }
    if let Some(title) = readme_title.filter(|s| is_good_display_name(s)) {
        return Some(title.to_string());
    }
    if let Some(id) = bundle_id {
        if let Some(segment) = id.rsplit('.').next() {
            let name = humanize_token(segment);
            if is_good_display_name(&name) {
                return Some(name);
            }
        }
    }
    if let Some(pkg) = package_name {
        if looks_like_workspace_package(pkg) {
            if let Some(folder) = project_root.file_name().and_then(|n| n.to_str()) {
                let name = humanize_token(folder);
                if is_good_display_name(&name) {
                    return Some(name);
                }
            }
        }
        let name = humanize_package_name(pkg);
        if is_good_display_name(&name) {
            return Some(name);
        }
    }
    project_root
        .file_name()
        .and_then(|n| n.to_str())
        .map(humanize_token)
        .filter(|s| is_good_display_name(s))
}

fn looks_like_workspace_package(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.ends_with("_workspace")
        || lower.ends_with("_root")
        || lower.contains("workspace")
        || lower.ends_with("_monorepo")
}

fn humanize_package_name(name: &str) -> String {
    let base = name
        .strip_suffix("_workspace")
        .or_else(|| name.strip_suffix("_root"))
        .unwrap_or(name);
    humanize_token(base)
}

pub fn humanize_token(token: &str) -> String {
    token
        .split(['_', '-'])
        .filter(|p| !p.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_good_display_name(name: &str) -> bool {
    let trimmed = name.trim();
    !trimmed.is_empty()
        && trimmed.len() >= 2
        && !trimmed.eq_ignore_ascii_case("Runner")
        && !trimmed.eq_ignore_ascii_case("App")
        && !looks_like_workspace_package(trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn workspace_package_uses_folder_name() {
        let root = Path::new("/Users/dev/tickle");
        let name = resolve_display_name(
            root,
            Some("tickle_workspace"),
            None,
            None,
            Some("com.maskedsyntax.tickle"),
            None,
        );
        assert_eq!(name.as_deref(), Some("Tickle"));
    }

    #[test]
    fn prefers_plist_display_name() {
        let root = Path::new("/Users/dev/tickle");
        let name = resolve_display_name(
            root,
            Some("tickle_workspace"),
            Some("Steepr"),
            None,
            None,
            None,
        );
        assert_eq!(name.as_deref(), Some("Steepr"));
    }

    #[test]
    fn humanizes_snake_case() {
        assert_eq!(humanize_token("my_cool_app"), "My Cool App");
    }
}
