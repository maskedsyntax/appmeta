use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRequest {
    pub field_name: String,
    pub system_prompt: String,
    pub user_prompt: String,
    /// When true, request JSON output (OpenAI requires "json" in the prompt text).
    #[serde(default)]
    pub json_mode: bool,
}
