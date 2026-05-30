use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRequest {
    pub field_name: String,
    pub system_prompt: String,
    pub user_prompt: String,
}
