use crate::error::{AppError, AppResult};
use crate::project::ProjectTruthFile;
use std::path::{Path, PathBuf};

pub fn projects_dir(data_dir: &Path) -> PathBuf {
    data_dir.join("projects")
}

fn project_path(data_dir: &Path, project_id: &str) -> PathBuf {
    projects_dir(data_dir).join(format!("{project_id}.json"))
}

pub fn save_project(data_dir: &Path, project: &ProjectTruthFile) -> AppResult<()> {
    std::fs::create_dir_all(projects_dir(data_dir))?;
    let path = project_path(data_dir, &project.project.id);
    let content = serde_json::to_string_pretty(project)?;
    std::fs::write(path, content)?;
    Ok(())
}

pub fn load_project(data_dir: &Path, project_id: &str) -> AppResult<ProjectTruthFile> {
    let path = project_path(data_dir, project_id);
    if !path.exists() {
        return Err(AppError::ProjectNotFound(project_id.to_string()));
    }
    let content = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProjectSummary {
    pub id: String,
    pub name: String,
    pub path: String,
    pub updated_at: String,
}

pub fn list_projects(data_dir: &Path) -> AppResult<Vec<ProjectSummary>> {
    let dir = projects_dir(data_dir);
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut projects = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(project) = serde_json::from_str::<ProjectTruthFile>(&content) {
                projects.push(ProjectSummary {
                    id: project.project.id,
                    name: project.project.name,
                    path: project.project.path,
                    updated_at: project.project.updated_at,
                });
            }
        }
    }
    projects.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(projects)
}

pub fn delete_project(data_dir: &Path, project_id: &str) -> AppResult<()> {
    let path = project_path(data_dir, project_id);
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}
