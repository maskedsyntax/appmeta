use crate::ai::types::AiRequest;
use crate::error::{AppError, AppResult};
use serde::Deserialize;
use serde_json::{json, Value};

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
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let body = build_request_body(
            &self.model,
            &request.system_prompt,
            &request.user_prompt,
            self.temperature,
            self.max_tokens,
            request.json_mode,
        );

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
            return Err(parse_api_error(status.as_u16(), &text));
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
            content: Option<String>,
        }

        let chat: ChatResponse = response.json().await?;
        chat.choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .filter(|c| !c.trim().is_empty())
            .ok_or_else(|| AppError::AiProvider("Empty response from AI provider".into()))
    }
}

pub fn uses_max_completion_tokens(model: &str) -> bool {
    let m = model.to_lowercase();
    m.starts_with("gpt-5")
        || m.starts_with("o1")
        || m.starts_with("o3")
        || m.starts_with("o4")
        || m.contains("gpt-5")
}

pub fn supports_temperature(model: &str) -> bool {
    let m = model.to_lowercase();
    !(m.starts_with("o1")
        || m.starts_with("o3")
        || m.starts_with("o4")
        || m.starts_with("gpt-5")
        || m.contains("gpt-5"))
}

pub fn supports_json_mode(model: &str) -> bool {
    let m = model.to_lowercase();
    !m.starts_with("o1") && !m.starts_with("o3") && !m.starts_with("o4")
}

fn prompts_mention_json(system_prompt: &str, user_prompt: &str) -> bool {
    format!("{system_prompt}{user_prompt}")
        .to_lowercase()
        .contains("json")
}

fn build_request_body(
    model: &str,
    system_prompt: &str,
    user_prompt: &str,
    temperature: f32,
    max_tokens: u32,
    json_mode: bool,
) -> Value {
    let mut body = json!({
        "model": model,
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": user_prompt }
        ],
    });

    if uses_max_completion_tokens(model) {
        body["max_completion_tokens"] = json!(max_tokens);
    } else {
        body["max_tokens"] = json!(max_tokens);
    }

    if supports_temperature(model) {
        body["temperature"] = json!(temperature);
    }

    if json_mode && supports_json_mode(model) && prompts_mention_json(system_prompt, user_prompt) {
        body["response_format"] = json!({ "type": "json_object" });
    }

    body
}

fn parse_api_error(status: u16, text: &str) -> AppError {
    if text.contains("max_tokens") && text.contains("max_completion_tokens") {
        return AppError::AiProvider(format!(
            "Model API error ({status}): this model needs max_completion_tokens. \
             Save settings again — AppMeta should handle this automatically. Raw: {text}"
        ));
    }
    if text.contains("invalid_api_key") || text.contains("Incorrect API key") {
        return AppError::AiProvider(
            "Invalid API key. Check your key in Settings.".into(),
        );
    }
    AppError::AiProvider(format!("API error {status}: {text}"))
}

pub async fn test_connection(settings: &crate::storage::AppSettings) -> AppResult<String> {
    let max = if uses_max_completion_tokens(&settings.model) {
        settings.max_tokens.max(256)
    } else {
        settings.max_tokens.max(16)
    };
    let provider = OpenAiProvider::new(
        settings.api_key.clone(),
        settings.base_url.clone(),
        settings.model.clone(),
        0.0,
        max,
    );
    provider
        .generate(AiRequest {
            field_name: "test".into(),
            system_prompt: "Reply with exactly: OK".into(),
            user_prompt: "ping".into(),
            json_mode: false,
        })
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpt5_uses_completion_tokens() {
        assert!(uses_max_completion_tokens("gpt-5"));
        assert!(uses_max_completion_tokens("gpt-5.2"));
        assert!(!uses_max_completion_tokens("gpt-4o-mini"));
    }

    #[test]
    fn gpt5_body_uses_max_completion_tokens() {
        let body = build_request_body("gpt-5", "Return json", "user", 0.7, 100, true);
        assert!(body.get("max_completion_tokens").is_some());
        assert!(body.get("max_tokens").is_none());
        assert!(body.get("temperature").is_none());
    }

    #[test]
    fn gpt4_body_uses_max_tokens_and_json_mode_when_prompt_mentions_json() {
        let body = build_request_body("gpt-4o-mini", "Return json", "user", 0.7, 100, true);
        assert!(body.get("max_tokens").is_some());
        assert!(body.get("temperature").is_some());
        assert_eq!(
            body.get("response_format").and_then(|v| v.get("type")),
            Some(&json!("json_object"))
        );
    }

    #[test]
    fn test_connection_skips_json_mode() {
        let body = build_request_body("gpt-4o-mini", "Reply OK", "ping", 0.0, 16, false);
        assert!(body.get("response_format").is_none());
    }
}
