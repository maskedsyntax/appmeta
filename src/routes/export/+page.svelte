<script lang="ts">
  import { project } from "$lib/stores/appStore";
  import {
    exportMarkdown,
    exportJson,
    saveSubmissionPack,
    pickSaveFolder,
  } from "$lib/api/tauri";

  let mdPreview = $state("");
  let jsonPreview = $state("");
  let savedPaths = $state<string[]>([]);
  let tab = $state<"markdown" | "json">("markdown");

  async function refreshPreview() {
    if (!$project) return;
    mdPreview = await exportMarkdown($project, $project.generated_fields);
    jsonPreview = await exportJson($project, $project.generated_fields);
  }

  $effect(() => {
    if ($project) refreshPreview();
  });

  async function copyAll() {
    const text = tab === "markdown" ? mdPreview : jsonPreview;
    await navigator.clipboard.writeText(text);
  }

  async function saveToFolder() {
    if (!$project) return;
    const dir = await pickSaveFolder();
    if (dir) {
      savedPaths = await saveSubmissionPack($project, $project.generated_fields, dir);
    }
  }
</script>

<h2>Export</h2>
<p>Export your submission pack as Markdown or JSON.</p>

{#if !$project}
  <p class="empty">No project loaded. Generate fields first.</p>
{:else}
  <div class="actions">
    <button type="button" onclick={refreshPreview}>Refresh Preview</button>
    <button type="button" onclick={copyAll}>Copy All</button>
    <button type="button" onclick={saveToFolder}>Save to Folder</button>
  </div>

  {#if savedPaths.length}
    <div class="saved">
      Saved:
      <ul>
        {#each savedPaths as p}<li><code>{p}</code></li>{/each}
      </ul>
    </div>
  {/if}

  <div class="tabs">
    <button type="button" class:active={tab === "markdown"} onclick={() => (tab = "markdown")}>Markdown</button>
    <button type="button" class:active={tab === "json"} onclick={() => (tab = "json")}>JSON</button>
  </div>

  <pre class="preview">{tab === "markdown" ? mdPreview : jsonPreview}</pre>
{/if}

<style>
  .actions { display: flex; gap: 0.5rem; margin: 1rem 0; }
  .actions button { padding: 0.5rem 1rem; }
  .tabs { margin-bottom: 0.5rem; }
  .tabs button { padding: 0.4rem 0.8rem; }
  .preview {
    padding: 1rem;
    font-size: 0.8rem;
    overflow: auto;
    max-height: 60vh;
    white-space: pre-wrap;
    word-break: break-word;
  }
  .saved { margin: 1rem 0; font-size: 0.9rem; border-radius: var(--radius-sm); padding: 0.75rem; }
  .saved ul { margin: 0.25rem 0 0; padding-left: 1.2rem; }
  code { font-size: 0.8rem; }
</style>
