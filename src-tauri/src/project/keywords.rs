use std::collections::HashSet;

/// Pack App Store keywords to use as much of the 100-char budget as possible.
pub fn optimize_keywords(value: &str, app_name: &str, extra_terms: &[String], max: usize) -> String {
    let app_tokens: HashSet<String> = app_name
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| t.len() >= 2)
        .map(String::from)
        .collect();

    let mut terms: Vec<String> = Vec::new();
    let mut seen = HashSet::new();

    for part in value.split(',') {
        push_term(part, &app_tokens, &mut terms, &mut seen);
    }
    for term in extra_terms {
        push_term(term, &app_tokens, &mut terms, &mut seen);
    }

    let mut packed = pack_terms(&terms, max);

    if packed.chars().count() < max * 85 / 100 {
        for fallback in [
            "counter",
            "count",
            "tracker",
            "tool",
            "productivity",
            "utility",
            "simple",
            "fast",
            "instant",
            "mobile",
            "ios",
        ] {
            push_term(fallback, &app_tokens, &mut terms, &mut seen);
        }
        packed = pack_terms(&terms, max);
    }

    packed
}

fn push_term(
    raw: &str,
    app_tokens: &HashSet<String>,
    terms: &mut Vec<String>,
    seen: &mut HashSet<String>,
) {
    let term = raw.trim().to_lowercase();
    if term.len() < 2 || term.len() > 30 {
        return;
    }
    if app_tokens.contains(&term) {
        return;
    }
    if seen.insert(term.clone()) {
        terms.push(term);
    }
}

fn pack_terms(terms: &[String], max: usize) -> String {
    let mut result = String::new();
    for term in terms {
        let candidate = if result.is_empty() {
            term.clone()
        } else {
            format!("{result},{term}")
        };
        if candidate.chars().count() <= max {
            result = candidate;
        } else {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_toward_limit() {
        let out = optimize_keywords("counting,instant", "Tickle", &[], 100);
        assert!(out.chars().count() <= 100);
        assert!(out.chars().count() >= 50);
        assert!(out.contains("counting"));
        assert!(!out.contains("tickle"));
    }

    #[test]
    fn never_exceeds_limit() {
        let long = (0..30).map(|i| format!("term{i}")).collect::<Vec<_>>().join(",");
        let out = optimize_keywords(&long, "App", &[], 100);
        assert!(out.chars().count() <= 100);
    }
}
