import { writable, derived } from "svelte/store";
import type {
  AppSettings,
  ConfirmationQuestion,
  GeneratedField,
  ProjectScanResult,
  ProjectSummary,
  ProjectTruthFile,
  ValidationWarning,
} from "$lib/types";
import * as api from "$lib/api/tauri";

export const project = writable<ProjectTruthFile | null>(null);
export const scanResult = writable<ProjectScanResult | null>(null);
export const pendingQuestions = writable<ConfirmationQuestion[]>([]);
export const settings = writable<AppSettings | null>(null);
export const warnings = writable<ValidationWarning[]>([]);
export const recentProjects = writable<ProjectSummary[]>([]);
export const loading = writable(false);
export const error = writable<string | null>(null);
export const showContextPreview = writable(false);
export const pendingGenerateField = writable<string | null>(null);

export const generatedFields = derived(project, ($p) => $p?.generated_fields ?? []);

export async function initApp() {
  try {
    settings.set(await api.loadSettings());
    recentProjects.set(await api.listProjects());
  } catch (e) {
    error.set(String(e));
  }
}

export async function refreshWarnings() {
  const p = getProject();
  if (p) {
    warnings.set(await api.validateProject(p));
  }
}

function getProject(): ProjectTruthFile | null {
  let p: ProjectTruthFile | null = null;
  project.subscribe((v) => (p = v))();
  return p;
}

export async function persistProject(p: ProjectTruthFile) {
  await api.saveProject(p);
  project.set(p);
  recentProjects.set(await api.listProjects());
  await refreshWarnings();
}

export async function runScan(path: string) {
  loading.set(true);
  error.set(null);
  try {
    const result = await api.scanProject(path);
    scanResult.set(result);
    pendingQuestions.set(result.questions);
    const p = await api.createProjectFromScan(result);
    project.set(p);
    await persistProject(p);
  } catch (e) {
    error.set(String(e));
  } finally {
    loading.set(false);
  }
}

export async function openProject(id: string) {
  loading.set(true);
  error.set(null);
  try {
    const p = await api.loadProject(id);
    project.set(p);
    if (p.project.path) {
      try {
        const result = await api.scanProject(p.project.path);
        scanResult.set(result);
        pendingQuestions.set(result.questions);
      } catch {
        scanResult.set(null);
      }
    }
    await refreshWarnings();
  } catch (e) {
    error.set(String(e));
  } finally {
    loading.set(false);
  }
}

export async function doGenerateField(fieldName: string) {
  const p = getProject();
  if (!p) return;
  loading.set(true);
  error.set(null);
  try {
    const field = await api.generateField(p, fieldName);
    const updated = await api.updateGeneratedFields(p, field);
    project.set(updated);
    await persistProject(updated);
  } catch (e) {
    error.set(String(e));
  } finally {
    loading.set(false);
    pendingGenerateField.set(null);
  }
}

export async function requestGenerateField(fieldName: string) {
  const s = await api.loadSettings();
  settings.set(s);
  if (!s.context_preview_acknowledged) {
    pendingGenerateField.set(fieldName);
    showContextPreview.set(true);
    return;
  }
  await doGenerateField(fieldName);
}

export async function confirmContextPreview() {
  const s = await api.loadSettings();
  if (s) {
    s.context_preview_acknowledged = true;
    await api.saveSettings(s);
    settings.set(s);
  }
  showContextPreview.set(false);
  const field = getPendingField();
  if (field) await doGenerateField(field);
}

function getPendingField(): string | null {
  let f: string | null = null;
  pendingGenerateField.subscribe((v) => (f = v))();
  return f;
}

export async function saveSettingsStore(s: AppSettings) {
  await api.saveSettings(s);
  settings.set(s);
}
