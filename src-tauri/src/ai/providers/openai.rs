use crate::ai::types::AiRequest;
use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};

pub struct OpenAiProvider {
    api_key: String,
    base_url: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
}

impl OpenAiProvider {
    pub fn new(
        api_key: String,
        base_url: String,
        model: String,
        temperature: f32,
        max_tokens: u32,
    ) -> Self {
        Self {
            api_key,
            base_url: if base_url.is_empty() {
                "https://api.openai.com/v1".to_string()
            } else {
                base_url
            },
            model,
            temperature,
            max_tokens,
        }
    }

    pub async fn generate(&self, request: AiRequest) -> AppResult<String> {
        #[derive(Serialize)]
        struct ChatRequest {
            model: String,
            messages: Vec<Message>,
            temperature: f32,
            max_tokens: u32,
        }

        #[derive(Serialize)]
        struct Message {
            role: String,
            content: String,
        }

        #[derive(Deserialize)]
        struct ChatResponse {
            choices: Vec<Choice>,
        }

        #[derive(Deserialize)]
        struct Choice {
            message: ResponseMessage,
        }

        #[derive(Deserialize)]
        struct ResponseMessage {
            content: String,
        }

        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let body = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "system".into(),
                    content: request.system_prompt,
                },
                Message {
                    role: "user".into(),
                    content: request.user_prompt,
                },
            ],
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::AiProvider(format!(
                "API error {status}: {text}"
            )));
        }

        let chat: ChatResponse = response.json().await?;
        chat.choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| AppError::AiProvider("Empty response from AI provider".into()))
    }
}

pub async fn test_connection(settings: &crate::storage::AppSettings) -> AppResult<String> {
    let provider = OpenAiProvider::new(
        settings.api_key.clone(),
        settings.base_url.clone(),
        settings.model.clone(),
        0.0,
        16,
    );
    provider
        .generate(AiRequest {
            field_name: "test".into(),
            system_prompt: "Reply with exactly: OK".into(),
            user_prompt: "ping".into(),
        })
        .await
}
