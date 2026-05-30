import { invoke } from "@tauri-apps/api/core";
import type {
  AppSettings,
  ConfirmationQuestion,
  GeneratedField,
  ProjectScanResult,
  ProjectSummary,
  ProjectTruthFile,
  ValidationWarning,
} from "$lib/types";

export async function pickProjectFolder(): Promise<string | null> {
  return invoke<string | null>("pick_project_folder");
}

export async function pickSaveFolder(): Promise<string | null> {
  return invoke<string | null>("pick_save_folder");
}

export async function scanProject(path: string): Promise<ProjectScanResult> {
  return invoke("scan_project_cmd", { path });
}

export async function createProjectFromScan(
  scan: ProjectScanResult,
): Promise<ProjectTruthFile> {
  return invoke("create_project_from_scan_cmd", { scan });
}

export async function refreshProjectFromScan(
  project: ProjectTruthFile,
  scan: ProjectScanResult,
): Promise<ProjectTruthFile> {
  return invoke("refresh_project_from_scan_cmd", { project, scan });
}

export async function saveProject(project: ProjectTruthFile): Promise<void> {
  return invoke("save_project_cmd", { project });
}

export async function loadProject(projectId: string): Promise<ProjectTruthFile> {
  return invoke("load_project_cmd", { projectId });
}

export async function listProjects(): Promise<ProjectSummary[]> {
  return invoke("list_projects_cmd");
}

export async function deleteProject(projectId: string): Promise<void> {
  return invoke("delete_project_cmd", { projectId });
}

export async function updateFact(
  project: ProjectTruthFile,
  factId: string,
  status: string,
): Promise<ProjectTruthFile> {
  return invoke("update_fact_cmd", { project, factId, status });
}

export async function answerQuestion(
  project: ProjectTruthFile,
  questionId: string,
  answer: string,
): Promise<ProjectTruthFile> {
  return invoke("answer_question_cmd", { project, questionId, answer });
}

export async function loadSettings(): Promise<AppSettings> {
  return invoke("load_settings_cmd");
}

export async function saveSettings(settings: AppSettings): Promise<void> {
  return invoke("save_settings_cmd", { settings });
}

export async function maskApiKey(key: string): Promise<string> {
  return invoke("mask_api_key_cmd", { key });
}

export async function generateField(
  project: ProjectTruthFile,
  field: string,
): Promise<GeneratedField> {
  return invoke("generate_field_cmd", { project, field });
}

export async function testAiConnection(): Promise<string> {
  return invoke("test_ai_connection_cmd");
}

export async function validateProject(
  project: ProjectTruthFile,
): Promise<ValidationWarning[]> {
  return invoke("validate_project_cmd", { project });
}

export async function exportMarkdown(
  project: ProjectTruthFile,
  fields: GeneratedField[],
): Promise<string> {
  return invoke("export_markdown_cmd", { project, fields });
}

export async function exportJson(
  project: ProjectTruthFile,
  fields: GeneratedField[],
): Promise<string> {
  return invoke("export_json_cmd", { project, fields });
}

export async function saveSubmissionPack(
  project: ProjectTruthFile,
  fields: GeneratedField[],
  directory: string,
): Promise<string[]> {
  return invoke("save_submission_pack_cmd", { project, fields, directory });
}

export async function getAiContextPreview(
  project: ProjectTruthFile,
): Promise<string[]> {
  return invoke("get_ai_context_preview", { project });
}

export async function updateGeneratedFields(
  project: ProjectTruthFile,
  field: GeneratedField,
): Promise<ProjectTruthFile> {
  return invoke("update_generated_fields_cmd", { project, field });
}

export type { ConfirmationQuestion };
