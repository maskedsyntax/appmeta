<script lang="ts">
  import { pickProjectFolder } from "$lib/api/tauri";
  import {
    recentProjects,
    openProject,
    runScan,
    project,
    deleteProjectById,
  } from "$lib/stores/appStore";

  async function connectFolder() {
    const path = await pickProjectFolder();
    if (path) await runScan(path);
  }

  async function removeProject(id: string, name: string) {
    if (!confirm(`Delete project "${name}"? This cannot be undone.`)) return;
    await deleteProjectById(id);
  }
</script>

<h2>Dashboard</h2>
<p>Connect a Flutter project folder to scan and generate App Store Connect metadata from verified facts.</p>

<div class="actions">
  <button type="button" class="primary" onclick={connectFolder}>Connect Project Folder</button>
</div>

{#if $project}
  <section class="current">
    <h3>Current Project</h3>
    <p><strong>{$project.app_identity.app_name}</strong></p>
    <p class="path">{$project.project.path}</p>
    <p>
      <a href="/scan">View scan</a> ·
      <a href="/facts">Review facts</a> ·
      <a href="/product-page">Generate fields</a>
    </p>
  </section>
{/if}

{#if $recentProjects.length > 0}
  <section>
    <h3>Recent Projects</h3>
    <ul>
      {#each $recentProjects as p}
        <li>
          <div class="row">
            <button type="button" class="link" onclick={() => openProject(p.id)}>
              {p.name}
            </button>
            <button
              type="button"
              class="delete"
              title="Delete project"
              onclick={() => removeProject(p.id, p.name)}
            >Delete</button>
          </div>
          <small>{p.path}</small>
        </li>
      {/each}
    </ul>
  </section>
{:else}
  <section class="empty">
    <p>No saved projects yet. Connect a project folder to get started.</p>
  </section>
{/if}

<style>
  .actions { margin: 0 0 1.5rem; }
  section { margin-top: 1.75rem; }
  .current {
    margin-top: 0;
  }
  .path { color: var(--color-text-muted); font-size: 0.8125rem; word-break: break-all; margin: 0.25rem 0 0.75rem; }
  ul { list-style: none; padding: 0; margin: 0; }
  li {
    padding: 0.65rem 0;
    border-bottom: 1px solid var(--color-border-subtle);
  }
  li:last-child { border-bottom: none; }
  .row { display: flex; align-items: center; gap: 0.75rem; }
  .delete {
    font-size: 0.75rem;
    padding: 0.2rem 0.45rem;
    color: var(--color-error);
    border-color: rgba(239, 107, 107, 0.3);
    background: transparent;
  }
  .delete:hover:not(:disabled) {
    background: var(--color-error-bg);
  }
  small { display: block; color: var(--color-text-dim); font-size: 0.78rem; margin-top: 0.2rem; }
  section.empty {
    padding: 1.25rem;
    background: var(--color-surface);
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-md);
    color: var(--color-text-muted);
  }
</style>
