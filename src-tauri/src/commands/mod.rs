use crate::error::AppResult;
use crate::project::{
    answer_question, create_project_from_scan, update_fact_status, validate_project,
    GeneratedField, ProjectTruthFile, ValidationWarning,
};
use crate::scanner::{scan_project, ProjectScanResult};
use crate::storage::{self, AppSettings, ProjectSummary};
use std::path::PathBuf;
use tauri::Manager;

fn data_dir(app: &tauri::AppHandle) -> AppResult<PathBuf> {
    app.path()
        .app_data_dir()
        .map_err(|e| crate::error::AppError::Message(e.to_string()))
}

#[tauri::command]
pub async fn pick_project_folder(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let folder = app
        .dialog()
        .file()
        .set_title("Select Project Folder")
        .blocking_pick_folder();
    Ok(folder.map(|p| p.to_string()))
}

#[tauri::command]
pub async fn scan_project_cmd(path: String) -> Result<ProjectScanResult, String> {
    scan_project(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_project_from_scan_cmd(
    scan: ProjectScanResult,
) -> Result<ProjectTruthFile, String> {
    Ok(create_project_from_scan(scan))
}

#[tauri::command]
pub async fn save_project_cmd(
    app: tauri::AppHandle,
    mut project: ProjectTruthFile,
) -> Result<(), String> {
    project.touch();
    storage::save_project(&data_dir(&app).map_err(|e| e.to_string())?, &project)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_project_cmd(
    app: tauri::AppHandle,
    project_id: String,
) -> Result<ProjectTruthFile, String> {
    storage::load_project(&data_dir(&app).map_err(|e| e.to_string())?, &project_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_projects_cmd(app: tauri::AppHandle) -> Result<Vec<ProjectSummary>, String> {
    storage::list_projects(&data_dir(&app).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_project_cmd(app: tauri::AppHandle, project_id: String) -> Result<(), String> {
    storage::delete_project(&data_dir(&app).map_err(|e| e.to_string())?, &project_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_fact_cmd(
    mut project: ProjectTruthFile,
    fact_id: String,
    status: String,
) -> Result<ProjectTruthFile, String> {
    update_fact_status(&mut project, &fact_id, &status);
    Ok(project)
}

#[tauri::command]
pub async fn answer_question_cmd(
    mut project: ProjectTruthFile,
    question_id: String,
    answer: String,
) -> Result<ProjectTruthFile, String> {
    answer_question(&mut project, &question_id, &answer);
    Ok(project)
}

#[tauri::command]
pub async fn load_settings_cmd(app: tauri::AppHandle) -> Result<AppSettings, String> {
    storage::load_settings(&data_dir(&app).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_settings_cmd(app: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    storage::save_settings(&data_dir(&app).map_err(|e| e.to_string())?, &settings)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mask_api_key_cmd(key: String) -> Result<String, String> {
    Ok(storage::mask_api_key(&key))
}

#[tauri::command]
pub fn refresh_project_from_scan_cmd(
    project: ProjectTruthFile,
    scan: crate::scanner::ProjectScanResult,
) -> Result<ProjectTruthFile, String> {
    Ok(crate::project::refresh_project_from_scan(project, scan))
}

#[tauri::command]
pub async fn generate_field_cmd(
    app: tauri::AppHandle,
    project: ProjectTruthFile,
    field: String,
) -> Result<GeneratedField, String> {
    let settings = storage::load_settings(&data_dir(&app).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    crate::ai::generate_field(&project, &field, &settings)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_ai_connection_cmd(app: tauri::AppHandle) -> Result<String, String> {
    let settings = storage::load_settings(&data_dir(&app).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    crate::ai::test_ai_connection(&settings)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn validate_project_cmd(
    project: ProjectTruthFile,
) -> Result<Vec<ValidationWarning>, String> {
    Ok(validate_project(&project))
}

#[tauri::command]
pub async fn export_markdown_cmd(
    project: ProjectTruthFile,
    fields: Vec<GeneratedField>,
) -> Result<String, String> {
    Ok(crate::export::export_markdown(&project, &fields))
}

#[tauri::command]
pub async fn export_json_cmd(
    project: ProjectTruthFile,
    fields: Vec<GeneratedField>,
) -> Result<String, String> {
    Ok(crate::export::export_json(&project, &fields))
}

#[tauri::command]
pub async fn save_submission_pack_cmd(
    _app: tauri::AppHandle,
    project: ProjectTruthFile,
    fields: Vec<GeneratedField>,
    directory: String,
) -> Result<Vec<String>, String> {
    let md = crate::export::export_markdown(&project, &fields);
    let json = crate::export::export_json(&project, &fields);
    let dir = PathBuf::from(&directory);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let md_path = dir.join("submission-pack.md");
    let json_path = dir.join("submission-pack.json");
    std::fs::write(&md_path, md).map_err(|e| e.to_string())?;
    std::fs::write(&json_path, json).map_err(|e| e.to_string())?;
    Ok(vec![
        md_path.to_string_lossy().to_string(),
        json_path.to_string_lossy().to_string(),
    ])
}

#[tauri::command]
pub async fn pick_save_folder(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let folder = app
        .dialog()
        .file()
        .set_title("Select Export Folder")
        .blocking_pick_folder();
    Ok(folder.map(|p| p.to_string()))
}

#[tauri::command]
pub async fn get_ai_context_preview(
    project: ProjectTruthFile,
) -> Result<Vec<String>, String> {
    Ok(crate::ai::ai_context_preview_items(&project))
}

#[tauri::command]
pub async fn update_generated_fields_cmd(
    mut project: ProjectTruthFile,
    field: GeneratedField,
) -> Result<ProjectTruthFile, String> {
    if let Some(existing) = project
        .generated_fields
        .iter_mut()
        .find(|f| f.field == field.field)
    {
        *existing = field;
    } else {
        project.generated_fields.push(field);
    }
    project.touch();
    Ok(project)
}
