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
    };

    let provider = OpenAiProvider::new(
        settings.api_key.clone(),
        settings.base_url.clone(),
        settings.model.clone(),
        settings.temperature,
        settings.max_tokens,
    );

    let content = provider.generate(request).await?;
    Ok(parse_ai_response(field, &content))
}

pub async fn test_ai_connection(settings: &AppSettings) -> AppResult<String> {
    providers::openai::test_connection(settings).await
}
