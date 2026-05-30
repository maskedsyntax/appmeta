use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Plist error: {0}")]
    Plist(#[from] plist::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("{0}")]
    Message(String),

    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("Invalid project folder: {0}")]
    InvalidProject(String),

    #[error("AI provider error: {0}")]
    AiProvider(String),

    #[error("Missing AI API key. Configure your provider in Settings.")]
    MissingApiKey,
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Message(s)
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Message(s.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
