use crate::error::AppResult;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub provider: String,
    pub api_key: String,
    pub model: String,
    pub base_url: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub tone: String,
    pub context_preview_acknowledged: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            api_key: String::new(),
            model: "gpt-4o-mini".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            temperature: 0.7,
            max_tokens: 2048,
            tone: "professional".to_string(),
            context_preview_acknowledged: false,
        }
    }
}

pub fn settings_path(data_dir: &Path) -> PathBuf {
    data_dir.join("settings.json")
}

pub fn load_settings(data_dir: &Path) -> AppResult<AppSettings> {
    let path = settings_path(data_dir);
    if !path.exists() {
        return Ok(AppSettings::default());
    }
    let content = std::fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&content)?)
}

pub fn save_settings(data_dir: &Path, settings: &AppSettings) -> AppResult<()> {
    std::fs::create_dir_all(data_dir)?;
    let path = settings_path(data_dir);
    let content = serde_json::to_string_pretty(settings)?;
    std::fs::write(path, content)?;
    Ok(())
}

pub fn mask_api_key(key: &str) -> String {
    if key.len() <= 8 {
        return "*".repeat(key.len());
    }
    format!("{}...{}", &key[..4], &key[key.len() - 4..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks_key() {
        assert_eq!(mask_api_key("sk-abcdefghijklmnop"), "sk-a...mnop");
    }
}
