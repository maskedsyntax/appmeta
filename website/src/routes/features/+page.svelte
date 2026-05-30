<script lang="ts">
  import CTABanner from "$lib/components/CTABanner.svelte";

  const features = [
    {
      id: "scan",
      tag: "scanner/",
      title: "Project scan",
      description:
        "Connect a local folder and parse the files App Store reviewers care about — manifests, plists, dependencies, permissions.",
      bullets: [
        "Flutter: pubspec.yaml, ios/Runner/Info.plist, lib/ screens",
        "Dependency risk mapping with confirmation prompts",
        "README / CHANGELOG / spec import",
        "Roadmap: Swift SPM, .xcodeproj, React Native, Expo",
      ],
    },
    {
      id: "facts",
      tag: "truth-file",
      title: "Facts & truth file",
      description:
        "Detected facts get a status. You confirm what is true. The truth file is the only context passed to generation.",
      bullets: [
        "verified · needs_confirmation · rejected per fact",
        "Manual questionnaire: summary, privacy, review flow",
        "Raw JSON view — no black box",
        "Persisted per project in local app data",
      ],
    },
    {
      id: "ai",
      tag: "byok",
      title: "AI generation",
      description:
        "Your API key. Your provider. The model transforms confirmed facts into App Store Connect copy — it does not invent project details.",
      bullets: [
        "OpenAI · OpenRouter",
        "subtitle · promotional_text · description · keywords",
        "review_notes · whats_new",
        "privacy_summary · IAP helpers · regenerate per field",
      ],
    },
    {
      id: "validation",
      tag: "lint",
      title: "Validation",
      description:
        "Warnings for missing fields, limit overruns, and inconsistencies — before you paste into App Store Connect.",
      bullets: [
        "Character limits on subtitle, keywords, descriptions",
        "Analytics deps vs privacy questionnaire mismatch",
        "IAP detected but review notes incomplete",
        "Live warnings panel in the app shell",
      ],
    },
    {
      id: "export",
      tag: "out/",
      title: "Export",
      description: "Structured output you can copy, diff, or commit alongside your release branch.",
      bullets: [
        "submission-pack.md",
        "submission-pack.json",
        "Save to any folder",
        "Copy all fields at once",
      ],
    },
  ];
</script>

<svelte:head>
  <title>Features — AppMeta</title>
  <meta
    name="description"
    content="Local project scan, truth file review, BYOK generation, validation, and Markdown/JSON export for App Store Connect."
  />
</svelte:head>

<section class="page-hero section">
  <div class="section-inner">
    <span class="label">reference</span>
    <h1>Features</h1>
    <p class="lead muted">
      The submission workflow, implemented as a desktop tool — scan through export, with a truth
      file in the middle.
    </p>
  </div>
</section>

{#each features as feature, i}
  <section class="section" id={feature.id}>
    <div class="section-inner feature-row">
      <span class="tag feature-tag">{feature.tag}</span>
      <div class="feature-body">
        <h2>{feature.title}</h2>
        <p class="muted">{feature.description}</p>
        <ul>
          {#each feature.bullets as bullet}
            <li><code>{bullet}</code></li>
          {/each}
        </ul>
      </div>
    </div>
  </section>
{/each}

<CTABanner
  title="Try it on a real project"
  subtitle="Build from source. Pre-signed releases are in progress."
  buttonLabel="download"
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
    max-width: 560px;
    margin-top: 0.75rem;
  }

  .feature-row {
    display: grid;
    grid-template-columns: 6rem 1fr;
    gap: 2rem;
    align-items: start;
  }

  .feature-tag {
    margin-top: 0.35rem;
    word-break: break-all;
  }

  .feature-body h2 {
    margin-bottom: 0.75rem;
  }

  .feature-body > p {
    margin-bottom: 1rem;
    line-height: 1.7;
    max-width: 640px;
  }

  .feature-body ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .feature-body li {
    padding: 0.35rem 0;
    border-bottom: 1px solid var(--color-border-subtle);
    font-size: 0.9375rem;
  }

  .feature-body li:last-child {
    border-bottom: none;
  }

  .feature-body li code {
    background: none;
    padding: 0;
    color: var(--color-text-muted);
    font-size: 0.8125rem;
  }

  @media (max-width: 640px) {
    .feature-row {
      grid-template-columns: 1fr;
      gap: 0.75rem;
    }
  }
</style>
