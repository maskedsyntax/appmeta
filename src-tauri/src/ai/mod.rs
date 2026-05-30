pub mod types;
pub mod prompts;
pub mod providers;

use crate::ai::prompts::{build_system_prompt, build_user_prompt, parse_ai_response};
use crate::ai::providers::openai::OpenAiProvider;
use crate::ai::types::AiRequest;
use crate::error::{AppError, AppResult};
use crate::project::truth_file::GeneratedField;
use crate::project::ProjectTruthFile;
use crate::storage::AppSettings;

pub use prompts::ai_context_preview_items;

fn effective_max_tokens(model: &str, configured: u32) -> u32 {
    if providers::openai::uses_max_completion_tokens(model) {
        configured.max(8192)
    } else {
        configured.max(512)
    }
}

pub async fn generate_field(
    project: &ProjectTruthFile,
    field: &str,
    settings: &AppSettings,
) -> AppResult<GeneratedField> {
    if settings.api_key.is_empty() {
        return Err(AppError::MissingApiKey);
    }

    let request = AiRequest {
        field_name: field.to_string(),
        system_prompt: build_system_prompt(),
        user_prompt: build_user_prompt(project, field, settings),
        json_mode: true,
    };

    let effective_max = effective_max_tokens(&settings.model, settings.max_tokens);

    let provider = OpenAiProvider::new(
        settings.api_key.clone(),
        settings.base_url.clone(),
        settings.model.clone(),
        settings.temperature,
        effective_max,
    );

    let content = provider.generate(request).await?;
    if content.trim().is_empty() {
        return Err(AppError::AiProvider(
            "AI returned empty content. If using GPT-5/o-series, increase Max Tokens in Settings (try 8192+) or use gpt-4o-mini.".into(),
        ));
    }

    let parsed = parse_ai_response(field, &content);
    if parsed.value.is_empty() {
        return Err(AppError::AiProvider(format!(
            "AI did not produce text for '{}'. {}",
            field,
            parsed.warnings.first().cloned().unwrap_or_else(|| "Try gpt-4o-mini.".into())
        )));
    }
    Ok(parsed)
}

pub async fn test_ai_connection(settings: &AppSettings) -> AppResult<String> {
    providers::openai::test_connection(settings).await
}
