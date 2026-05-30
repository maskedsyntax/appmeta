<script lang="ts">
  import { base } from "$app/paths";
  import Steps from "$lib/components/Steps.svelte";
  import FeatureSection from "$lib/components/FeatureSection.svelte";
  import CTABanner from "$lib/components/CTABanner.svelte";
  import TerminalDemo from "$lib/components/TerminalDemo.svelte";

  const steps = [
    {
      number: "01",
      cmd: "appmeta connect ./project",
      title: "Connect",
      description: "Point AppMeta at your local mobile project folder.",
    },
    {
      number: "02",
      cmd: "appmeta scan",
      title: "Scan",
      description: "Parse manifests, plists, dependencies, permissions, and source structure.",
    },
    {
      number: "03",
      cmd: "appmeta facts review",
      title: "Confirm",
      description: "Build a truth file from verified facts and your answers.",
    },
    {
      number: "04",
      cmd: "appmeta generate",
      title: "Generate",
      description: "Draft App Store Connect fields via your own API key.",
    },
    {
      number: "05",
      cmd: "appmeta export",
      title: "Export",
      description: "Write submission-pack.md and submission-pack.json.",
    },
  ];

  const differentiators = [
    {
      tag: "local",
      title: "Local-first scanning",
      description:
        "Reads pubspec.yaml, Info.plist, and source paths on disk. No cloud upload of your codebase.",
    },
    {
      tag: "truth.json",
      title: "Truth file first",
      description:
        "Generation runs against a confirmed project profile — not against whatever the model feels like inferring.",
    },
    {
      tag: "validate",
      title: "Field validation",
      description:
        "Character counts, limit warnings, and consistency checks before you paste into App Store Connect.",
    },
    {
      tag: "export",
      title: "Structured export",
      description:
        "Markdown and JSON submission packs — product page, review notes, privacy helpers, IAP copy.",
    },
  ];
</script>

<svelte:head>
  <title>AppMeta — App Store Connect Submission Assistant</title>
</svelte:head>

<section class="landing-hero">
  <div class="hero-grid">
    <div class="hero-copy">
      <p class="label">macOS · BYOK · Tauri + Rust</p>
      <h1>
        App Store metadata from <em class="highlight">verified project facts</em>
      </h1>
      <p class="subtitle muted">
        AppMeta scans your repo locally, helps you confirm what is actually true, then uses your
        OpenAI or OpenRouter key to draft App Store Connect fields. No guessing. No upload.
      </p>
      <div class="actions">
        <a href="{base}/download/" class="btn btn-primary">download</a>
        <a href="{base}/features/" class="btn btn-secondary">read the spec</a>
      </div>
      <p class="pipeline mono muted">
        scan → confirm → generate → export
      </p>
    </div>
    <TerminalDemo />
  </div>
</section>

<section class="section">
  <div class="section-inner">
    <div class="section-header">
      <span class="label">workflow</span>
      <h2>How it works</h2>
      <p>
        A pipeline that builds a verified project profile before any model writes a single
        character of metadata.
      </p>
    </div>
    <Steps {steps} />
  </div>
</section>

<section class="section">
  <div class="section-inner">
    <div class="section-header">
      <span class="label">design</span>
      <h2>Built for people who read plists</h2>
      <p>
        Helps prepare App Store Connect metadata from confirmed project details — not generic
        marketing copy.
      </p>
    </div>
    <div class="grid">
      {#each differentiators as item}
        <FeatureSection tag={item.tag} title={item.title} description={item.description} />
      {/each}
    </div>
  </div>
</section>

<section class="section">
  <div class="section-inner">
    <div class="section-header">
      <span class="label">privacy</span>
      <h2>Your key, your machine</h2>
      <p>BYOK. Local scan. Confirmed facts only in the prompt.</p>
    </div>
    <div class="trust card">
      <pre class="trust-pre"><code># what leaves your machine
- confirmed facts → your AI provider (via your API key)
- nothing else, unless you export or save a pack yourself

# what stays local
- full source tree
- API keys (settings.json; Keychain planned)
- project truth files</code></pre>
    </div>
  </div>
</section>

<CTABanner />

<style>
  .landing-hero {
    padding: 4rem 1.5rem 3.5rem;
    border-bottom: 1px solid var(--color-border-subtle);
  }

  .hero-grid {
    max-width: var(--site-max-width);
    margin: 0 auto;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 3rem;
    align-items: center;
  }

  .hero-copy .label {
    margin-bottom: 1rem;
  }

  .subtitle {
    font-size: 1.0625rem;
    margin: 1.25rem 0 1.75rem;
    max-width: 480px;
    line-height: 1.7;
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.625rem;
    margin-bottom: 1.5rem;
  }

  .pipeline {
    font-size: 0.8125rem;
    margin: 0;
    letter-spacing: 0.02em;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 1rem;
  }

  .trust-pre {
    margin: 0;
    border: none;
    background: none;
    padding: 0;
  }

  .trust-pre code {
    background: none;
    color: var(--color-text-muted);
    font-size: 0.8125rem;
    line-height: 1.8;
  }

  @media (max-width: 860px) {
    .hero-grid {
      grid-template-columns: 1fr;
      gap: 2rem;
    }
  }
</style>
