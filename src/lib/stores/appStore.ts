import { writable, derived, get } from "svelte/store";
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
export const pendingGenerateQueue = writable<string[]>([]);

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
  const p = get(project);
  if (p) {
    warnings.set(await api.validateProject(p));
  }
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

export async function rescanCurrentProject() {
  const p = get(project);
  if (!p?.project.path) return;
  loading.set(true);
  error.set(null);
  try {
    const result = await api.scanProject(p.project.path);
    scanResult.set(result);
    pendingQuestions.set(result.questions);
    const updated = { ...p, scan_questions: result.questions };
    await persistProject(updated);
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
    pendingQuestions.set(p.scan_questions ?? []);
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

export async function deleteProjectById(id: string) {
  loading.set(true);
  error.set(null);
  try {
    await api.deleteProject(id);
    const current = get(project);
    if (current?.project.id === id) {
      project.set(null);
      scanResult.set(null);
      pendingQuestions.set([]);
      warnings.set([]);
    }
    recentProjects.set(await api.listProjects());
  } catch (e) {
    error.set(String(e));
  } finally {
    loading.set(false);
  }
}

export async function doGenerateField(fieldName: string) {
  const p = get(project);
  if (!p) return;
  error.set(null);
  try {
    const field = await api.generateField(p, fieldName);
    const updated = await api.updateGeneratedFields(get(project) ?? p, field);
    project.set(updated);
    await persistProject(updated);
  } catch (e) {
    error.set(String(e));
    throw e;
  }
}

export async function generateAllFields(fieldNames: string[]) {
  if (fieldNames.length === 0) return;
  const s = await api.loadSettings();
  settings.set(s);
  if (!s.context_preview_acknowledged) {
    pendingGenerateField.set(fieldNames[0]);
    pendingGenerateQueue.set(fieldNames.slice(1));
    showContextPreview.set(true);
    return;
  }
  loading.set(true);
  error.set(null);
  let failures = 0;
  try {
    for (const field of fieldNames) {
      try {
        await doGenerateField(field);
      } catch {
        failures += 1;
      }
    }
    if (failures > 0) {
      const ok = fieldNames.length - failures;
      error.set(
        ok > 0
          ? `Generated ${ok}/${fieldNames.length} fields. Check the error above and retry failed ones.`
          : `All ${fieldNames.length} fields failed to generate.`,
      );
    }
  } finally {
    loading.set(false);
  }
}

export async function requestGenerateField(fieldName: string) {
  await generateAllFields([fieldName]);
}

async function drainGenerateQueue() {
  const queue = get(pendingGenerateQueue);
  pendingGenerateQueue.set([]);
  if (queue.length === 0) return;
  loading.set(true);
  try {
    for (const field of queue) {
      await doGenerateField(field);
    }
  } finally {
    loading.set(false);
  }
}

export async function confirmContextPreview() {
  const s = await api.loadSettings();
  if (s) {
    s.context_preview_acknowledged = true;
    await api.saveSettings(s);
    settings.set(s);
  }
  showContextPreview.set(false);
  const field = get(pendingGenerateField);
  pendingGenerateField.set(null);
  if (field) {
    loading.set(true);
    try {
      await doGenerateField(field);
      await drainGenerateQueue();
    } finally {
      loading.set(false);
    }
  }
}

export async function saveSettingsStore(s: AppSettings) {
  await api.saveSettings(s);
  settings.set(s);
}

export async function resetContextPreviewAck() {
  const s = await api.loadSettings();
  s.context_preview_acknowledged = false;
  await saveSettingsStore(s);
}
