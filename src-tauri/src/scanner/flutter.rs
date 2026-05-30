use crate::scanner::DetectedDependency;
use crate::error::AppResult;
use std::path::Path;

pub struct PubspecInfo {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub dependencies: Vec<DetectedDependency>,
}

pub fn parse_pubspec(path: &Path) -> AppResult<PubspecInfo> {
    let content = std::fs::read_to_string(path)?;
    let doc: serde_yaml::Value = serde_yaml::from_str(&content)?;

    let name = doc.get("name").and_then(|v| v.as_str()).map(String::from);
    let version = doc
        .get("version")
        .and_then(|v| v.as_str())
        .map(String::from);
    let description = doc
        .get("description")
        .and_then(|v| v.as_str())
        .map(String::from);

    let mut dependencies = Vec::new();
    if let Some(deps) = doc.get("dependencies").and_then(|v| v.as_mapping()) {
        for (key, value) in deps {
            let dep_name = key.as_str().unwrap_or("").to_string();
            if dep_name == "flutter" {
                continue;
            }

            let version_str = match value {
                serde_yaml::Value::String(s) => Some(s.clone()),
                serde_yaml::Value::Mapping(m) => m
                    .get(serde_yaml::Value::String("version".into()))
                    .and_then(|v| v.as_str())
                    .map(String::from),
                _ => None,
            };

            dependencies.push(DetectedDependency {
                name: dep_name,
                version: version_str,
                source_file: "pubspec.yaml".to_string(),
                category: None,
                risk: None,
                requires_confirmation: false,
            });
        }
    }

    Ok(PubspecInfo {
        name,
        version,
        description,
        dependencies,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn parses_pubspec_dependencies() {
        let mut f = NamedTempFile::new().unwrap();
        write!(
            f,
            r#"name: myapp
version: 2.0.0
dependencies:
  flutter:
    sdk: flutter
  drift: ^2.0.0
"#
        )
        .unwrap();
        let info = parse_pubspec(f.path()).unwrap();
        assert_eq!(info.name.as_deref(), Some("myapp"));
        assert_eq!(info.dependencies.len(), 1);
        assert_eq!(info.dependencies[0].name, "drift");
    }
}
