<script lang="ts">
  import { pickProjectFolder } from "$lib/api/tauri";
  import { scanResult, runScan, project, loading, rescanCurrentProject } from "$lib/stores/appStore";

  async function pickAndScan() {
    const path = await pickProjectFolder();
    if (path) await runScan(path);
  }
</script>

<h2>Project Scan</h2>
<p>Scan a local Flutter project folder. Files are read locally — nothing is uploaded.</p>

<div class="actions">
  <button type="button" onclick={pickAndScan} disabled={$loading}>
    {$loading ? "Scanning..." : "Pick Folder & Scan"}
  </button>
  {#if $project?.project.path}
    <button type="button" onclick={rescanCurrentProject} disabled={$loading}>
      Rescan Current Project
    </button>
  {/if}
</div>

{#if $scanResult}
  <section class="results">
    <h3>Scan Results</h3>
    <dl>
      <dt>Framework</dt><dd>{$scanResult.framework ?? "Unknown"}</dd>
      <dt>App Name</dt><dd>{$scanResult.app_name ?? "—"}</dd>
      <dt>Bundle ID</dt><dd>{$scanResult.bundle_id ?? "—"}</dd>
      <dt>Version</dt><dd>{$scanResult.version ?? "—"}</dd>
      <dt>Platforms</dt><dd>{$scanResult.platforms.join(", ") || "—"}</dd>
      <dt>Confidence</dt><dd>{$scanResult.confidence}</dd>
    </dl>

    {#if $scanResult.dependencies.length}
      <h4>Dependencies ({$scanResult.dependencies.length})</h4>
      <ul class="compact">
        {#each $scanResult.dependencies as dep}
          <li>{dep.name} {dep.version ?? ""}</li>
        {/each}
      </ul>
    {/if}

    {#if $scanResult.permissions.length}
      <h4>Permissions</h4>
      <ul class="compact">
        {#each $scanResult.permissions as perm}
          <li>{perm.permission} ({perm.platform})</li>
        {/each}
      </ul>
    {/if}

    {#if $scanResult.detected_features.length}
      <h4>Detected Features</h4>
      <ul class="compact">
        {#each $scanResult.detected_features as f}
          <li>{f.name} <small>({f.source_file})</small></li>
        {/each}
      </ul>
    {/if}

    {#if $scanResult.risk_flags.length}
      <h4>Risk Flags</h4>
      <ul class="compact flags">
        {#each $scanResult.risk_flags as flag}
          <li><strong>{flag.flag}</strong>: {flag.reason}</li>
        {/each}
      </ul>
    {/if}

    {#if $scanResult.detected_urls?.length}
      <h4>Detected URLs</h4>
      <ul class="compact">
        {#each $scanResult.detected_urls as link}
          <li>
            <strong>{link.kind.replace(/_/g, " ")}</strong>:
            <a href={link.url} target="_blank" rel="noopener">{link.url}</a>
            <small>({link.source_file}, {link.confidence})</small>
          </li>
        {/each}
      </ul>
      <p class="hint">Privacy URLs from project files are pre-filled on the Facts page — confirm before submitting.</p>
    {/if}

    {#if $scanResult.questions.length}
      <h4>Confirmation Questions ({$scanResult.questions.length})</h4>
      <p>Answer these on the <a href="/facts">Facts</a> page.</p>
    {/if}

    <details>
      <summary>Raw JSON</summary>
      <pre>{JSON.stringify($scanResult, null, 2)}</pre>
    </details>
  </section>
{:else if $project}
  <section class="results">
    <p>Project loaded from: <code>{$project.project.path}</code></p>
    <p><a href="/facts">Continue to Facts review →</a></p>
  </section>
{:else}
  <p class="empty">No scan yet. Pick a project folder to begin.</p>
{/if}

<style>
  .actions { display: flex; gap: 0.5rem; margin: 0 0 1.25rem; flex-wrap: wrap; }
  .results { margin-top: 0; }
  dl {
    display: grid;
    grid-template-columns: 120px 1fr;
    gap: 0.35rem 1rem;
    margin: 0;
    font-size: 0.875rem;
  }
  dt { font-weight: 500; color: var(--color-text-dim); font-size: 0.8125rem; }
  dd { margin: 0; color: var(--color-text); }
  h4 { margin: 1.25rem 0 0.5rem; }
  .compact { margin: 0; padding-left: 1.15rem; font-size: 0.875rem; line-height: 1.55; }
  .flags li { color: var(--color-warning); }
  pre { font-size: 0.75rem; overflow: auto; max-height: 400px; padding: 0.75rem; }
  .empty { margin-top: 1.5rem; }
  .hint { font-size: 0.8125rem; color: var(--color-text-dim); margin-top: 0.5rem; line-height: 1.45; }
  code { font-size: 0.8125rem; word-break: break-all; }
</style>
