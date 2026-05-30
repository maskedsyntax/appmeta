use crate::scanner::DetectedPermission;
use crate::scanner::permissions as perm_map;
use crate::error::{AppError, AppResult};
use std::path::Path;

pub struct IosInfo {
    pub bundle_id: Option<String>,
    pub display_name: Option<String>,
    pub bundle_name: Option<String>,
    pub version: Option<String>,
    pub build_number: Option<String>,
    pub min_os_version: Option<String>,
    pub permissions: Vec<DetectedPermission>,
}

pub fn parse_info_plist(path: &Path) -> AppResult<IosInfo> {
    let file = std::fs::File::open(path)?;
    let plist_value: plist::Value = plist::from_reader(file)?;

    let dict = match plist_value {
        plist::Value::Dictionary(d) => d,
        _ => return Err(AppError::Message("Info.plist is not a dictionary".into())),
    };

    let bundle_id = dict
        .get("CFBundleIdentifier")
        .and_then(|v| v.as_string())
        .map(String::from);
    let display_name = dict
        .get("CFBundleDisplayName")
        .and_then(|v| v.as_string())
        .map(String::from);
    let bundle_name = dict
        .get("CFBundleName")
        .and_then(|v| v.as_string())
        .map(String::from);
    let version = dict
        .get("CFBundleShortVersionString")
        .and_then(|v| v.as_string())
        .map(String::from);
    let build_number = dict
        .get("CFBundleVersion")
        .and_then(|v| v.as_string())
        .map(String::from);
    let min_os_version = dict
        .get("MinimumOSVersion")
        .and_then(|v| v.as_string())
        .map(String::from);

    let permission_keys = perm_map::ios_usage_keys();
    let platform = if path.to_string_lossy().contains("macos") {
        "macOS"
    } else {
        "iOS"
    };

    let detected_from = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Info.plist".to_string());

    let mut detected_permissions = Vec::new();
    for (key, permission_name) in &permission_keys {
        if dict.contains_key(key.as_str()) {
            detected_permissions.push(DetectedPermission {
                permission: permission_name.clone(),
                platform: platform.to_string(),
                detected_from: detected_from.clone(),
                plist_key: Some(key.clone()),
            });
        }
    }

    Ok(IosInfo {
        bundle_id,
        display_name,
        bundle_name,
        version,
        build_number,
        min_os_version,
        permissions: detected_permissions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn parses_plist_permissions() {
        let mut f = NamedTempFile::new().unwrap();
        write!(
            f,
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>CFBundleIdentifier</key><string>com.test.app</string>
  <key>NSMicrophoneUsageDescription</key><string>Record audio</string>
</dict></plist>"#
        )
        .unwrap();
        let info = parse_info_plist(f.path()).unwrap();
        assert_eq!(info.bundle_id.as_deref(), Some("com.test.app"));
        assert_eq!(info.permissions.len(), 1);
        assert_eq!(info.permissions[0].permission, "Microphone");
    }
}
