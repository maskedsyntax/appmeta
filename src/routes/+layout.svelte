<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { NAV_ITEMS } from "$lib/types";
  import {
    project,
    warnings,
    error,
    loading,
    showContextPreview,
    pendingGenerateField,
    pendingGenerateQueue,
    initApp,
    confirmContextPreview,
    refreshWarnings,
  } from "$lib/stores/appStore";
  import { getAiContextPreview } from "$lib/api/tauri";
  import WarningItem from "$lib/components/WarningItem.svelte";

  let { children } = $props();
  let previewItems = $state<string[]>([]);

  onMount(() => {
    initApp();
  });

  $effect(() => {
    if ($project) refreshWarnings();
  });

  $effect(() => {
    if ($showContextPreview && $project) {
      getAiContextPreview($project).then((items) => (previewItems = items));
    }
  });

  function isActive(href: string, pathname: string): boolean {
    if (href === "/") return pathname === "/";
    return pathname.startsWith(href);
  }
</script>

<div class="app">
  <aside class="sidebar">
    <div class="brand">
      <img src="/logo.png" alt="AppMeta" class="brand-logo" />
      <div class="brand-text">
        <span class="brand-name">AppMeta</span>
        <span class="brand-tagline">App Store metadata</span>
      </div>
    </div>
    <nav>
      {#each NAV_ITEMS as item}
        <a href={item.href} class:active={isActive(item.href, $page.url.pathname)}>
          {item.label}
        </a>
      {/each}
    </nav>
    {#if $project}
      <div class="project-badge">
        <span class="project-label">Active project</span>
        <strong>{$project.app_identity.app_name || $project.project.name}</strong>
        <small>{$project.project.path}</small>
      </div>
    {/if}
  </aside>

  <main class="main">
    {#if $loading}<div class="banner loading">Working...</div>{/if}
    {#if $error}<div class="banner error">{$error}</div>{/if}
    {@render children()}
  </main>

  <aside class="warnings-panel">
    <h2>Warnings</h2>
    {#if $warnings.length === 0}
      <p class="muted empty-warnings">No issues</p>
    {:else}
      {#each $warnings as w}
        <WarningItem severity={w.severity} field={w.field} message={w.message} />
      {/each}
    {/if}
  </aside>
</div>

{#if $showContextPreview}
  <div class="modal-overlay" role="dialog">
    <div class="modal">
      <h2>AI Context Preview</h2>
      <p class="modal-lead">The following context will be sent to your AI provider. Source code will NOT be sent.</p>
      <ul>
        {#each previewItems as item}<li>{item}</li>{/each}
      </ul>
      <div class="modal-actions">
        <button type="button" onclick={() => showContextPreview.set(false)}>Cancel</button>
        <button type="button" class="primary" onclick={confirmContextPreview}>
          Confirm & Generate{($pendingGenerateQueue.length > 0 ? ` (${1 + $pendingGenerateQueue.length} fields)` : "")}{ $pendingGenerateField ? `: ${$pendingGenerateField}` : ""}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .app {
    display: grid;
    grid-template-columns: 216px 1fr 248px;
    min-height: 100vh;
    background: var(--color-bg-main);
  }

  .sidebar {
    background: var(--color-bg-sidebar);
    border-right: 1px solid var(--color-border-subtle);
    padding: 0.85rem 0;
    display: flex;
    flex-direction: column;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    margin: 0 0.75rem 1rem;
    padding: 0.25rem 0.25rem 1rem;
    border-bottom: 1px solid var(--color-border-subtle);
  }

  .brand-logo {
    width: 36px;
    height: 36px;
    border-radius: 8px;
    flex-shrink: 0;
  }

  .brand-text {
    display: flex;
    flex-direction: column;
    min-width: 0;
    gap: 0.1rem;
  }

  .brand-name {
    font-size: 0.95rem;
    font-weight: 600;
    color: var(--color-text);
    letter-spacing: -0.01em;
    line-height: 1.2;
  }

  .brand-tagline {
    font-size: 0.68rem;
    color: var(--color-text-dim);
    letter-spacing: 0.01em;
  }

  nav {
    display: flex;
    flex-direction: column;
    gap: 1px;
    padding: 0 0.5rem;
  }

  nav a {
    display: block;
    padding: 0.42rem 0.65rem;
    color: var(--color-text-muted);
    text-decoration: none;
    border-radius: var(--radius-sm);
    font-size: 0.875rem;
    border: 1px solid transparent;
    transition:
      background 0.1s ease,
      color 0.1s ease;
  }

  nav a:hover {
    background: var(--color-surface-hover);
    color: var(--color-text);
  }

  nav a.active {
    background: var(--color-primary-muted);
    color: var(--color-text);
    font-weight: 500;
    border-color: rgba(74, 158, 255, 0.2);
  }

  .project-badge {
    margin-top: auto;
    padding: 0.85rem 0.85rem 0.5rem;
    border-top: 1px solid var(--color-border-subtle);
    font-size: 0.78rem;
  }

  .project-label {
    display: block;
    font-size: 0.65rem;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--color-text-dim);
    margin-bottom: 0.35rem;
  }

  .project-badge strong {
    color: var(--color-text);
    display: block;
    margin-bottom: 0.2rem;
    font-size: 0.82rem;
    font-weight: 500;
  }

  .project-badge small {
    display: block;
    color: var(--color-text-dim);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 185px;
    line-height: 1.35;
  }

  .main {
    padding: 1.5rem 1.75rem;
    overflow-y: auto;
    background: var(--color-bg-main);
  }

  .warnings-panel {
    background: var(--color-bg-panel);
    border-left: 1px solid var(--color-border-subtle);
    padding: 1.25rem 1rem;
    overflow-y: auto;
  }

  .warnings-panel h2 {
    font-size: 0.72rem;
    margin: 0 0 0.85rem;
    color: var(--color-text-dim);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    font-weight: 600;
  }

  .empty-warnings {
    font-size: 0.85rem;
    margin: 0;
  }

  .banner {
    padding: 0.55rem 0.75rem;
    margin-bottom: 1rem;
    border-radius: var(--radius-sm);
    font-size: 0.875rem;
  }

  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.65);
    backdrop-filter: blur(6px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    padding: 1.35rem 1.5rem;
    border-radius: var(--radius-lg);
    max-width: 480px;
    width: 90%;
    box-shadow: var(--shadow-md);
  }

  .modal h2 {
    margin-top: 0;
    margin-bottom: 0.5rem;
  }

  .modal-lead {
    color: var(--color-text-muted);
    font-size: 0.875rem;
    margin: 0 0 1rem;
    line-height: 1.5;
  }

  .modal ul {
    margin: 0;
    padding-left: 1.15rem;
    color: var(--color-text-muted);
    font-size: 0.875rem;
    line-height: 1.6;
  }

  .modal-actions {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
    margin-top: 1.25rem;
  }
</style>
