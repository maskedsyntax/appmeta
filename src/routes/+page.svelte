<script lang="ts">
  import { pickProjectFolder } from "$lib/api/tauri";
  import { recentProjects, openProject, runScan, project } from "$lib/stores/appStore";

  async function connectFolder() {
    const path = await pickProjectFolder();
    if (path) await runScan(path);
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
          <button type="button" class="link" onclick={() => openProject(p.id)}>
            {p.name}
          </button>
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
  .actions { margin: 1.5rem 0; }
  section { margin-top: 2rem; }
  .path { color: var(--color-text-muted); font-size: 0.85rem; word-break: break-all; }
  ul { list-style: none; padding: 0; }
  li { padding: 0.5rem 0; border-bottom: 1px solid var(--color-border-subtle); }
  small { display: block; color: var(--color-text-dim); font-size: 0.8rem; }
</style>
