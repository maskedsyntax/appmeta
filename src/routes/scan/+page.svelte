<script lang="ts">
  import { pickProjectFolder } from "$lib/api/tauri";
  import { scanResult, runScan, project, loading } from "$lib/stores/appStore";

  async function pickAndScan() {
    const path = await pickProjectFolder();
    if (path) await runScan(path);
  }
</script>

<h2>Project Scan</h2>
<p>Scan a local Flutter project folder. Files are read locally — nothing is uploaded.</p>

<button type="button" onclick={pickAndScan} disabled={$loading}>
  {$loading ? "Scanning..." : "Pick Folder & Scan"}
</button>

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
  button { padding: 0.5rem 1rem; margin: 1rem 0; }
  .results { margin-top: 1rem; }
  dl { display: grid; grid-template-columns: 140px 1fr; gap: 0.25rem 1rem; }
  dt { font-weight: 600; color: var(--color-text-muted); }
  dd { margin: 0; }
  h4 { margin: 1.25rem 0 0.5rem; font-size: 0.95rem; color: var(--color-text); }
  .compact { margin: 0; padding-left: 1.2rem; font-size: 0.9rem; }
  .flags li { color: var(--color-warning); }
  pre { font-size: 0.75rem; overflow: auto; max-height: 400px; padding: 0.75rem; }
  .empty { margin-top: 2rem; }
  code { font-size: 0.85rem; word-break: break-all; }
</style>
