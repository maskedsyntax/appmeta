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
    initApp,
    confirmContextPreview,
  } from "$lib/stores/appStore";
  import { getAiContextPreview } from "$lib/api/tauri";
  import WarningItem from "$lib/components/WarningItem.svelte";

  let { children } = $props();
  let previewItems = $state<string[]>([]);

  onMount(() => {
    initApp();
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
        <span class="brand-tagline">Store submission</span>
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
      <p class="muted">No warnings</p>
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
      <p>The following context will be sent to your AI provider. Source code will NOT be sent.</p>
      <ul>
        {#each previewItems as item}<li>{item}</li>{/each}
      </ul>
      <div class="modal-actions">
        <button type="button" onclick={() => showContextPreview.set(false)}>Cancel</button>
        <button type="button" class="primary" onclick={confirmContextPreview}>
          Confirm & Generate {$pendingGenerateField ?? ""}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .app {
    display: grid;
    grid-template-columns: 220px 1fr 260px;
    min-height: 100vh;
    background: var(--color-bg-main);
  }

  .sidebar {
    background: var(--color-bg-sidebar);
    border-right: 1px solid var(--color-border-subtle);
    padding: 1rem 0;
    display: flex;
    flex-direction: column;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.65rem;
    margin: 0 0.85rem 1.25rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--color-border-subtle);
  }

  .brand-logo {
    width: 40px;
    height: 40px;
    border-radius: 10px;
    flex-shrink: 0;
    box-shadow: var(--shadow-glow);
  }

  .brand-text {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .brand-name {
    font-size: 1.05rem;
    font-weight: 700;
    background: var(--gradient-brand);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    line-height: 1.2;
  }

  .brand-tagline {
    font-size: 0.7rem;
    color: var(--color-text-dim);
    letter-spacing: 0.02em;
  }

  nav a {
    display: block;
    padding: 0.5rem 1rem;
    margin: 0 0.5rem;
    color: var(--color-text-muted);
    text-decoration: none;
    border-radius: var(--radius-sm);
    font-size: 0.9rem;
    transition: background 0.12s, color 0.12s;
  }

  nav a:hover {
    background: var(--color-surface-hover);
    color: var(--color-text);
  }

  nav a.active {
    background: var(--gradient-brand-subtle);
    color: var(--color-primary);
    font-weight: 600;
    border: 1px solid rgba(56, 151, 255, 0.2);
  }

  .project-badge {
    margin-top: auto;
    padding: 0.75rem 1rem;
    border-top: 1px solid var(--color-border-subtle);
    font-size: 0.8rem;
  }

  .project-badge strong {
    color: var(--color-text);
    display: block;
    margin-bottom: 0.2rem;
  }

  .project-badge small {
    display: block;
    color: var(--color-text-dim);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 190px;
  }

  .main {
    padding: 1.25rem 1.5rem;
    overflow-y: auto;
    background: var(--color-bg-main);
  }

  .warnings-panel {
    background: var(--color-bg-panel);
    border-left: 1px solid var(--color-border-subtle);
    padding: 1rem;
    overflow-y: auto;
  }

  .warnings-panel h2 {
    font-size: 0.95rem;
    margin: 0 0 0.75rem;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    font-weight: 600;
  }

  .banner {
    padding: 0.5rem 0.75rem;
    margin-bottom: 1rem;
    border-radius: var(--radius-sm);
  }

  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.75);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    padding: 1.5rem;
    border-radius: var(--radius-lg);
    max-width: 480px;
    width: 90%;
    box-shadow: var(--shadow-glow);
  }

  .modal h2 {
    margin-top: 0;
    background: var(--gradient-brand);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .modal ul {
    padding-left: 1.2rem;
    color: var(--color-text-muted);
  }

  .modal-actions {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
    margin-top: 1rem;
  }
</style>
