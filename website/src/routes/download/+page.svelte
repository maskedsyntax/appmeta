<script lang="ts">
  import CTABanner from "$lib/components/CTABanner.svelte";

  const requirements = [
    "macOS (MVP target)",
    "Node.js 18+",
    "Rust stable",
    "Tauri prerequisites (see tauri.app)",
  ];

  const buildSteps = [
    "git clone the repository",
    "npm install (repo root)",
    "npm run tauri dev",
    "npm run tauri build → .app in src-tauri/target/release/bundle/",
  ];
</script>

<svelte:head>
  <title>Download — AppMeta</title>
  <meta
    name="description"
    content="Download AppMeta for macOS. Build from source — pre-built releases coming soon."
  />
</svelte:head>

<section class="page-hero section">
  <div class="section-inner">
    <span class="label">install</span>
    <h1>Download</h1>
    <p class="lead muted">
      Active development. No signed release yet — build from source on your Mac.
    </p>
  </div>
</section>

<section class="section">
  <div class="section-inner">
    <div class="card releases">
      <span class="label">releases</span>
      <h2>GitHub Releases</h2>
      <p class="muted">
        Pre-built .dmg artifacts will land here when v0.1 ships. Until then, compile locally.
      </p>
      <a
        href="https://github.com/maskedsyntax/appmeta/releases"
        class="btn btn-secondary"
        target="_blank"
        rel="noopener noreferrer"
      >
        github.com/.../releases
      </a>
      <p class="notice mono muted">status: building toward first release</p>
    </div>
  </div>
</section>

<section class="section">
  <div class="section-inner two-col">
    <div>
      <span class="label">from source</span>
      <h2>Build instructions</h2>
      <ol class="steps-list mono">
        {#each buildSteps as step}
          <li>{step}</li>
        {/each}
      </ol>
      <a
        href="https://github.com/maskedsyntax/appmeta"
        class="btn btn-primary"
        target="_blank"
        rel="noopener noreferrer"
      >
        open repository
      </a>
    </div>

    <div class="terminal">
      <div class="terminal-bar" aria-hidden="true">
        <span class="terminal-dot"></span>
        <span class="terminal-dot"></span>
        <span class="terminal-dot"></span>
      </div>
      <div class="terminal-body">
        <div><span class="prompt">$</span> git clone https://github.com/maskedsyntax/appmeta.git</div>
        <div><span class="prompt">$</span> cd appmeta && npm install</div>
        <div><span class="prompt">$</span> npm run tauri dev</div>
      </div>
    </div>
  </div>
</section>

<section class="section">
  <div class="section-inner">
    <span class="label">requirements</span>
    <h2>System requirements</h2>
    <ul class="requirements mono">
      {#each requirements as req}
        <li>{req}</li>
      {/each}
    </ul>
    <p class="muted">
      Setup guide:
      <a href="https://v2.tauri.app/start/prerequisites/" target="_blank" rel="noopener noreferrer"
        >tauri.app/start/prerequisites</a
      >
    </p>
  </div>
</section>

<CTABanner
  title="Found a bug?"
  subtitle="File an issue. PRs welcome."
  buttonLabel="github issues"
  href="https://github.com/maskedsyntax/appmeta/issues"
/>

<style>
  .page-hero {
    padding-bottom: 1rem;
    border-top: none;
  }

  .page-hero .label {
    margin-bottom: 0.75rem;
  }

  .lead {
    font-size: 1.0625rem;
    max-width: 480px;
    margin-top: 0.75rem;
  }

  .releases .label {
    margin-bottom: 0.5rem;
  }

  .releases h2 {
    margin-bottom: 0.75rem;
  }

  .releases p {
    margin-bottom: 1rem;
  }

  .notice {
    margin-top: 1rem;
    font-size: 0.75rem;
    padding: 0.625rem 0.75rem;
    background: var(--color-bg-elevated);
    border-radius: var(--radius-sm);
    border: 1px solid var(--color-border-subtle);
  }

  .two-col {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2.5rem;
    align-items: start;
  }

  .two-col .label {
    display: block;
    margin-bottom: 0.5rem;
  }

  .steps-list {
    color: var(--color-text-muted);
    line-height: 1.9;
    margin: 1rem 0 1.5rem;
    font-size: 0.8125rem;
  }

  .requirements {
    color: var(--color-text-muted);
    line-height: 1.9;
    margin: 1rem 0;
    font-size: 0.8125rem;
  }

  .section .label + h2 {
    margin-top: 0;
  }

  @media (max-width: 768px) {
    .two-col {
      grid-template-columns: 1fr;
    }
  }
</style>
